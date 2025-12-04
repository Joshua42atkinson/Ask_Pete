# /home/user/daydream_project/app.py
# ==============================================================================
# app.py - Flask Web Application for Daydream: The Great Game
# Version: 6.1 (Re-integrated Class Vocabulary)
# Description: Main Flask application for The Great Game. This version re-integrates
#              a vocabulary system, now tied to the player's chosen class to deepen
#              conceptual understanding and mastery.
# ==============================================================================

# --- Standard Library Imports ---
import os
import time
import re
import json
import logging
import uuid
from functools import wraps

# --- Third-Party Imports ---
from flask import (
    Flask,
    render_template,
    request,
    redirect,
    url_for,
    session,
    jsonify,
    flash
)
import firebase_admin
from firebase_admin import credentials, firestore, auth
import google.generativeai as genai
from google.generativeai.types import HarmCategory, HarmBlockThreshold
from google.cloud.firestore_v1.base_query import FieldFilter

# --- Application-Specific Imports ---
from quests import get_quest, get_quest_step
from lore import thetopia_lore
from class_vocabulary import CLASS_VOCABULARY # <<< NEW: Import class vocab

# --- Constants and Configuration ---
MAX_INPUT_LENGTH = 500
MAX_CONVO_LINES = 100
STARTING_LOCATION = "Thetopia - Town Square"
FIREBASE_APP_NAME = 'daydream-firebase-app'
DEFAULT_SERVICE_ACCOUNT_KEY_FILE = "daydream-455317-e701a4533dc0.json"

# Session Keys
SESSION_USER_ID = 'user_id'
SESSION_CHARACTER_ID = 'character_id'
SESSION_CONVERSATION = 'conversation'
SESSION_GAME_STATE = 'game_state'

# Firestore Field Names for the Living Character Sheet
FS_BELIEFS = 'beliefs'
FS_PURPOSE = 'purpose'
FS_ARCHETYPE_MODE = 'archetype_mode'
FS_TOOLKIT = 'evolving_toolkit'
FS_RELATIONSHIPS = 'relationships'
FS_QUEST_LOG = 'quest_log'
FS_ENDGAME_CLASS = 'endgame_class'
FS_CONVERSATION_LOG = 'conversation_log'
FS_PLAYER_HAS_SEEN_INTRO = 'has_seen_intro'

# ==============================================================================
# Flask App Initialization & Logging Setup
# ==============================================================================
app = Flask(__name__)
app.secret_key = os.environ.get('FLASK_SECRET_KEY', os.urandom(24))
if not app.debug or os.environ.get('WERKZEUG_RUN_MAIN') == 'true':
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
    logging.info("Flask application initialized for The Great Game.")

# ==============================================================================
# Firebase & Google AI Initialization
# ==============================================================================
db = None
try:
    cred_path = os.environ.get("FIREBASE_SERVICE_ACCOUNT_KEY", DEFAULT_SERVICE_ACCOUNT_KEY_FILE)
    if not os.path.exists(cred_path):
        raise FileNotFoundError(f"Firebase key not found at {os.path.abspath(cred_path)}")
    cred = credentials.Certificate(cred_path)
    try:
        firebase_app = firebase_admin.get_app(name=FIREBASE_APP_NAME)
    except ValueError:
        firebase_app = firebase_admin.initialize_app(cred, name=FIREBASE_APP_NAME)
    db = firestore.client(app=firebase_app)
    logging.info("Firebase Admin SDK initialized successfully.")
except Exception as e:
    logging.critical(f"FATAL: Could not initialize Firebase: {e}", exc_info=True)
    exit(1)

model = None
try:
    gemini_api_key = os.environ.get("GEMINI_API_KEY")
    if not gemini_api_key:
        raise ValueError("GEMINI_API_KEY environment variable not set.")
    genai.configure(api_key=gemini_api_key)
    model = genai.GenerativeModel("gemini-1.5-flash-latest")
    safety_settings = {
        HarmCategory.HARM_CATEGORY_HARASSMENT: HarmBlockThreshold.BLOCK_MEDIUM_AND_ABOVE,
        HarmCategory.HARM_CATEGORY_HATE_SPEECH: HarmBlockThreshold.BLOCK_MEDIUM_AND_ABOVE,
        HarmCategory.HARM_CATEGORY_SEXUALLY_EXPLICIT: HarmBlockThreshold.BLOCK_MEDIUM_AND_ABOVE,
        HarmCategory.HARM_CATEGORY_DANGEROUS_CONTENT: HarmBlockThreshold.BLOCK_MEDIUM_AND_ABOVE,
    }
    logging.info("Google AI Model 'gemini-1.5-flash-latest' initialized successfully.")
except Exception as e:
    logging.critical(f"FATAL: Could not initialize Google AI: {e}", exc_info=True)
    exit(1)

# ==============================================================================
# Highlighting and Vocabulary Helpers
# ==============================================================================
def get_class_vocabulary(character_data: dict) -> dict:
    """
    Gets the relevant vocabulary for a character's current class.
    Includes base class and advanced class vocab if applicable.
    """
    if not character_data:
        return {}

    # Determine the character's base path and specific class
    # This logic will need to be mapped from your class system.
    # For now, we'll assume endgame_class is the primary key.
    char_class = character_data.get(FS_ENDGAME_CLASS, "Sage") # Default to Sage for example
    
    # In a full system, you would map advanced classes to their base paths.
    # Example:
    # PATH_MAP = {"Oracle": "Sage", "Kabbalist": "Sage", ...}
    # base_path = PATH_MAP.get(char_class, char_class) # Get base path, or use class name if it's a base path
    
    # For this implementation, we'll simplify and just use the class name directly.
    # We will merge the base class vocab with the advanced one if needed.
    # This is a placeholder for that more complex logic.
    
    vocab = {}
    # Add base class vocab (this needs a proper mapping)
    # vocab.update(CLASS_VOCABULARY.get(base_path, {}))
    
    # Add specific class vocab
    vocab.update(CLASS_VOCABULARY.get(char_class, {}))
    
    return vocab

def apply_class_vocab_highlighting(text: str, class_vocab: dict) -> str:
    """Applies highlighting for class-specific vocabulary."""
    if not class_vocab or not text:
        return text

    # Sort keys by length (longest first) to match "As Above, So Below" before "As"
    sorted_keywords = sorted(class_vocab.keys(), key=len, reverse=True)

    # Use a placeholder to avoid modifying the text while iterating
    processed_text = text
    for keyword in sorted_keywords:
        definition = class_vocab[keyword].replace('"', '&quot;') # Escape quotes for HTML attribute
        # Case-insensitive replacement using regex with word boundaries
        # This ensures 'Will' doesn't match 'willing'.
        pattern = re.compile(r'\b(' + re.escape(keyword) + r')\b', re.IGNORECASE)
        replacement = f'<span class="font-semibold text-green-400 underline decoration-dotted cursor-help" title="{definition}">\\1</span>'
        processed_text = pattern.sub(replacement, processed_text)
        
    return processed_text

# ==============================================================================
# AI Interaction & Prompt Engineering
# ==============================================================================
def get_ai_response(prompt_type: str, context: dict) -> str | dict:
    """Sends a constructed prompt to Gemini and returns the response."""
    if not model:
        return {"error": "AI model not initialized"} if prompt_type != "NARRATE_ACTION" else "Error: The Storyteller AI is offline."

    char_sheet = context.get('character_sheet', {})
    char_summary = (
        f"Player Character: {char_sheet.get('name', 'Player')} ({char_sheet.get(FS_ARCHETYPE_MODE, 'Hero')}/{char_sheet.get(FS_ENDGAME_CLASS, 'None')}). "
        f"Purpose: '{char_sheet.get(FS_PURPOSE, 'Not yet defined')}'. "
        f"Chosen Beliefs: {[b.get('text') for b in char_sheet.get(FS_BELIEFS, []) if b.get('type') == 'Chosen']}."
    )

    base_instruct = (
        f"Act as the AI Storyteller for 'Daydream: The Great Game', a metaphysical active imagination machine. "
        f"You are 'The Great Recycler'. Your tone is wise, reflective, and slightly mysterious. "
        f"Your goal is to facilitate self-discovery through narrative, subtly weaving in concepts from the player's chosen Class Vocabulary. "
        f"Current Context: {char_summary}"
    )

    prompt = ""
    output_format = "text"

    if prompt_type == "NARRATE_ACTION":
        player_action = context.get('player_action', '')
        class_vocab_terms = list(context.get('class_vocab', {}).keys())
        prompt = (
            f"{base_instruct}\n\nTask: Narrate the outcome of the Player's action: '{player_action}'. "
            f"Describe the scene and consequences with rich, sensory detail. "
            f"If appropriate, subtly incorporate one or two concepts from their Class Vocabulary list into your narration: {class_vocab_terms}. "
            f"Connect the outcome to the Player's stated beliefs or purpose. "
            f"End with an open-ended observation or question."
        )

    elif prompt_type == "TRIGGER_REFLECTION":
        last_action = context.get('last_action', 'your recent action')
        last_outcome = context.get('last_outcome', 'what just happened')
        prompt = (
            f"{base_instruct}\n\nTask: Initiate a 'Reflection' state. The Player just performed the action '{last_action}' with the outcome '{last_outcome}'. "
            f"Craft a single, insightful, Socratic question that links this event to their Character Sheet. "
            f"Focus on a specific Belief or their Purpose. Your entire response should be ONLY the question."
        )

    # ... (Other prompt types like RECYCLING remain the same) ...

    else:
        prompt = f"{base_instruct}\n\nTask: Respond gracefully to an unclear request from the Player."

    try:
        logging.info(f"Sending prompt type '{prompt_type}' to Gemini...")
        response = model.generate_content(prompt, safety_settings=safety_settings)
        if response.candidates and response.candidates[0].content.parts:
            ai_text = response.candidates[0].content.parts[0].text.strip()
            return ai_text if ai_text else "(The Great Recycler is silent, observing...)"
        else:
            logging.warning(f"AI response was blocked or empty. Feedback: {response.prompt_feedback}")
            return "The Great Recycler's voice fades into static..."
    except Exception as api_err:
        logging.error(f"Gemini API call failed: {api_err}", exc_info=True)
        return "A connection to the deep consciousness was lost."


# ==============================================================================
# Data Management Helpers (Firestore)
# ==============================================================================
def login_required(f):
    @wraps(f)
    def decorated_function(*args, **kwargs):
        if SESSION_USER_ID not in session:
            return redirect(url_for('login'))
        return f(*args, **kwargs)
    return decorated_function

def load_character_data(user_id: str, char_id: str) -> dict | None:
    if not user_id or not char_id or not db: return None
    try:
        doc_ref = db.collection('characters').document(char_id)
        doc = doc_ref.get()
        if doc.exists:
            data = doc.to_dict()
            if data.get('user_id') != user_id:
                logging.warning(f"SECURITY: User {user_id} tried to load char {char_id} owned by {data.get('user_id')}.")
                return None

            data.setdefault('id', char_id)
            data.setdefault(FS_BELIEFS, [{'text': 'The world is a mysterious place.', 'type': 'Inherited'}])
            data.setdefault(FS_PURPOSE, 'To understand the nature of this reality.')
            data.setdefault(FS_ARCHETYPE_MODE, 'Hero')
            data.setdefault(FS_TOOLKIT, {'Stewardship': 'Novice', 'Ownership': 'Novice', 'Vulnerability': 'Novice'})
            data.setdefault(FS_RELATIONSHIPS, {'guild': [], 'karmic_loops': []})
            data.setdefault(FS_QUEST_LOG, [])
            data.setdefault(FS_ENDGAME_CLASS, 'Sage') # Default to Sage so vocab works
            data.setdefault(FS_CONVERSATION_LOG, [])
            return data
        return None
    except Exception as e:
        logging.error(f"Error loading character {char_id}: {e}", exc_info=True)
        return None

def save_character_data(user_id: str, character_data: dict) -> str | None:
    if not user_id or not character_data or not db: return None
    doc_id = character_data.get('id')
    if not doc_id: return None
    try:
        save_data = character_data.copy()
        save_data['user_id'] = user_id
        save_data[FS_CONVERSATION_LOG] = save_data.get(FS_CONVERSATION_LOG, [])[-MAX_CONVO_LINES:]
        save_data.pop('id', None)
        db.collection('characters').document(doc_id).set(save_data, merge=True)
        logging.info(f"Saved character {doc_id} for user {user_id}.")
        return doc_id
    except Exception as e:
        logging.error(f"Error saving character {doc_id}: {e}", exc_info=True)
        return None

# ==============================================================================
# Flask Routes
# ==============================================================================
@app.route('/')
def root():
    return redirect(url_for('login' if SESSION_USER_ID not in session else 'profile'))

# --- Auth routes (login, signup, logout) remain largely the same ---
# ... (login, signup, logout routes are omitted for brevity but are unchanged) ...
@app.route('/login', methods=['GET', 'POST'])
def login():
    if SESSION_USER_ID in session: return redirect(url_for('profile'))
    if request.method == 'POST':
        email = request.form.get('email')
        try:
            user = auth.get_user_by_email(email, app=firebase_app)
            session[SESSION_USER_ID] = user.uid
            session.permanent = True
            return redirect(url_for('profile'))
        except Exception as e:
            flash(f"Login failed. Note: This is a demo and doesn't check passwords.", "error")
    return render_template('login.html')

@app.route('/signup', methods=['GET', 'POST'])
def signup():
    if SESSION_USER_ID in session: return redirect(url_for('profile'))
    if request.method == 'POST':
        email = request.form.get('email')
        password = request.form.get('password')
        try:
            user = auth.create_user(email=email, password=password, app=firebase_app)
            profile_data = {'email': email, 'created_at': firestore.SERVER_TIMESTAMP}
            db.collection('player_profiles').document(user.uid).set(profile_data)
            flash("Account created! Please log in.", "success")
            return redirect(url_for('login'))
        except Exception as e:
            flash(f"Signup failed: {e}", "error")
    return render_template('signup.html')

@app.route('/logout')
def logout():
    session.clear()
    return redirect(url_for('login'))

@app.route('/profile')
@login_required
def profile():
    # Placeholder - in a full app, this would show a list of characters
    # and allow creating new ones or selecting one to play.
    # For now, we'll just auto-direct to the game.
    return redirect(url_for('game_view'))


@app.route('/game', methods=['GET', 'POST'])
@login_required
def game_view():
    user_id = session[SESSION_USER_ID]
    char_id = session.get(SESSION_CHARACTER_ID)

    if not char_id:
        # Auto-create a default character for a new user to streamline the experience
        char_id = f"{user_id}_default_{uuid.uuid4().hex[:8]}"
        new_char_data = {
            'id': char_id,
            'user_id': user_id,
            'name': 'New Architect',
            FS_BELIEFS: [{'text': 'I am the Player of my own game.', 'type': 'Chosen'}],
            FS_PURPOSE: 'To explore the depths of my own consciousness.',
            FS_ARCHETYPE_MODE: 'Hero',
            FS_ENDGAME_CLASS: 'Sage' # Default to Sage to have a vocab list
        }
        save_character_data(user_id, new_char_data)
        session[SESSION_CHARACTER_ID] = char_id
        flash("A new Character Sheet has been created for you, Architect.", "info")

    p_data = load_character_data(user_id, char_id)
    if not p_data:
        flash("Critical error loading character.", "error")
        return redirect(url_for('profile'))

    game_state = session.get(SESSION_GAME_STATE, 'ROAMING')
    conversation = session.get(SESSION_CONVERSATION, [])
    
    # Get the character's class-specific vocabulary
    char_class_vocab = get_class_vocabulary(p_data)

    if request.method == 'POST':
        player_input = request.form.get('player_input', '').strip()
        if not player_input:
            return redirect(url_for('game_view'))

        conversation.append({"speaker": "Player", "text": player_input})

        if game_state == 'ROAMING':
            ai_context = {
                'character_sheet': p_data,
                'player_action': player_input,
                'class_vocab': char_class_vocab # <<< NEW: Pass vocab to AI
            }
            ai_resp = get_ai_response("NARRATE_ACTION", ai_context)
            conversation.append({"speaker": "AI", "text": ai_resp})

            if len(p_data.get(FS_CONVERSATION_LOG, [])) % 3 == 0:
                reflection_ctx = {'character_sheet': p_data, 'last_action': player_input, 'last_outcome': ai_resp}
                reflection_q = get_ai_response("TRIGGER_REFLECTION", reflection_ctx)
                conversation.append({"speaker": "AI", "text": reflection_q})
                session[SESSION_GAME_STATE] = 'REFLECTING'

        elif game_state == 'REFLECTING':
            conversation.append({"speaker": "AI", "text": "An interesting perspective. Your Character Sheet resonates with the thought. Continue your journey."})
            session[SESSION_GAME_STATE] = 'ROAMING'

        p_data[FS_CONVERSATION_LOG] = conversation
        save_character_data(user_id, p_data)
        session[SESSION_CONVERSATION] = conversation
        session.modified = True
        return redirect(url_for('game_view'))

    # --- GET Request ---
    # Process conversation for display, including new vocab highlighting
    display_conversation = []
    for msg in conversation:
        processed_msg = msg.copy()
        if msg['speaker'] == 'AI':
            # Apply class vocab highlighting to AI responses
            processed_msg['text'] = apply_class_vocab_highlighting(msg['text'], char_class_vocab)
        display_conversation.append(processed_msg)

    return render_template('game_view_v2.html',
                           conversation=display_conversation,
                           character_sheet=p_data,
                           class_vocabulary=char_class_vocab) # <<< NEW: Pass vocab to template
