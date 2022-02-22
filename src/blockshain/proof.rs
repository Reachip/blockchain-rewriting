use crate::blockshain::block::Block;
use crate::blockshain::block_hash::BlockHash;
use std::cell::RefCell;
use std::str::FromStr;
use uuid::Uuid;

pub(crate) struct ProofOfWork {
    complexity: u32,
}

impl ProofOfWork {
    pub(crate) fn new(complexity: u32) -> Self {
        Self { complexity }
    }

    pub(crate) fn complexity_factor(&self) -> String {
        (0..self.complexity).into_iter().map(|_| "0").collect()
    }
}

/// TODO
struct ProofOfStack;

pub(crate) trait Proof {
    fn proof(block: &Block) -> String;
    fn finish(&self, proof: String) -> bool;
}

impl Proof for ProofOfWork {
    fn proof(block: &Block) -> String {
        let proof = Uuid::new_v4().to_string();
        let block = Block::new_with_proof(
            block.signature.clone(),
            block.data.to_string(),
            block.previous_block_hash.clone(),
            proof,
        );

        block.hash.as_string()
    }

    fn finish(&self, proof: String) -> bool {
        proof.starts_with(&self.complexity_factor())
    }
}

#[cfg(test)]
mod tests {
    use crate::blockshain::block::Block;
    use crate::blockshain::block_hash::SHA256BlockHash;
    use crate::blockshain::proof::{Proof, ProofOfWork};

    #[test]
    fn test_proof_for_proof_of_work() {
        let block = Block::genesis();
    }

    #[test]
    fn test_proof_for_proof_of_work_complexity_factor() {
        let proof = ProofOfWork::new(2);
        assert_eq!("00", proof.complexity_factor())
    }

    #[test]
    fn test_finish_for_proof_of_work() {
        let block_hash = SHA256BlockHash::new(Block::genesis());
    }
}
