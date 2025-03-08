use std::fs::File;
use std::io::{BufReader, Read, Write};
use bincode::{Decode, Encode};
use crate::block::{Block, Transactions};
use crate::{hash_str, tools};
use crate::tools::serialize;
use sha2::Digest;

#[derive(Encode,Decode)]
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

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            // 验证每个区块的哈希是否符合预期
            if current_block.header.pre_hash != previous_block.hash {
                return false;
            }

            // 验证当前区块的哈希值是否正确
            if current_block.hash != current_block.calc_hash() {
                return false;
            }
        }
        true
    }

    // 保存到二进制文件
    pub fn save_to_bin_file(&self, filename: &str) -> std::io::Result<()> {
        let config = bincode::config::standard();
        let serialized = bincode::encode_to_vec(self, config).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    // 从二进制文件加载
    pub fn load_from_bin_file(filename: &str) -> std::io::Result<Self> {
        let config = bincode::config::standard();
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let blockchain: BlockChain = bincode::decode_from_reader(&mut reader, config).unwrap();
        Ok(blockchain)
    }
}
