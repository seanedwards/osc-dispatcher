use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio_stream::StreamExt;
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;

use bytes::BytesMut;
use tracing::{debug, info, span, trace, warn};

use crate::config::ConfigHandle;

pub async fn listen(_config: &ConfigHandle, socket_addr: &SocketAddr, tx: &Sender<BytesMut>) -> () {
    let _span = span!(tracing::Level::INFO, "udp_listen");
    // This is a listening UDP socket that receives packets, usually from the local system or LAN.
    let socket = UdpSocket::bind(socket_addr)
        .await
        .expect("Couid not bind socket.");
    let mut stream = UdpFramed::new(socket, BytesCodec::new());

    while let Some(value) = stream.next().await {
        let _span = span!(tracing::Level::DEBUG, "udp_recv");

        let (buf, addr) = value.unwrap();
        trace!(proto="udp", side="recv", buf = ?buf, addr = ?socket_addr);
        let size = tx.send(buf).unwrap();
    }
}

pub async fn send(_config: &ConfigHandle, socket_addr: &SocketAddr, tx: &Sender<BytesMut>) -> () {
    let _span = span!(tracing::Level::INFO, "udp_send");
    // This is a listening UDP socket that receives packets, usually from the local system or LAN.
    let socket = UdpSocket::bind(socket_addr)
        .await
        .expect("Couid not bind socket.");
    let mut rx = tx.subscribe();

    loop {
        let _span = span!(tracing::Level::DEBUG, "udp_send");

        // packet go brrrr
        let buf = rx.recv().await.unwrap();
        trace!(proto="udp", side="send", buf = ?buf, addr = ?socket_addr);
        let size = socket.send_to(buf.as_ref(), socket_addr).await.unwrap();
    }
}
