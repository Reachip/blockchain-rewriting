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

#[cfg(test)]
mod tests {
    use crate::blockshain::block::Block;
    use crate::blockshain::blockchain::BlockChain;

    #[test]
    fn test_add_node() {
        let mut blockchain = BlockChain::new();

        blockchain.add_block(Block::new());
        blockchain.add_block(Block::new());

        assert_eq!(blockchain.blocks.len(), 2);
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = BlockChain::new();
        assert_eq!(blockchain.nodes.len(), 0);
    }
}
