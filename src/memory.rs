// Memory Module: Simulates a simple memory structure

pub struct Memory {
    pub data: Vec<i32>, // Memory data, using a vector for simplicity
}

impl Memory {
    // Creates a new Memory instance with a specified size
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    // Loads a value from a given memory address
    pub fn load(&self, address: usize) -> i32 {
        self.data[address]
    }

    // Stores a value at a given memory address
    pub fn store(&mut self, address: usize, value: i32) {
        self.data[address] = value;
    }
}
