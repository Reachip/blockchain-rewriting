use std::io;
use std::os::unix::net::UnixStream;
use crate::network::message::BlockChainMessage;
use crate::network::physical_node::PhysicalNode;

pub struct UnixSocketTransmitter;

pub(crate) struct PhysicalNodeTransmitter {
    transmitter: UnixStream,
}

impl PhysicalNodeTransmitter {
    pub(crate) fn new(transmitter: UnixStream) -> Self {
        Self { transmitter }
    }

    fn send(to: &PhysicalNode, message: BlockChainMessage) -> io::Result<()> {
        Ok(()) // TODO
    }
}

impl UnixSocketTransmitter {
    pub(crate) fn new(socket_path: String) -> std::io::Result<UnixStream> {
        Ok(UnixStream::connect(socket_path)?)
    }
}

