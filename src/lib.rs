#[cfg(test)]
mod tests {
    use super::*;
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
        let cpu = Cpu::new();
        assert_eq!(cpu.registers[0], 0);
        assert_eq!(cpu.pc, 0);

        for &reg in &cpu.registers {
            assert_eq!(reg, 0);
        }
    }

    #[test]
    fn test_cpu_reset() {
        let mut cpu = Cpu::new();
        cpu.registers[1] = 10;
        cpu.pc = 5;

        cpu.reset();
        assert_eq!(cpu.registers[1], 0);
        assert_eq!(cpu.pc, 0);
    }

    #[test]
    fn test_memory_operations() {
        let mut memory = Memory::new(10);
        memory.store(0, 42);
        assert_eq!(memory.load(0), 42);

        memory.store(5, 99);
        assert_eq!(memory.load(5), 99);
    }

    #[test]
    fn test_instruction_enum() {
        let add_inst = Instruction::Add(1, 2, 3);
        let sub_inst = Instruction::Sub(4, 5, 6);

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
