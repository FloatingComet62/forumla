use forumla::{self, client::RequestClient, cache::Cache};
use tracing::error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(cache) = Cache::new() else {
        error!("Cache initialization failed");
        return;
    };
    let client = RequestClient::new(Some(cache));
    if let Err(e) = client.health_check().await {
        error!("Health Check failed: {:?}", e);
        return;
    }

    let _ = client.list_all_f1_drivers().await;
}
