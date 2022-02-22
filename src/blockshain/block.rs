use crate::blockshain::block_hash::BlockHash;
use crate::blockshain::block_hash::SHA256BlockHash;
use uuid::Uuid;

pub(crate) struct Block {
    pub(crate) previous_block_hash: SHA256BlockHash,
    pub data: String,
    pub signature: String,
    pub proof_of_work: String,
    pub(crate) hash: SHA256BlockHash,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.hash.as_string() == other.hash.as_string()
    }
}

impl Eq for Block {}

impl Block {
    pub fn new(author: Uuid, data: String, previous_block: Block) -> Self {
        let signature = author.to_string();
        let proof_of_work = Uuid::new_v4().to_string();

        Self {
            previous_block_hash: SHA256BlockHash::new(previous_block),
            data: data.clone(),
            signature: signature.clone(),
            proof_of_work: proof_of_work.clone(),
            hash: SHA256BlockHash::from(data, signature, proof_of_work),
        }
    }

    pub fn new_with_proof(
        signature: String,
        data: String,
        previous_block_hash: SHA256BlockHash,
        proof: String,
    ) -> Self {
        let proof_of_work = proof;

        Self {
            previous_block_hash,
            data: data.clone(),
            signature: signature.clone(),
            proof_of_work: proof_of_work.clone(),
            hash: SHA256BlockHash::from(data, signature, proof_of_work),
        }
    }

    pub fn genesis() -> Self {
        Self {
            previous_block_hash: SHA256BlockHash::default(),
            data: String::default(),
            signature: String::default(),
            proof_of_work: String::default(),
            hash: SHA256BlockHash::default(),
        }
    }
}
