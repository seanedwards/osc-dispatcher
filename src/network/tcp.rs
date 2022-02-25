use crate::config::ConfigHandle;
use std::net::SocketAddr;
use tokio::sync::broadcast::Sender;
use tokio::{io::BufStream, net::TcpListener, net::TcpSocket};

use bytes::BytesMut;
use tracing::span;

pub async fn listen(config: &ConfigHandle, socket_addr: &SocketAddr, tx: &Sender<BytesMut>) -> () {
    let socket = TcpListener::bind(socket_addr)
        .await
        .expect("Could not bind TCP socket.");

    loop {
        let _span = span!(tracing::Level::INFO, "tcp_bind");

        let (stream, _socket_addr) = socket.accept().await.unwrap();
        let stream = BufStream::new(stream);

        let config = config.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            crate::network::agent::spawn(&config, &stream, &tx, &mut rx).await;
        });
    }
}

pub async fn send(config: &ConfigHandle, socket_addr: &SocketAddr, tx: &Sender<BytesMut>) -> () {
    loop {
        let _span = span!(tracing::Level::INFO, "tcp_connect");

        // reconnect loop
        // TODO: Backoff
        let socket = TcpSocket::new_v4().unwrap();
        let stream = socket.connect(socket_addr.clone()).await.unwrap();
        let stream = BufStream::new(stream);

        let config = config.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            crate::network::agent::spawn(&config, &stream, &tx, &mut rx).await;
        });
    }
}
