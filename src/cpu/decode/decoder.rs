use std::error::Error;

use crate::cpu::decode::{instruction_element_decoder, instructions::Instruction};

pub fn decode_operation(operation: u32) -> Result<Instruction, Box<dyn Error>> {
    let decoded_instruction = instruction_element_decoder::DeconstructedInstruction::new(operation);

    match (
        decoded_instruction.opcode,
        decoded_instruction.funct3,
        decoded_instruction.funct7,
    ) {
        (0x63, 0x7, _) => Ok(Instruction::BGEU {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x6, _) => Ok(Instruction::BLTU {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x5, _) => Ok(Instruction::BGE {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x4, _) => Ok(Instruction::BLT {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x1, _) => Ok(Instruction::BNE {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x63, 0x0, _) => Ok(Instruction::BEQ {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.b_type_imm,
        }),
        (0x23, 0x2, _) => Ok(Instruction::SW {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.s_type_imm,
        }),
        (0x23, 0x1, _) => Ok(Instruction::SH {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.s_type_imm,
        }),
        (0x23, 0x0, _) => Ok(Instruction::SB {
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
            imm: decoded_instruction.s_type_imm,
        }),
        (0x3, 0x5, _) => Ok(Instruction::LHU {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x3, 0x4, _) => Ok(Instruction::LBU {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x3, 0x2, _) => Ok(Instruction::LW {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x3, 0x1, _) => Ok(Instruction::LH {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x3, 0x0, _) => Ok(Instruction::LB {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x03, _) => Ok(Instruction::SLTIU {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x02, _) => Ok(Instruction::SLTI {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x05, 0x20) => Ok(Instruction::SRAI {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x05, 0x00) => Ok(Instruction::SRLI {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x01, 0x00) => Ok(Instruction::SLLI {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x07, _) => Ok(Instruction::ANDI {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x06, _) => Ok(Instruction::ORI {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x00, _) => Ok(Instruction::ADDI {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x13, 0x04, _) => Ok(Instruction::XORI {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            imm: decoded_instruction.i_type_imm,
        }),
        (0x33, 0x0, 0x00) => Ok(Instruction::ADD {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x0, 0x20) => Ok(Instruction::SUB {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x4, 0x00) => Ok(Instruction::XOR {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x6, 0x00) => Ok(Instruction::OR {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x7, 0x00) => Ok(Instruction::AND {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x1, 0x00) => Ok(Instruction::SLL {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x5, 0x00) => Ok(Instruction::SRL {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x5, 0x20) => Ok(Instruction::SRA {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x2, 0x00) => Ok(Instruction::SLT {
            rd: decoded_instruction.rd as u8,
            rs1: decoded_instruction.rs1 as u8,
            rs2: decoded_instruction.rs2 as u8,
        }),
        (0x33, 0x3, 0x00) => Ok(Instruction::SLTU {
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
                rd: 0,
                rs1: 0,
                imm: 0
            },
            decode_operation(0x3013).unwrap()
        );
    }
}
