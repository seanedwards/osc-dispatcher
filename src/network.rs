use std::fmt::Error;

use crate::config;
use bytes::BytesMut;
use tokio::{
    net::TcpListener,
    sync::broadcast::{Receiver, Sender},
};

pub struct Agent {
    config: std::sync::Arc<config::Config>,
    // Outgoing limits = controlled by the counterparty, adhered to by us
    outgoing_limits: config::LimitConfig,
    sender: Sender<bytes::BytesMut>,
    receiver: Receiver<bytes::BytesMut>,
}

impl Agent {
    pub async fn spawn(
        config: std::sync::Arc<config::Config>,
        socket: &TcpListener,
        sender: Sender<BytesMut>,
        receiver: Receiver<BytesMut>,
    ) {
        let (stream, socket_addr) = socket.accept().await.unwrap();

        tokio::spawn(async move {
            let agent = Self {
                sender: sender,
                receiver: receiver,
                config: config,
                outgoing_limits: config::LimitConfig {
                    rate_bps: 0,
                    timeout_secs: 30,
                    max_subscriptions: 1024,
                    prefix_filter: "".to_string(),
                    ignored_until: chrono::offset::Utc::now(),
                },
            };

            loop {
                // accept
                tokio::spawn(async move {
                    loop {
                        // select
                    }
                });
            }
        });
    }
}

trait Filter {
    fn filter(config: &config::Config);
}
