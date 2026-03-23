use forumla::{cache::Cache, client::RequestClient, params::{DriverParams}};
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

    let mut italian_drivers = DriverParams::default();
    italian_drivers.country_code = Some("ITA".to_string());

    let drivers = client.list_all_drivers(italian_drivers).await;
    println!("{:?}", drivers);
}
