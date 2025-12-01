use anyhow::Result;
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tokio::sync::mpsc::Sender;

pub struct DownloadProgress {
    pub percent: f32,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}

pub async fn download_file(
    url: String,
    path: PathBuf,
    progress_sender: Sender<DownloadProgress>,
) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let total_size = response.content_length().unwrap_or(0);

    let mut stream = response.bytes_stream();
    let mut file = File::create(&path)?;
    let mut downloaded: u64 = 0;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;

        downloaded += chunk.len() as u64;

        let percent = if total_size > 0 {
            (downloaded as f32 / total_size as f32) * 100.0
        } else {
            0.0
        };

        // Ignore send errors (receiver might have dropped)
        let _ = progress_sender
            .send(DownloadProgress {
                percent,
                downloaded_bytes: downloaded,
                total_bytes: total_size,
            })
            .await;
    }

    Ok(())
}
