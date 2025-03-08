use bincode::Encode;
use sha2::Digest;
use crate::{hash_str, tools::{serialize}};

#[derive(bincode::Encode,Debug)]
pub struct BlockHeader {
    // 版本号
    pub version: String,
    // 时间戳
    pub timestamp: i64,
    // 交易的hash, merkle hash值
    pub tx_hash: String,
    // 用于工作量证明
    pub difficulty_target: u64,
    // 前一个的hash
    pub pre_hash: String,
    // nonce 用来进行猜测
    pub nonce : u64,
}

impl BlockHeader {
    pub fn new(tx_hash: String, pre_hash: String,difficulty : u64,nonce:u64) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        Self {
            // 目前定为固定0.1.0
            version: "0.1.0".to_string(),
            timestamp,
            tx_hash,
            //未完成
            difficulty_target:difficulty,
            pre_hash,
            nonce,
        }
    }
}

#[derive(Encode,Debug,Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Transaction {
        let sender = hash_str!(&sender);
        let receiver = hash_str!(&receiver);
        Self {
            sender,
            receiver,
            amount,
        }
    }
}


#[derive(Encode,Debug,Clone)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

impl Transactions {
    pub fn new() -> Transactions {
        Transactions {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, sender: String, receiver: String, amount: f64) {
        self.transactions
            .push(Transaction::new(sender, receiver, amount));
    }

    pub fn calc_hash(&self) -> String {
        if self.transactions.is_empty() {
            return "".to_string();
        }
        let mut hashes: Vec<String> = self
            .transactions
            .iter()
            .map(|tx| hash_str!(&serialize(tx)    )) // 计算每个交易的哈希
            .collect();
        if hashes.len() % 2 == 1 {
            hashes.push(hashes.last().unwrap().clone());
        }

        while hashes.len() > 1 {
            let mut new_level = Vec::new();

            for chunk in hashes.chunks(2) {
                let combined = format!("{}{}", chunk[0], chunk[1]); // 连接两个哈希
                let parent_hash = hash_str!(combined.as_bytes()); // 计算父节点哈希
                new_level.push(parent_hash);
            }

            hashes = new_level;

            // 如果当前层的节点数仍然是奇数，复制最后一个节点
            if hashes.len() % 2 == 1 && hashes.len() > 1 {
                hashes.push(hashes.last().unwrap().clone());
            }
        }

        hashes[0].clone()
    }
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Transactions,
    pub hash: String,
}

impl Block {
    ///
    pub fn new( transactions: Transactions, pre_hash: String,difficulty: u64 ,nonce: u64 ) -> Self    {
        let tx_hash = transactions.calc_hash();
        let header = BlockHeader::new(tx_hash, pre_hash,difficulty,nonce);
        let hash = hash_str!( &header , &transactions);
        Self{
            header,
            transactions,
            hash,
        }
    }
}
