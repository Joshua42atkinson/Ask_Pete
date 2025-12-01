pub mod components;
pub mod multiplayer_client;
pub mod systems;

use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(multiplayer_client::MultiplayerPlugin);
        // Register systems and components here
        // app.add_systems(Update, systems::...);
    }
}
