use crate::blockshain::blockchain::BlockChain;
use crate::network::physical_node::PhysicalNode;
use crate::network::receiver::PhysicalNodeReceiver;
use crate::network::transmitter::PhysicalNodeTransmitter;
use std::cell::{RefCell, RefMut};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) id: Uuid,
    pub(crate) blockchain: RefCell<BlockChain>,
}

impl Node {
    pub(crate) fn update_blockchain(&self, new_blockchain: BlockChain) {
        *self.blockchain.borrow_mut() = new_blockchain;
    }

    pub(crate) fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            blockchain: RefCell::new(BlockChain::default()),
        }
    }

    fn mount(node: &Node) -> (PhysicalNodeReceiver, PhysicalNodeTransmitter) {
        let physical = PhysicalNode::new(node);
        (
            PhysicalNodeReceiver::new(physical.listener),
            PhysicalNodeTransmitter::new(physical.transmitter),
        )
    }
}
