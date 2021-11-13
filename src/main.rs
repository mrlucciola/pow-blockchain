use std::vec;

use blockchainlib::{Block, Blockchain, Transaction, now, transaction};

fn main() {
    let difficulty = 0xefffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 50,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 7,
                    },
                ],
            }
        ],
        difficulty,
    );
    
    genesis_block.mine();
    println!("\n mined genesis block: \n{:?}", &genesis_block);
    
    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    // block 2
    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Chris".to_owned(),
                        value: 536,
                    },
                ],
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone(),
                ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 360,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ],
        difficulty,
    );
    
    block.mine();
    println!("\n mined genesis block: \n{:?}", &block);
    
    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add genesis block");
}
