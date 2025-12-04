pub mod combustion;
pub mod components;
pub mod multiplayer_client;
pub mod systems;

use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(multiplayer_client::MultiplayerPlugin);
        app.add_systems(Update, combustion::combustion_system);
    }
}
