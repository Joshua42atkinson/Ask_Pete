use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGarden {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius_meters: f64,
    pub context_tag: String, // Links to VaaM context
}

impl NodeGarden {
    pub fn new(id: String, name: String, lat: f64, lon: f64, radius: f64, tag: String) -> Self {
        Self {
            id,
            name,
            latitude: lat,
            longitude: lon,
            radius_meters: radius,
            context_tag: tag,
        }
    }

    /// Haversine formula to check if a point is within the garden's radius.
    pub fn is_within_range(&self, player_lat: f64, player_lon: f64) -> bool {
        let earth_radius_km = 6371.0;

        let d_lat = (player_lat - self.latitude).to_radians();
        let d_lon = (player_lon - self.longitude).to_radians();

        let lat1 = self.latitude.to_radians();
        let lat2 = player_lat.to_radians();

        let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        let distance_km = earth_radius_km * c;
        let distance_meters = distance_km * 1000.0;

        distance_meters <= self.radius_meters
    }
}
