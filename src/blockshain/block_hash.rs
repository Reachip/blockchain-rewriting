use crate::blockshain::block::Block;


pub trait BlockHash {
    fn as_string(&self) -> String;
    fn get<T: ToString>(&self) -> Box<dyn ToString>;
}

#[derive(Default)]
pub struct SHA256BlockHash {
    hash: String
}


impl SHA256BlockHash  {
    pub(crate) fn new(block: Block) -> Self {
        let hash = sha256::digest(block.data + &*block.signature + &*block.proof_of_work);

        Self {
            hash
        }
    }

    pub(crate) fn from(data: String, signature: String, proof_of_work: String) -> Self {
        let hash = sha256::digest(data + &*signature + &*proof_of_work);

        Self {
            hash
        }
    }
}

impl BlockHash for SHA256BlockHash {
    fn as_string(&self) ->  String {
        (&self.get::<String>()).to_string()
    }

    fn get<T: ToString>(&self) -> Box<dyn ToString> {
        Box::new(sha256::digest(&self.hash))
    }
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::blockshain::block::Block;
    use crate::blockshain::block_hash::{BlockHash, SHA256BlockHash};

    #[test]
    fn test_hash_genesis_block() {
        let block_hash = SHA256BlockHash::new(Block::genesis());
    }

    #[test]
    fn test_hash_normal_block() {
        let previous_block = Block::genesis();
        let block_hash = SHA256BlockHash::new(Block::new(Uuid::default(), "data".to_string(), previous_block));
    }
}
