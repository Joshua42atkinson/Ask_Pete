use crate::services::weigh_station::WeighStationService;
use bevy::prelude::*;
use domain_physics::components::SharedGraphManagerResource;
use std::sync::Arc;

#[derive(Resource, Clone)]
pub struct SharedWeighStationResource(pub Option<Arc<WeighStationService>>);

#[derive(Resource, Clone)]
pub struct SharedTokioHandle(pub tokio::runtime::Handle);

pub fn weigh_station_system(
    graph_manager: Res<SharedGraphManagerResource>,
    weigh_station: Res<SharedWeighStationResource>,
    tokio_handle: Res<SharedTokioHandle>,
) {
    // 1. Check if we have a weigh station service
    let service = match &weigh_station.0 {
        Some(s) => s.clone(),
        None => return, // No service, no weighing
    };

    // 2. Scan for unweighed nodes
    let mut nodes_to_weigh = Vec::new();

    {
        // Read lock to find nodes
        if let Ok(manager) = graph_manager.0.read() {
            // Debug log to verify graph state
            if !manager.nodes.is_empty() {
                // println!("DEBUG: Graph has {} nodes", manager.nodes.len());
            }

            for (id, node) in &manager.nodes {
                if node.mass.is_none() {
                    println!("DEBUG: Found unweighed node: {}", id);
                    nodes_to_weigh.push((id.clone(), node.content.clone()));
                }
            }
        }
    }

    // 3. Spawn tasks
    if !nodes_to_weigh.is_empty() {
        println!(
            "⚖️ Weigh Station: Found {} nodes to weigh.",
            nodes_to_weigh.len()
        );

        let graph_manager_arc = graph_manager.0.clone();

        for (id, content) in nodes_to_weigh {
            let service_clone = service.clone();
            let graph_manager_clone = graph_manager_arc.clone();

            // Spawn async task using the Tokio Handle from the main thread
            tokio_handle.0.spawn(async move {
                // Call AI
                match service_clone.weigh_node(&content).await {
                    Ok(physics) => {
                        println!(
                            "✅ Weigh Station: Node '{}' weighed. Score: {}",
                            id, physics.complexity_score
                        );

                        // Update Graph
                        // Note: This locks for writing.
                        if let Ok(mut manager) = graph_manager_clone.write() {
                            if let Some(node) = manager.nodes.get_mut(&id) {
                                node.mass = Some(physics.complexity_score as f32);
                                // We could also store concept_count or reasoning if we added fields for them
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Weigh Station: Failed to weigh node '{}': {}", id, e);
                    }
                }
            });
        }
    }
}
