use std::io::{self, Write};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use bitcode::block::Transactions;
use bitcode::chain::BlockChain;
use rand::random;

fn main() {
    let mut difficulty = 4;  // 默认难度
    let mut chain = BlockChain::new(difficulty);

    loop {
        println!("请选择操作:");
        println!("1. 检查区块链一致性");
        println!("2. 进行挖矿");
        println!("3. 手动添加交易");
        println!("4. 查看区块链");
        println!("5. 修改难度");
        println!("6. 保存区块链");
        println!("7. 加载区块链");
        println!("8. 退出");

        let mut choice = String::new();
        print!("请输入选项: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                // 检查一致性
                if chain.is_valid() {
                    println!("区块链是有效的");
                } else {
                    println!("区块链无效");
                }
            }
            2 => {
                // 进行挖矿
                let mut transactions = Transactions::new();
                for _ in 0..5 {
                    let a = random::<u32>();
                    let b = random::<u32>();
                    transactions.add_transaction(
                        a.to_string(),
                        b.to_string(),
                        random::<f64>(),
                    );
                }
                chain.mine(&transactions);
                println!("挖矿成功，区块已加入");
            }
            3 => {
                // 手动添加交易
                let mut transactions = Transactions::new();
                let mut input = String::new();
                print!("请输入交易数据 (格式：发送方 接收方 金额): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.len() == 3 {
                    let sender = parts[0].to_string();
                    let receiver = parts[1].to_string();
                    let amount: f64 = parts[2].parse().unwrap_or_else(|_| 0.0);
                    transactions.add_transaction(sender, receiver, amount);
                    println!("交易已添加");
                } else {
                    println!("无效的交易格式，请重新输入");
                }
            }
            4 => {
                // 查看区块链
                for block in chain.get_blocks() {
                    println!("{:#?}", block);
                }
            }
            5 => {
                // 修改难度
                let mut new_difficulty = String::new();
                print!("请输入新的难度: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_difficulty).unwrap();
                let new_difficulty: u64 = new_difficulty.trim().parse().unwrap_or_else(|_| difficulty);
                chain.set_difficulty(new_difficulty);
                println!("难度已修改为 {}", new_difficulty);
            }
            6 => {
                // 保存区块链
                match chain.save_to_bin_file("blockchain.bin") {
                    Ok(_) => println!("区块链已保存"),
                    Err(e) => println!("保存区块链失败: {}", e),
                }
            }
            7 => {
                // 加载区块链
                match BlockChain::load_from_bin_file("blockchain.bin") {
                    Ok(loaded_chain) => {
                        chain = loaded_chain;
                        println!("区块链已加载");
                    }
                    Err(e) => println!("加载区块链失败: {}", e),
                }
            }
            8 => break, // 退出
            _ => println!("无效的选项，请重新输入"),
        }
    }
}
