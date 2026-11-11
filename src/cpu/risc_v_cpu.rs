use std::error::Error;

use crate::cpu::{
    decode::decoder::decode_operation,
    execute::executer::execute,
    execution_step::{InstructionExecutionSteps, ResultSet, Step::WRITEBACK},
    memory::memory_access::handle_memory_access,
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

    pub fn load_program(&mut self, program: &[u8]) {
        for (offset, byte) in program.iter().enumerate() {
            self.memory[offset] = *byte;
        }
    }

    pub fn fetch(&mut self) -> u32 {
        let first = self.memory[self.program_counter as usize];
        let second = self.memory[(self.program_counter + 1) as usize];
        let third = self.memory[(self.program_counter + 2) as usize];
        let fourth = self.memory[(self.program_counter + 3) as usize];

        ((fourth as u32) << 24) | (((third as u32) << 16) | ((second as u32) << 8 | first as u32))
    }

    pub fn next_operation(&mut self) {
        self.program_counter += 4;
    }

    pub fn decode(&mut self, operation: u32) -> Result<InstructionExecutionSteps, Box<dyn Error>> {
        decode_operation(operation)
    }

    pub fn execute(
        &mut self,
        instruction: &InstructionExecutionSteps,
    ) -> Result<Option<ResultSet>, Box<dyn Error>> {
        execute(self, instruction)
    }

    pub fn memory_access(
        &mut self,
        instruction: &InstructionExecutionSteps,
    ) -> Result<Option<ResultSet>, Box<dyn Error>> {
        handle_memory_access(self, instruction)
    }

    pub fn write_back(
        &mut self,
        instruction: &InstructionExecutionSteps,
    ) -> Result<(), Box<dyn Error>> {
        if !instruction.next_steps.contains(&WRITEBACK) {
            return Ok(());
        }

        let write_op = match &instruction.memory_access_result {
            Some(res) => res,
            None => match &instruction.arithmetic_result {
                Some(res) => res,
                None => {
                    return Err(Box::from(
                        "Could not write back, even tough there must be a valid value.",
                    ));
                }
            },
        };

        if write_op.target == 0 {
            return Ok(());
        }
        self.register[write_op.target as usize] = write_op.result;
        Ok(())
    }

    pub fn print_cpu_state(&self) {
        println!("----- CPU STATE -----");
        println!("PC: {}", self.program_counter);
        println!("----- REGISTERS -----");
        for (number, value) in self.register.iter().enumerate() {
            println!("Register [{}]: {:x}", number, value);
        }
        println!("----- MEMORY -----");
        for (number, value) in self.memory.iter().enumerate().filter(|(n, _)| *n <= 100) {
            println!("Memory address [{}]: {:x}", number, value);
        }
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
