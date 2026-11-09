use std::error::Error;

use crate::cpu::{
    decode::{instruction_element_decoder, instructions::Instruction},
    execution_step::{
        InstructionExecutionSteps,
        Step::{EXECUTE, MEMORY_ACCESS, WRITEBACK},
    },
};

pub fn decode_operation(operation: u32) -> Result<InstructionExecutionSteps, Box<dyn Error>> {
    let decoded_instruction = instruction_element_decoder::DeconstructedInstruction::new(operation);

    match (
        decoded_instruction.opcode,
        decoded_instruction.funct3,
        decoded_instruction.funct7,
    ) {
        (0x63, 0x7, _) => Ok(InstructionExecutionSteps::new(
            Instruction::BGEU {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.b_type_imm,
            },
            vec![EXECUTE],
            None,
            None,
        )),
        (0x63, 0x6, _) => Ok(InstructionExecutionSteps::new(
            Instruction::BLTU {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.b_type_imm,
            },
            vec![EXECUTE],
            None,
            None,
        )),
        (0x63, 0x5, _) => Ok(InstructionExecutionSteps::new(
            Instruction::BGE {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.b_type_imm,
            },
            vec![EXECUTE],
            None,
            None,
        )),
        (0x63, 0x4, _) => Ok(InstructionExecutionSteps::new(
            Instruction::BLT {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.b_type_imm,
            },
            vec![EXECUTE],
            None,
            None,
        )),
        (0x63, 0x1, _) => Ok(InstructionExecutionSteps::new(
            Instruction::BNE {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.b_type_imm,
            },
            vec![EXECUTE],
            None,
            None,
        )),
        (0x63, 0x0, _) => Ok(InstructionExecutionSteps::new(
            Instruction::BEQ {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.b_type_imm,
            },
            vec![EXECUTE],
            None,
            None,
        )),
        (0x23, 0x2, _) => Ok(InstructionExecutionSteps::new(
            Instruction::SW {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.s_type_imm,
            },
            vec![EXECUTE, MEMORY_ACCESS],
            None,
            None,
        )),
        (0x23, 0x1, _) => Ok(InstructionExecutionSteps::new(
            Instruction::SH {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.s_type_imm,
            },
            vec![EXECUTE, MEMORY_ACCESS],
            None,
            None,
        )),
        (0x23, 0x0, _) => Ok(InstructionExecutionSteps::new(
            Instruction::SB {
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
                imm: decoded_instruction.s_type_imm,
            },
            vec![EXECUTE, MEMORY_ACCESS, WRITEBACK],
            None,
            None,
        )),
        (0x3, 0x5, _) => Ok(InstructionExecutionSteps::new(
            Instruction::LHU {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, MEMORY_ACCESS, WRITEBACK],
            None,
            None,
        )),
        (0x3, 0x4, _) => Ok(InstructionExecutionSteps::new(
            Instruction::LBU {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, MEMORY_ACCESS, WRITEBACK],
            None,
            None,
        )),
        (0x3, 0x2, _) => Ok(InstructionExecutionSteps::new(
            Instruction::LW {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, MEMORY_ACCESS, WRITEBACK],
            None,
            None,
        )),
        (0x3, 0x1, _) => Ok(InstructionExecutionSteps::new(
            Instruction::LH {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, MEMORY_ACCESS, WRITEBACK],
            None,
            None,
        )),
        (0x3, 0x0, _) => Ok(InstructionExecutionSteps::new(
            Instruction::LB {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, MEMORY_ACCESS, WRITEBACK],
            None,
            None,
        )),
        (0x13, 0x03, _) => Ok(InstructionExecutionSteps::new(
            Instruction::SLTIU {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x13, 0x02, _) => Ok(InstructionExecutionSteps::new(
            Instruction::SLTI {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x13, 0x05, 0x20) => Ok(InstructionExecutionSteps::new(
            Instruction::SRAI {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x13, 0x05, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::SRLI {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x13, 0x01, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::SLLI {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x13, 0x07, _) => Ok(InstructionExecutionSteps::new(
            Instruction::ANDI {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),

        (0x13, 0x06, _) => Ok(InstructionExecutionSteps::new(
            Instruction::ORI {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x13, 0x00, _) => Ok(InstructionExecutionSteps::new(
            Instruction::ADDI {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x13, 0x04, _) => Ok(InstructionExecutionSteps::new(
            Instruction::XORI {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                imm: decoded_instruction.i_type_imm,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x0, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::ADD {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x0, 0x20) => Ok(InstructionExecutionSteps::new(
            Instruction::SUB {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x4, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::XOR {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x6, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::OR {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x7, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::AND {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x1, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::SLL {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x5, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::SRL {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x5, 0x20) => Ok(InstructionExecutionSteps::new(
            Instruction::SRA {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x2, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::SLT {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        (0x33, 0x3, 0x00) => Ok(InstructionExecutionSteps::new(
            Instruction::SLTU {
                rd: decoded_instruction.rd as u8,
                rs1: decoded_instruction.rs1 as u8,
                rs2: decoded_instruction.rs2 as u8,
            },
            vec![EXECUTE, WRITEBACK],
            None,
            None,
        )),
        _ => Err(Box::from("No viable OpCode was found.")),
    }
}
