# Train Yard Web Editor - Quick Start Guide

## 🎨 What You're Looking At

The Train Yard is your **visual authoring tool** for creating learning pathways. Think of it like a flowchart builder, but for education.

---

## 🖱️ Creating Your First Learning Pathway

### **Step 1: Add Nodes**

- **Click anywhere on the canvas** to add a dialogue node
- Each node represents a "learning moment" (a lesson, question, or choice)

### **Step 2: Edit Node Content**

Click on a node to edit:

- **Speaker:** Who's talking (e.g., "Pete", "Student", "Professor")
- **Text:** What they say (keep it < 280 chars for cognitive load)
- **Choices:** Branching options (e.g., "Tell me more" → next node)

### **Step 3: Connect Nodes**

- **Drag from a node's bottom handle** to another node's top handle
- This creates the "track" students follow

### **Step 4: Export**

- Click **"Export JSON"** button (top of screen)
- This creates a file your game engine can load

---

## 💡 Example: Simple Physics Lesson

**Node 1 (Pete - Intro)**

```
Speaker: Pete (The Sage)
Text: "Welcome to Force and Motion! Ready to learn?"
Choices: 
  → "Yes, let's go!" (connects to Node 2)
```

**Node 2 (Pete - Lesson)**

```
Speaker: Pete
Text: "Force = Mass × Acceleration. This means heavier objects need more force to move."
Choices:
  → "Show me an example" (connects to Node 3)
  → "I understand" (connects to Node 4)
```

**Node 3 (Pete - Example)**

```
Speaker: Pete  
Text: "Pushing a car (heavy) is harder than pushing a bicycle (light) at the same speed!"
Choices:
  → "That makes sense" (connects to Node 4)
```

**Node 4 (Quiz)**

```
Speaker: Quiz System
Text: "If you double the mass, how much more force is needed?"
Choices:
  → "Twice as much" (correct, connects to Node 5)
  → "Same amount" (incorrect, back to Node 2)
```

---

## 🎯 Tips from the Field Manual

**Cognitive Load (The Weigh Station):**

- Keep text under 280 characters
- One concept per node
- Don't overwhelm with choices (max 3-4)

**Archetype Icons** (coming soon):

- 🗲 Hero - Action/Challenges
- 📖 Sage - Information/Lessons  
- 😊 Jester - Fun/Exploration
- 🛡️ Caregiver - Support/Guidance

**Literary Devices:**

- Hero's Journey - Classic quest structure
- Mystery - Reveal information gradually
- Simulation - Real-world scenarios

---

## 🚀 Try It Now

1. Create 3 nodes
2. Connect them in sequence  
3. Add meaningful text to each
4. Click "Export JSON"
5. Open the file - that's your curriculum!

---

**Your first graph is just a few clicks away! 🎨**
