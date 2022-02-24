pub mod config;
mod network;
use std::net::SocketAddr;
use std::sync::Arc;

use bytes;
use tokio::net::{TcpListener, UdpSocket};
use tracing::{debug, info, span, warn};

#[tokio::main]
#[tracing::instrument]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let config = Arc::new(config::read_config("config.toml"));

    let (tx, mut _rx) = tokio::sync::broadcast::channel(16);

    /******************************************************/
    /*                   Set up binds                     */
    /******************************************************/
    let binds = config.bind.clone();
    match binds {
        None => (),
        Some(binds) => {
            let config = config.clone();
            binds
                .iter()
                .map(move |bind| {
                    let config = config.clone();
                    let tx = tx.clone();

                    let socket_addr: SocketAddr = bind
                        .socket_addrs(|| Some(9000 as u16))
                        .expect("Could not parse SocketAddr")
                        .first()
                        .expect("Could not resolve SocketAddr")
                        .clone();

                    tokio::spawn(async move {
                        match bind.scheme() {
                            "udp" => {
                                // This is a listening UDP socket that receives packets, usually from the local system or LAN.
                                let socket = UdpSocket::bind(socket_addr)
                                    .await
                                    .expect("Couid not bind socket.");

                                loop {
                                    let _span = span!(tracing::Level::WARN, "udp_bind");

                                    // With UDP, we just yeet the packets
                                    let mut buf = bytes::BytesMut::with_capacity(512);
                                    let (_len, _addr) = socket.recv_from(&mut buf).await.unwrap();
                                    tx.send(buf).unwrap();
                                }
                            }
                            "tcp" => {
                                let socket = TcpListener::bind(socket_addr)
                                    .await
                                    .expect("Could not bind TCP socket.");

                                loop {
                                    let _span = span!(tracing::Level::WARN, "tcp_bind");

                                    // With TCP, we spawn stateful agents that coordinate with each other
                                    let tx = tx.clone();
                                    let rx = tx.subscribe();
                                    network::Agent::spawn(config.clone(), &socket, tx, rx).await;
                                }
                            }
                            _ => (), //panic!("Invalid scheme {}", bind.scheme()),
                        };
                    });
                })
                .collect::<()>();
        }
    };
    /******************************************************/
    /*                  Set up connects                   */
    /******************************************************/

    debug!("Test: {:?}", config);

    ctrlc::set_handler(move || {
        true;
        ()
    })
    .expect("Error setting Ctrl-C handler")
}
