use crate::block::{Block, Transactions};
use crate::{hash_str, tools};
use crate::tools::serialize;
use sha2::Digest;

pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub difficulty: u64,
}

impl BlockChain {
    pub fn new(difficulty : u64) -> Self {
        let transactions = Transactions::new();
        Self {
            blocks: vec![Block::new(transactions, hash_str!("创世链 hash"), difficulty,0)],
            difficulty,
        }
    }

    pub fn add_block (&mut self, transactions: Transactions,nonce:u64) {
        let pre_hash = self.blocks.get(self.blocks.len()-1).unwrap().hash.clone();
        let block =  Block::new(transactions,  pre_hash, self.difficulty, nonce );
        self.blocks.push(block);
    }
    pub fn get_blocks (&self) -> & Vec<Block> {
        &self.blocks
    }

    pub fn set_difficulty (&mut self, difficulty : u64) {
        self.difficulty = difficulty;
    }
    pub fn mine (&mut self,transactions : &Transactions) {
        let target = "0".repeat(self.difficulty as usize);
        let mut nonce = 0;
        let mut s =self.mine_hash(transactions, &nonce );
        while  !s.starts_with(&target) {
            nonce += 1;  // 每次尝试增加 nonce
            s = self.mine_hash(transactions, &nonce );
        }
        println!("Congratulations");
        self.add_block(transactions.clone(),nonce);

    }

    fn mine_hash (&self,transactions : &Transactions, nonce : & u64) -> String {
        let pre_hash = self.blocks.get(self.blocks.len()-1).unwrap().hash.clone();
        hash_str!(&tools::serialize(transactions) , &pre_hash ,nonce )
    }
}
