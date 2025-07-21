use indexmap::IndexMap;
use std::error::Error;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{error, info};
use zyst::aof::clean_up_db;
use zyst::config::get_config;
use zyst::database::delete_expired_keys;
use zyst::database::restore_from_aof;
use zyst::server::handle_client;
use zyst::types::Db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let db: Db = Arc::new(RwLock::new(IndexMap::new()));

    // Config
    let config = get_config();
    let port: u16 = config.get("port").expect("Port is missing");
    let bind: Ipv4Addr = config.get("bind").expect("Bind is missing");
    let full_address = format!("{bind}:{port}");

    let listener = TcpListener::bind(full_address.to_string()).await?;
    let message = format!("Listening {full_address}...");

    info!(message);

    // Restoring DB from AOF file at start up
    tokio::spawn(restore_from_aof(db.clone()));

    // Delete expired keys every 60 seconds
    tokio::spawn(delete_expired_keys(db.clone()));

    // Clean database every 60 seconds
    tokio::spawn(clean_up_db(db.clone()));

    loop {
        let (socket, addr) = listener.accept().await?;

        let db = Arc::clone(&db);

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, db).await {
                error!("Error handling client {}: {:?}", addr, e);
            }
        });
    }
}
