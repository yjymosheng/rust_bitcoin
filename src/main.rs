use bitcode::block::Transactions;
use bitcode::chain::BlockChain;

fn main() {
    let difficulty = 4;
    let mut chain = BlockChain::new(difficulty);

    let mut transactions = Transactions::new();

    for x in 0..5 {
        let a = rand::random::<u32>();
        let b = rand::random::<u32>();
        transactions.add_transaction(
            a.to_string(),
            b.to_string(),
            rand::random_range(((x as f64) * 100.0)..((x as f64) * 200.0)+100.0),
        );
        //这里的nonce 先写死为0  ,因为还没有实现mine, 实现mine以后,有transaction 就只添加到Transactions ,等mine到之后才进行add_blocks
        chain.mine(&transactions);
    }



    for x in chain.get_blocks() {
        println!("{:#?}", x);
    }


}
