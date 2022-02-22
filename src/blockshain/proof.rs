use crate::blockshain::block::Block;
use crate::blockshain::block_hash::BlockHash;
use std::cell::RefCell;
use std::str::FromStr;
use uuid::Uuid;

struct ProofOfWork;

/// TODO
struct ProofOfStack;

trait Proof {
    fn proof(block: &Block) -> String;
    fn finish(hash: &impl BlockHash) -> bool;
}

impl Proof for ProofOfWork {
    fn proof(block: &Block) -> String {
        let proof = Uuid::new_v4().to_string();
        let block = Block::new_with_proof(
            Uuid::from_str(&*block.signature).unwrap(),
            block.data.to_string(),
            block.previous_block_hash.clone(),
            proof,
        );

        block.hash.as_string()
    }

    fn finish(hash: &impl BlockHash) -> bool {
        todo!()
    }
}
