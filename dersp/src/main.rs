use dersp::{Config, service::{DerpService, Service}};
use clap::Parser;
use log::info;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let config = Config::parse();
    info!("Config: {config:?}");

    let listener = TcpListener::bind(&config.listen_on).await?;
    let service: Arc<RwLock<DerpService>> = DerpService::new(config).await?;

    info!("Listening on: {:?}", listener.local_addr());

    service.run(listener).await
}
