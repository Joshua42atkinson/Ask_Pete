use common::economy::Steam;

#[derive(Clone, Debug)]
pub struct AntigravityClient;

impl AntigravityClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn sync_steam(
        &self,
        _user_id: &str,
        _amount: Steam,
        _source: &str,
    ) -> anyhow::Result<()> {
        // Stub implementation
        Ok(())
    }
}
