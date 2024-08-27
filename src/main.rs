use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis block!".to_owned(), difficulty);

    block.mine();

    println!("Mined genesis block {:?}", &block);

    let mut last_hash = block.hash.clone(); 

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(
            i, 
            now(),
            last_hash,
            0,
            "Another block".to_owned(), 
            difficulty
        );

        block.mine();
        println!("Mined block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);

        println!("Blockchain is valid: {}", blockchain.verify());
    }
}
