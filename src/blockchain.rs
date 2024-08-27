use super::*;

pub struct Blockchain {
  pub blocks : Vec<Block>,
}

impl Blockchain {
  pub fn verify (&self) -> bool {
    for (i, block) in self.blocks.iter().enumerate() {
      if block.index != i as u32 
      {
        print!("Index mismatch {} != {}", 
          &block.index, &i
        );
        return false;
      } 
      else if !block::check_difficulty(&block.hash, block.difficulty) 
      {
        print!("Hash difficulty fail");
        return false;
      } 
      else if i != 0 
      {
        let prev_block = &self.blocks[i - 1];
        if block.timestamp <= prev_block.timestamp 
        {
          print!("Timestamp not greater than previous block");
          return false;
        } 
        else if block.prev_block_hash != prev_block.hash 
        {
          print!("Hash does not match previous block");
          return false;
        }
      } 
      else 
      {
        if block.prev_block_hash != vec![0; 32] 
        {
          print!("Genesis block prev_block_hash invalid");
          return false;
        }
      }
    }
    true
  }
}