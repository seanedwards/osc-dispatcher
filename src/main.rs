pub mod config;
mod network;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tracing::{debug, info, span, trace, warn};

use config::ConfigHandle;

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

    /******************************************************/
    /*                   Set up binds                     */
    /******************************************************/
    let tx = tx_orig.clone();
    let binds = config.bind.clone();
    match binds {
        None => (),
        Some(binds) => {
            let config = config.clone();
            binds.into_iter().for_each(|bind| {
                let config = config.clone();
                let tx = tx.clone();

                debug!(bind = ?bind);

                let socket_addr = bind
                    .socket_addrs(|| Some(9000 as u16))
                    .expect("Could not parse SocketAddr")
                    .first()
                    .expect("Could not resolve SocketAddr")
                    .clone();

                tokio::spawn(async move {
                    match bind.scheme() {
                        "udp" => network::udp::listen(&config, &socket_addr, &tx).await,
                        "tcp" => network::tcp::listen(&config, &socket_addr, &tx).await,
                        _ => {
                            panic!("Invalid scheme {}", bind.scheme());
                        }
                    }
                });
            })
        }
    };

    /******************************************************/
    /*                  Set up connects                   */
    /******************************************************/
    let tx = tx_orig.clone();
    let connects = config.connect.clone();
    match connects {
        None => (),
        Some(connects) => {
            let config = config.clone();
            let tx = tx.clone();
            //let config = config.clone();
            connects.into_iter().for_each(move |connect| {
                let config = config.clone();
                let tx = tx.clone();

                debug!(connect = ?connect);

                let socket_addr = connect
                    .socket_addrs(|| Some(9000 as u16))
                    .expect("Could not parse SocketAddr")
                    .first()
                    .expect("Could not resolve SocketAddr")
                    .clone();

                tokio::spawn(async move {
                    match connect.scheme() {
                        "udp" => network::udp::send(&config, &socket_addr, &tx).await,
                        "tcp" => network::tcp::send(&config, &socket_addr, &tx).await,
                        _ => (),
                    }
                });
            });
        }
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {}
}
