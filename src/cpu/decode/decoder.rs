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
        (0x33, 0x0, 0x20) => Ok(Instruction::SUB {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        (0x33, 0x4, 0x00) => Ok(Instruction::XOR {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        (0x33, 0x6, 0x00) => Ok(Instruction::OR {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        (0x33, 0x7, 0x00) => Ok(Instruction::AND {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        (0x33, 0x1, 0x00) => Ok(Instruction::SLL {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        (0x33, 0x5, 0x00) => Ok(Instruction::SRL {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        (0x33, 0x5, 0x20) => Ok(Instruction::SRA {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        (0x33, 0x2, 0x00) => Ok(Instruction::SLT {
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
        }),
        (0x33, 0x3, 0x00) => Ok(Instruction::SLTU {
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
        assert_eq!(
            Instruction::ADD {
                funct3: 0,
                funct7: 0,
                fmt: instructions::FmtType::R,
                rd: 0,
                rs1: 0,
                rs2: 0,
            },
            decode_operation(0x33).unwrap()
        );
    }

    #[test]
    fn test_sub_decode() {
        assert_eq!(
            Instruction::SUB {
                funct3: 0,
                funct7: 32,
                fmt: instructions::FmtType::R,
                rd: 13,
                rs1: 10,
                rs2: 11,
            },
            decode_operation(0x40B506B3).unwrap()
        );
    }

    #[test]
    fn test_xor_decode() {
        assert_eq!(
            Instruction::XOR {
                funct3: 0x4,
                funct7: 0,
                fmt: instructions::FmtType::R,
                rd: 13,
                rs1: 10,
                rs2: 11,
            },
            decode_operation(0x0B546B3).unwrap()
        );
    }

    #[test]
    fn test_or_decode() {
        assert_eq!(
            Instruction::OR {
                funct3: 0x6,
                funct7: 0,
                fmt: instructions::FmtType::R,
                rd: 13,
                rs1: 10,
                rs2: 11,
            },
            decode_operation(0x0B566B3).unwrap()
        );
    }

    #[test]
    fn test_and_decode() {
        assert_eq!(
            Instruction::AND {
                funct3: 0x7,
                funct7: 0,
                fmt: instructions::FmtType::R,
                rd: 13,
                rs1: 10,
                rs2: 11,
            },
            decode_operation(0x0B576B3).unwrap()
        );
    }

    #[test]
    fn test_sll_decode() {
        assert_eq!(
            Instruction::SLL {
                funct3: 0x1,
                funct7: 0,
                fmt: instructions::FmtType::R,
                rd: 13,
                rs1: 10,
                rs2: 11,
            },
            decode_operation(0x0B516B3).unwrap()
        );
    }

    #[test]
    fn test_srl_decode() {
        assert_eq!(
            Instruction::SRL {
                funct3: 0x5,
                funct7: 0,
                fmt: instructions::FmtType::R,
                rd: 13,
                rs1: 10,
                rs2: 11,
            },
            decode_operation(0x0B556B3).unwrap()
        );
    }

    #[test]
    fn test_sra_decode() {
        assert_eq!(
            Instruction::SRA {
                funct3: 0x5,
                funct7: 0x20,
                fmt: instructions::FmtType::R,
                rd: 0,
                rs1: 0,
                rs2: 0,
            },
            decode_operation(0x40005033).unwrap()
        );
    }

    #[test]
    fn test_slt_decode() {
        assert_eq!(
            Instruction::SLT {
                funct3: 0x2,
                funct7: 0x0,
                fmt: instructions::FmtType::R,
                rd: 0,
                rs1: 0,
                rs2: 0,
            },
            decode_operation(0x00002033).unwrap()
        );
    }

    #[test]
    fn test_sltu_decode() {
        assert_eq!(
            Instruction::SLTU {
                funct3: 0x3,
                funct7: 0x0,
                fmt: instructions::FmtType::R,
                rd: 0,
                rs1: 0,
                rs2: 0,
            },
            decode_operation(0x00003033).unwrap()
        );
    }
}
