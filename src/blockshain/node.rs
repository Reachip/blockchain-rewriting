use crate::network::physical_node::PhysicalNode;
use crate::network::receiver::PhysicalNodeReceiver;
use crate::network::transmitter::PhysicalNodeTransmitter;
use uuid::Uuid;

pub struct Node {
    pub(crate) id: Uuid,
}

impl Node {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    fn mount(node: &Node) -> (PhysicalNodeReceiver, PhysicalNodeTransmitter) {
        let physical = PhysicalNode::new(node);
        (
            PhysicalNodeReceiver::new(physical.listener),
            PhysicalNodeTransmitter::new(physical.transmitter),
        )
    }
}
