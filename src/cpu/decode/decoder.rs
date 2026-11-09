use std::error::Error;

use crate::cpu::decode::{
    instruction_element_decoder,
    instructions::{self, FmtType, Instruction},
};

pub fn decode_operation(operation: u32) -> Result<Instruction, Box<dyn Error>> {
    let decoded_instruction = instruction_element_decoder::DeconstructedInstruction::new(operation);

    match (
        decoded_instruction.opcode,
        decoded_instruction.funct3,
        decoded_instruction.funct7,
    ) {
        (0x63, 0x7, _) => Ok(Instruction::BGEU {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::B,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x6, _) => Ok(Instruction::BLTU {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::B,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x5, _) => Ok(Instruction::BGE {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::B,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x4, _) => Ok(Instruction::BLT {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::B,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x1, _) => Ok(Instruction::BNE {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::B,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x0, _) => Ok(Instruction::BEQ {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::B,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x23, 0x2, _) => Ok(Instruction::SW {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::S,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.s_type_imm,
        }),
        (0x23, 0x1, _) => Ok(Instruction::SH {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::S,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.s_type_imm,
        }),
        (0x23, 0x0, _) => Ok(Instruction::SB {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::S,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.s_type_imm,
        }),
        (0x3, 0x5, _) => Ok(Instruction::LHU {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x3, 0x4, _) => Ok(Instruction::LBU {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x3, 0x2, _) => Ok(Instruction::LW {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x3, 0x1, _) => Ok(Instruction::LH {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x3, 0x0, _) => Ok(Instruction::LB {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x03, _) => Ok(Instruction::SLTIU {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x02, _) => Ok(Instruction::SLTI {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x05, 0x20) => Ok(Instruction::SRAI {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x05, 0x00) => Ok(Instruction::SRLI {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x01, 0x00) => Ok(Instruction::SLLI {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x07, _) => Ok(Instruction::ANDI {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x06, _) => Ok(Instruction::ORI {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x00, _) => Ok(Instruction::ADDI {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x04, _) => Ok(Instruction::XORI {
            funct3: decoded_instruction.funct3 as u8,
            fmt: FmtType::I,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x33, 0x0, 0x00) => Ok(Instruction::ADD {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x0, 0x20) => Ok(Instruction::SUB {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x4, 0x00) => Ok(Instruction::XOR {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x6, 0x00) => Ok(Instruction::OR {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x7, 0x00) => Ok(Instruction::AND {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x1, 0x00) => Ok(Instruction::SLL {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x5, 0x00) => Ok(Instruction::SRL {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x5, 0x20) => Ok(Instruction::SRA {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x2, 0x00) => Ok(Instruction::SLT {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x3, 0x00) => Ok(Instruction::SLTU {
            funct3: decoded_instruction.funct3 as u8,
            funct7: decoded_instruction.funct7 as u8,
            fmt: instructions::FmtType::R,
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        _ => Err(Box::from("No viable OpCode was found.")),
    }
}

#[cfg(test)]
mod tests {

    use super::*; // Brings everything from the outer module into scope

    #[test]
    fn test_sw_decode() {
        assert_eq!(
            Instruction::SW {
                funct3: 2,
                fmt: instructions::FmtType::S,
                rs1: 0,
                rs2: 0,
                imm: 0,
            },
            decode_operation(0x2023).unwrap()
        );
    }

    #[test]
    fn test_sh_decode() {
        assert_eq!(
            Instruction::SH {
                funct3: 1,
                fmt: instructions::FmtType::S,
                rs1: 0,
                rs2: 0,
                imm: 0,
            },
            decode_operation(0x1023).unwrap()
        );
    }

    #[test]
    fn test_sb_decode() {
        assert_eq!(
            Instruction::SB {
                funct3: 0,
                fmt: instructions::FmtType::S,
                rs1: 0,
                rs2: 0,
                imm: 3220,
            },
            decode_operation(0xC8000A23).unwrap()
        );
    }

    #[test]
    fn test_lhu_decode() {
        assert_eq!(
            Instruction::LHU {
                funct3: 5,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0,
            },
            decode_operation(0x5003).unwrap()
        );
    }

    #[test]
    fn test_lbu_decode() {
        assert_eq!(
            Instruction::LBU {
                funct3: 4,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0,
            },
            decode_operation(0x4003).unwrap()
        );
    }

    #[test]
    fn test_lw_decode() {
        assert_eq!(
            Instruction::LW {
                funct3: 2,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0,
            },
            decode_operation(0x2003).unwrap()
        );
    }

    #[test]
    fn test_lh_decode() {
        assert_eq!(
            Instruction::LH {
                funct3: 1,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0,
            },
            decode_operation(0x1003).unwrap()
        );
    }

    #[test]
    fn test_lb_decode() {
        assert_eq!(
            Instruction::LB {
                funct3: 0,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0,
            },
            decode_operation(0x3).unwrap()
        );
    }

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
    fn test_addi_decode() {
        assert_eq!(
            Instruction::ADDI {
                funct3: 0,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x13).unwrap()
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

    #[test]
    fn test_xori_decode() {
        assert_eq!(
            Instruction::XORI {
                funct3: 0x4,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x4013).unwrap()
        );
    }

    #[test]
    fn test_ori_decode() {
        assert_eq!(
            Instruction::ORI {
                funct3: 0x6,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x6013).unwrap()
        );
    }

    #[test]
    fn test_andi_decode() {
        assert_eq!(
            Instruction::ANDI {
                funct3: 0x7,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x7013).unwrap()
        );
    }

    #[test]
    fn test_slli_decode() {
        assert_eq!(
            Instruction::SLLI {
                funct3: 0x1,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x1013).unwrap()
        );
    }

    #[test]
    fn test_srli_decode() {
        assert_eq!(
            Instruction::SRLI {
                funct3: 0x5,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x5013).unwrap()
        );
    }

    #[test]
    fn test_srai_decode() {
        assert_eq!(
            Instruction::SRAI {
                funct3: 0x5,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0x400
            },
            decode_operation(0x40005013).unwrap()
        );
    }

    #[test]
    fn test_slti_decode() {
        assert_eq!(
            Instruction::SLTI {
                funct3: 0x2,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x2013).unwrap()
        );
    }

    #[test]
    fn test_sltiu_decode() {
        assert_eq!(
            Instruction::SLTIU {
                funct3: 0x3,
                fmt: instructions::FmtType::I,
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x3013).unwrap()
        );
    }
}
