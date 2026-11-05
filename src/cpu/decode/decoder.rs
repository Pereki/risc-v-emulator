use std::error::Error;

use crate::cpu::decode::instructions::{self, Instruction};

pub fn decode_operation(operation: u32) -> Result<Instruction, Box<dyn Error>> {
    let opcode = operation & 0x7F;
    let funct3 = (operation & 0x7000) >> 12;
    let funct7 = (operation & 0xFC000000) >> 25;
    let rd = (operation & 0xF80) >> 7;
    let rs1 = (operation & 0xF8000) >> 15;
    let rs2 = (operation & 0x1F00000) >> 20;

    match (opcode, funct3, funct7) {
        (0x33, 0x0, 0x00) => Ok(Instruction::ADD {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        _ => Err(Box::from("No viable OpCode was found.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Brings everything from the outer module into scope

    #[test]
    fn test_add_decode() {
        decode_operation(0x33);
    }
}
