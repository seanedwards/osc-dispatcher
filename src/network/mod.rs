pub mod tcp;
pub mod udp;
pub mod agent;

use std::fmt::Error;

use crate::config;
use bytes::BytesMut;
use tokio::io::{AsyncRead, AsyncWrite, BufStream};
use tokio::{
    io::Sink,
    net::TcpStream,
    sync::broadcast::{Receiver, Sender},
};
use tracing::{debug, info, span, trace, warn};


trait Filter {
    fn filter(config: &config::Config);
}
