// CPU Module: Simulates a simple RISC-V CPU structure

pub struct Cpu {
    pub registers: [i32; 32], // 32 general-purpose registers
    pub pc: usize,            // Program counter
}

impl Cpu {
    // Creates a new CPU with registers initialized to zero and pc set to 0
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
        }
    }

    // A simple method to reset the CPU
    pub fn reset(&mut self) {
        self.registers = [0; 32];
        self.pc = 0;
    }
}
