use std::os::unix::net::{UnixListener, UnixStream};
use crate::blockshain::node::Node;
use crate::network::receiver::UnixSocketListener;
use crate::network::transmitter::UnixSocketTransmitter;

pub struct PhysicalNode<'a> {
    node: &'a Node,
    socket_path: String,
    pub(crate) listener: UnixListener,
    pub(crate) transmitter: UnixStream,
}

impl<'a> PhysicalNode<'a> {
    pub(crate) fn new(node: &'a Node) -> Self {
        let socket_path = "/Users/rached/Desktop/".to_string() + &*node.id.to_string() + ".sock";

        Self {
            node,
            socket_path: socket_path.clone(),
            listener: UnixSocketListener::new(socket_path.clone()).unwrap(),
            transmitter: UnixSocketTransmitter::new(socket_path).unwrap(),
        }
    }
}