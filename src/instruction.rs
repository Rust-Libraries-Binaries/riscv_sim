// Instruction Module: Defines a simple set of RISC-V-like instructions

#[derive(Debug)]
pub enum Instruction {
    Add(usize, usize, usize), // ADD rd, rs1, rs2
    Sub(usize, usize, usize), // SUB rd, rs1, rs2
    Load(usize, usize),       // LOAD rd, addr
    Store(usize, usize),      // STORE rs, addr
    Jump(isize),              // JUMP offset
    Halt,                     // HALT
}
