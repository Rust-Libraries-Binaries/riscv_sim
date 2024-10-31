// riscv_sim Library

// Declare the modules
pub mod cpu;
pub mod memory;
pub mod instruction;

// A simple "hello" function to ensure the library works correctly
pub fn hello() {
    println!("Hello from riscv_sim!");
}

// Unit tests to validate the "hello" function and other modules
#[cfg(test)]
mod tests {
    use super::*; // Correctly imports items from the root of the crate
    use crate::cpu::Cpu;
    use crate::memory::Memory;
    use crate::instruction::Instruction;

    #[test]
    fn test_hello() {
        // Basic test to ensure the hello function does not panic
        hello();
    }

    #[test]
    fn test_cpu_initialization() {
        // Test if the CPU is initialized correctly with all registers set to 0
        let cpu = Cpu::new();
        assert_eq!(cpu.registers[0], 0);
        assert_eq!(cpu.pc, 0);

        // Check that all other registers are zero-initialized
        for &reg in &cpu.registers {
            assert_eq!(reg, 0);
        }
    }

    #[test]
    fn test_cpu_reset() {
        // Test the reset function of the CPU
        let mut cpu = Cpu::new();
        cpu.registers[1] = 10; // Modify a register
        cpu.pc = 5;            // Modify the program counter

        cpu.reset();
        assert_eq!(cpu.registers[1], 0); // Check if register is reset
        assert_eq!(cpu.pc, 0);           // Check if PC is reset
    }

    #[test]
    fn test_memory_operations() {
        // Test memory load and store operations
        let mut memory = Memory::new(10); // Create a small memory block of size 10
        memory.store(0, 42);              // Store value 42 at address 0
        assert_eq!(memory.load(0), 42);   // Check if the value is loaded correctly

        memory.store(5, 99);              // Store value 99 at address 5
        assert_eq!(memory.load(5), 99);   // Check if the value is loaded correctly
    }

    #[test]
    fn test_instruction_enum() {
        // Test if the Instruction enum can be created and used
        let add_inst = Instruction::Add(1, 2, 3); // ADD rd, rs1, rs2
        let sub_inst = Instruction::Sub(4, 5, 6); // SUB rd, rs1, rs2

        // Simple checks to ensure the enum variants are created correctly
        if let Instruction::Add(rd, rs1, rs2) = add_inst {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        } else {
            panic!("Expected Instruction::Add variant");
        }

        if let Instruction::Sub(rd, rs1, rs2) = sub_inst {
            assert_eq!(rd, 4);
            assert_eq!(rs1, 5);
            assert_eq!(rs2, 6);
        } else {
            panic!("Expected Instruction::Sub variant");
        }
    }
}
