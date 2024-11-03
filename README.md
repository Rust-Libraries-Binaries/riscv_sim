# riscv_sim

A simple and efficient RISC-V instruction simulator written in Rust. This library is designed for developers and enthusiasts who want to learn, prototype, or test RISC-V assembly code in a controlled software environment.

## Features
- **Simulate RISC-V Instructions**: Support for basic instructions like `ADD`, `SUB`, `LOAD`, `STORE`, `JUMP`, and `HALT`.
- **Memory and Register Management**: Includes models for memory and a set of 32 general-purpose registers.
- **Easy to Extend**: Add more instructions or features as needed to simulate more complex RISC-V programs.
- **Rust-Powered**: Built with Rust for safety and performance.

## Getting Started
### Prerequisites
- **Rust**: Make sure you have Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org).

### Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/riscv_sim.git
   cd riscv_sim
```
## Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/riscv_sim.git
   cd riscv_sim
```
## Build the project:
```
sh
cargo build
```
## Run the tests to make sure everything is working:
```
sh
cargo test
```
## Usage
Basic Example
Here's how you can use riscv_sim in your Rust project:
```rust
use riscv_sim::Cpu;
use riscv_sim::Memory;
use riscv_sim::Instruction;

fn main() {
    // Create a new CPU and Memory
    let mut cpu = Cpu::new();
    let mut memory = Memory::new(1024); // 1KB of memory

    // Load some values into memory and execute simple instructions
    memory.store(0, 42); // Store 42 at address 0
    memory.store(1, 100); // Store 100 at address 1

    // Example instructions
    let instructions = vec![
        Instruction::Add(1, 0, 0),  // Add registers 0 and 0, store in register 1
        Instruction::Sub(2, 1, 0),  // Subtract register 0 from register 1, store in register 2
        Instruction::Halt,          // Halt the CPU
    ];

    // Load and execute instructions here (to be implemented)
}
```
## Modules Overview
Cpu: Simulates the RISC-V CPU with registers and a program counter. Includes methods for resetting and managing the CPU state.
Memory: A simple memory model to load and store values.
Instruction: Defines the Instruction enum with various RISC-V-like instructions.

## License
This project is licensed under the MIT License 

## Author
Ben Santora (<bensatlantik@gmail.com>)

## Support My Work
If you find this library helpful, consider supporting my open-source efforts: 
https://www.paypal.com/donate/?business=QMSZ2E6L75BYN&no_recurring=0&item_name=If+my+Rust+libraries+have+added+value+to+your+projects%2C+consider+a+small+donation+to+help+me+continue.+Thank+you%21&currency_code=USD
