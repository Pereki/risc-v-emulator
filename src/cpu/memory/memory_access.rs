use std::error::Error;

use crate::cpu::{
    decode::instructions::Instruction,
    execution_step::{InstructionExecutionSteps, ResultSet, Step::MEMORY_ACCESS},
    memory::memory_handler::{
        handle_lb, handle_lbu, handle_lh, handle_lhu, handle_lw, handle_sb, handle_sh, handle_sw,
    },
    risc_v_cpu::RiscVCpu,
};

pub fn handle_memory_access(
    cpu: &mut RiscVCpu,
    instruction_result: &InstructionExecutionSteps,
) -> Option<Result<ResultSet, Box<dyn Error>>> {
    if !instruction_result.next_steps.contains(&MEMORY_ACCESS) {
        return None;
    }

    let result = match &instruction_result.arithmetic_result {
        Some(t) => t,
        None => {
            return Some(Err(Box::from(
                "No Execution Result was found for Memory Opereation.",
            )));
        }
    };

    match instruction_result.instruction {
        Instruction::SB {
            rs1: _,
            rs2: _,
            imm: _,
        } => Some(Ok(handle_sb(cpu, &result))),
        Instruction::SH {
            rs1: _,
            rs2: _,
            imm: _,
        } => Some(Ok(handle_sh(cpu, &result))),
        Instruction::SW {
            rs1: _,
            rs2: _,
            imm: _,
        } => Some(Ok(handle_sw(cpu, &result))),
        Instruction::LHU {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lhu(cpu, &result))),
        Instruction::LBU {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lbu(cpu, &result))),
        Instruction::LW {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lw(cpu, &result))),
        Instruction::LH {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lh(cpu, &result))),
        Instruction::LB {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lb(cpu, &result))),
        _ => Some(Err(Box::from("Did not find viable Instruction."))),
    }
}
