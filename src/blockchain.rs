use std::{collections::HashSet};

use crate::{Block, Hash, Hashable, block::{check_difficulty}};

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronTimeStamp,
    MismatchedPrevHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTxn,
}
pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs:HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();
        // fail case
        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            // not genesis block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronTimeStamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPrevHash);
            }
        } else {
            // genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTxn);
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;
            for txn in transactions {
                let input_hashes = txn.input_hashes();
                if 
                    !(&input_hashes - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes & &block_spent).is_empty() {
                        return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = txn.input_value();
                let output_value = txn.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(txn.output_hashes());
            }

            // more validation on the coinbase txn
            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTxn);
            } else {
                // update block-created with the hashes created in the coinbase txn
                block_created.extend(coinbase.output_hashes());
            }

            // update the unspent output pool
            // get the ones we have spent first
            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);

        }

        // add block to the blockchain
        self.blocks.push(block);

        Ok(())
    }
}
