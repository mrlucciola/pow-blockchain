use std::vec;

use blockchainlib::{Block, now, Hashable};

fn main () {
    let mut block = Block::new(13, now(), vec![0, 32], 0, "Genesis block".to_owned());
    println!("{:?}", &block);

    let h = block.hash();
    println!("\n hash: {:?}", &h);
    
    block.hash = h;
    println!("\n block: {:?}", &block);
}
