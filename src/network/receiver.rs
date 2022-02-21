use crate::network::message::BlockChainMessage;
use crate::network::physical_node::PhysicalNode;
use std::io;
use std::os::unix::net::UnixListener;

pub struct PhysicalNodeReceiver {
    receiver: UnixListener,
}

impl PhysicalNodeReceiver {
    pub(crate) fn new(receiver: UnixListener) -> Self {
        Self { receiver }
    }
    fn send(to: PhysicalNode, message: BlockChainMessage) -> io::Result<()> {
        Ok(())
    }
}

pub struct UnixSocketListener;

impl UnixSocketListener {
    pub(crate) fn new(socket_path: String) -> std::io::Result<UnixListener> {
        Ok(UnixListener::bind(socket_path)?)
    }
}
