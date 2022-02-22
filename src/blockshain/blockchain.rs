use crate::blockshain::block::Block;
use crate::blockshain::node::Node;

pub struct BlockChain {
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

        blockchain.add_block(Block::genesis());
        blockchain.add_block(Block::genesis());

        assert_eq!(blockchain.blocks.len(), 2);
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = BlockChain::new();
        assert_eq!(blockchain.nodes.len(), 0);
    }
}
