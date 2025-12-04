use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PartyStyle {
    #[default]
    IronHorse, // Speed/Efficiency
    ScenicRoute, // Exploration/Lore
    NightTrain,  // Hardcore/Challenge
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MemberRole {
    #[default]
    Engineer, // DPS / Warrior
    Conductor, // Tank / Leader
    Stoker,    // Healer / Support
    Signalman, // CC / Mage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub name: String,
    pub role: MemberRole,
    pub archetype: String, // e.g., "The Hero", "The Sage"
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteParams {
    pub question: String,        // "Which track do we take?"
    pub options: Vec<String>,    // ["Library", "Lab"]
    pub deadline: u64,           // Unix Timestamp
    pub current_tally: Vec<u32>, // [3, 2]
}

// 1. THE SHARED REALITY
// This struct mirrors the data in your Google Database.
// When it changes in the cloud, it changes on everyone's screen.
#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    pub campaign_id: String,
    pub style: PartyStyle, // [NEW]
    pub current_station_id: String,
    pub party_members: HashMap<String, CharacterProfile>, // [MODIFIED] UserID -> Profile

    // The Group's "Health" and "Fuel"
    pub collective_coal: u32,  // Sum of all contributions
    pub collective_steam: u32, // Group mastery score

    // Asynchronous Voting State
    pub active_vote: Option<VoteParams>,
}

// 2. EVENTS (How the Game Reacts)
#[derive(Event, Clone)]
pub struct StateUpdateEvent(pub CampaignState);

#[derive(Event)]
pub struct ChatMessageEvent {
    pub sender: String,
    pub text: String,
}

// 3. THE SYNC SYSTEM (The "Heartbeat")
// This system runs every frame but checks for updates efficiently.
// In this Headless Bevy setup, we sync from the Shared Resource (injected by Axum)
pub fn sync_campaign_state(
    _commands: Commands,
    mut current_state: ResMut<CampaignState>,
    shared_state: Res<crate::components::SharedCampaignStateResource>,
    mut event_writer: EventWriter<StateUpdateEvent>,
) {
    if let Ok(guard) = shared_state.0.read() {
        // Simple check: if campaign_id changes or coal/steam changes, update
        // In a real app, we'd have a more robust "dirty" flag or version number
        let new_data = &*guard;

        let changed = current_state.campaign_id != new_data.campaign_id
            || current_state.style != new_data.style
            || current_state.collective_coal != new_data.collective_coal
            || current_state.collective_steam != new_data.collective_steam
            || current_state.active_vote.is_some() != new_data.active_vote.is_some(); // Simplified check

        if changed {
            *current_state = new_data.clone();
            event_writer.send(StateUpdateEvent(new_data.clone()));
            info!(
                "Campaign State Synced: ID={}, Coal={}",
                current_state.campaign_id, current_state.collective_coal
            );
        }
    }
}

// 4. THE VOTING SYSTEM (Async Interaction)
// In Headless Bevy, we don't use KeyCode. We use Events triggered by the API.
// See `sync_vote_inbox` in systems.rs
pub fn process_vote_events(
    mut events: EventReader<crate::components::VoteEvent>,
    mut state: ResMut<CampaignState>,
) {
    for event in events.read() {
        info!(
            "🗳️ Processing Vote for Option {} in Campaign: {}",
            event.option_index, state.campaign_id
        );
        // In a real implementation, this would update the local state optimistically
        // AND the backend would have already updated the DB.
        // For now, we just log it.
        if let Some(vote) = &mut state.active_vote {
            if event.option_index < vote.current_tally.len() {
                vote.current_tally[event.option_index] += 1;
            }
        }
    }
}

// 5. THE PLUGIN (Boilerplate to add to your App)
pub struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CampaignState>()
            .add_event::<StateUpdateEvent>()
            .add_event::<StateUpdateEvent>()
            .add_event::<ChatMessageEvent>()
            .add_event::<crate::components::VoteEvent>() // [FIX]
            .add_systems(Update, (sync_campaign_state, process_vote_events));
    }
}
