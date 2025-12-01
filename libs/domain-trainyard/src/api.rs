use gloo_net::http::Request;
use pete_core::expert::StoryGraph;

pub async fn get_graph() -> Result<StoryGraph, String> {
    let res = Request::get("/api/expert/graph")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.ok() {
        let graph: StoryGraph = res.json().await.map_err(|e| e.to_string())?;
        Ok(graph)
    } else {
        Err(format!("Failed to fetch graph: {}", res.status()))
    }
}

pub async fn save_graph(graph: StoryGraph) -> Result<StoryGraph, String> {
    let res = Request::post("/api/expert/graph")
        .json(&graph)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.ok() {
        let saved_graph: StoryGraph = res.json().await.map_err(|e| e.to_string())?;
        Ok(saved_graph)
    } else {
        Err(format!("Failed to save graph: {}", res.status()))
    }
}
