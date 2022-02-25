pub mod config;
mod network;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tracing::{debug, info, warn};

use config::{ConfigHandle, SocketConfig, SocketMode};

#[tokio::main]
#[tracing::instrument]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let config: ConfigHandle = Arc::new(config::read_config("config.toml"));

    let (tx_orig, mut _rx) = tokio::sync::broadcast::channel(16);

    info!(
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        major = env!("CARGO_PKG_VERSION_MAJOR"),
        minor = env!("CARGO_PKG_VERSION_MINOR"),
        patch = env!("CARGO_PKG_VERSION_PATCH"),
        pre = env!("CARGO_PKG_VERSION_PRE")
    );

    let tx = tx_orig.clone();
    config.socket.clone().into_iter().for_each(|socket| {
        let config = config.clone();
        let tx = tx.clone();

        let socket_addr = socket
            .addr
            .socket_addrs(|| Some(9000 as u16))
            .expect("Could not parse SocketAddr")
            .first()
            .expect("Could not resolve SocketAddr")
            .clone();
        debug!(socket = ?socket, socket_addr = ?socket_addr);

        tokio::spawn(async move {
            match socket {
                // Receivers
                SocketConfig {
                    mode: SocketMode::Bind,
                    addr,
                } if addr.scheme() == "udp" => {
                    network::udp::listen(&config, &socket_addr, &tx).await
                }
                SocketConfig {
                    mode: SocketMode::Bind,
                    addr,
                } if addr.scheme() == "tcp" => {
                    network::tcp::listen(&config, &socket_addr, &tx).await
                }
                // Senders
                SocketConfig {
                    mode: SocketMode::Connect,
                    addr,
                } if addr.scheme() == "udp" => network::udp::send(&config, &socket_addr, &tx).await,
                SocketConfig {
                    mode: SocketMode::Connect,
                    addr,
                } if addr.scheme() == "tcp" => network::tcp::send(&config, &socket_addr, &tx).await,
                // Oh no!
                _ => {
                    panic!("Invalid socket config: {:?}", socket)
                }
            }
        });
    });

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {}
}
