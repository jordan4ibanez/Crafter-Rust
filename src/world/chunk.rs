
#[derive(Clone)]
pub struct BlockChunk {
    block: Vec<u32>
}

impl BlockChunk {
    pub fn new() -> Self {
        Self {
            block: vec![0; 32768],
        }
    }

    pub fn get(&self, index: usize) -> u32 {
        self.block[index]
    }

    pub fn set(&mut self, index: usize, new_value: u32) {
        self.block[index] = new_value;
    }
}