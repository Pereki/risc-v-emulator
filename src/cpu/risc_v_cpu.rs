use crate::cpu::decode::decoder::decode_operation;

const REGISTER_SIZE: usize = 32;
const MEMORY_SIZE: usize = 4096;

pub struct RiscVCpu {
    register: [u32; REGISTER_SIZE],
    program_counter: u8,
    memory: [u8; MEMORY_SIZE],
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
        self.program_counter += 4;

        ((fourth as u32) << 24)
            | (((third as u32) << 16) as u32 | ((second as u32) << 8 | first as u32))
    }

    pub fn decode(&mut self, operation: u32) {
        decode_operation(operation);
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
