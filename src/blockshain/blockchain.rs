use crate::blockshain::block::Block;
use crate::blockshain::node::Node;

pub struct BlockChain {
    // node_path: Path
    blocks: Vec<Block>,
    nodes: Vec<Node>,
}

impl BlockChain {
    fn new() -> Self {
        Self {
            blocks: vec![],
            nodes: vec![],
        }
    }

    fn add_block(&mut self, block: Block) {
        &self.blocks.push(block);
    }

    fn add_node(&mut self, node: Node) {
        &self.nodes.push(node);
    }
}