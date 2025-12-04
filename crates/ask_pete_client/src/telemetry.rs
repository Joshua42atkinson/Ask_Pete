use lazy_static::lazy_static;
use std::sync::RwLock;

#[derive(Debug, Clone, Copy, Default)]
pub struct PhysicsState {
    pub coal: f32,
    pub steam: f32,
    pub velocity: f32,
    pub latitude: f64,
    pub longitude: f64,
}

lazy_static! {
    pub static ref PHYSICS_STATE: RwLock<PhysicsState> = RwLock::new(PhysicsState {
        coal: 100.0,
        steam: 0.0,
        velocity: 0.0,
        latitude: 0.0,
        longitude: 0.0,
    });
}
