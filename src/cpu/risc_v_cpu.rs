use std::error::Error;

use crate::cpu::{
    decode::{decoder::decode_operation, instructions::Instruction},
    execute::executer::{ExecutionResult, execute},
};

const REGISTER_SIZE: usize = 32;
const MEMORY_SIZE: usize = 4096;

pub struct RiscVCpu {
    pub register: [u32; REGISTER_SIZE],
    pub program_counter: u8,
    pub memory: [u8; MEMORY_SIZE],
}

impl RiscVCpu {
    pub fn new() -> Self {
        Self {
            register: [0; 32],
            program_counter: 0,
            memory: [0; 4096],
        }
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        let mut offset = 0;
        for byte in program {
            self.memory[offset] = byte.clone();
            offset += 1;
        }
    }

    pub fn fetch(&mut self) -> u32 {
        let first = self.memory[self.program_counter as usize];
        let second = self.memory[(self.program_counter + 1) as usize];
        let third = self.memory[(self.program_counter + 2) as usize];
        let fourth = self.memory[(self.program_counter + 3) as usize];

        ((fourth as u32) << 24)
            | (((third as u32) << 16) as u32 | ((second as u32) << 8 | first as u32))
    }

    pub fn next_operation(&mut self) {
        self.program_counter += 4;
    }

    pub fn decode(&mut self, operation: u32) -> Result<Instruction, Box<dyn Error>> {
        decode_operation(operation)
    }

    pub fn execute(
        &mut self,
        instruction: &Instruction,
    ) -> Result<ExecutionResult, Box<dyn Error>> {
        execute(self, instruction)
    }

    pub fn memory_access(&mut self, instruction: &Instruction) {}

    pub fn write_back(&mut self, register: u8, value: u32) {
        if register == 0 {
            return;
        }
        self.register[register as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Brings everything from the outer module into scope

    #[test]
    fn test_fetch() {
        let program = vec![0x78, 0x56, 0x34, 0x12];
        let mut cpu = RiscVCpu::new();

        cpu.load_program(&program);
        let res = cpu.fetch();
        assert_eq!(res, 0x12345678);
    }
}
