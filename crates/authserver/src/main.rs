use tracing::{info, Level};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Legacy Authentication Server");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Starting authentication server...");
}