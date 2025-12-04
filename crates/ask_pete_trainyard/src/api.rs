use pete_core::trainyard::StoryGraph;
use reqwest::Client;

pub async fn get_graph() -> Result<StoryGraph, String> {
    let client = Client::new();
    let res = client
        .get("/api/expert/graph")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let graph: StoryGraph = res.json().await.map_err(|e| e.to_string())?;
        Ok(graph)
    } else {
        Err(format!("Failed to fetch graph: {}", res.status()))
    }
}

pub async fn save_graph(graph: StoryGraph) -> Result<StoryGraph, String> {
    let client = Client::new();
    let res = client
        .post("/api/expert/graph")
        .json(&graph)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let saved_graph: StoryGraph = res.json().await.map_err(|e| e.to_string())?;
        Ok(saved_graph)
    } else {
        Err(format!("Failed to save graph: {}", res.status()))
    }
}
