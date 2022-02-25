use crate::config;
use bytes::BytesMut;
use tokio::sync::broadcast::{Receiver, Sender};
use tracing::{debug, info, span, trace, warn};

pub async fn spawn<Protocol>(
    config: &config::ConfigHandle,
    socket: &tokio::io::BufStream<Protocol>,
    sender: &Sender<BytesMut>,
    receiver: &mut Receiver<BytesMut>,
) {
    let _span = span!(tracing::Level::INFO, "agent");

    loop {
        // select
        //tokio::select!(r = socket.poll_read() {});
    }
}
