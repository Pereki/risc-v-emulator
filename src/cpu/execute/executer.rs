use std::error::Error;

use crate::cpu::{
    decode::instructions::Instruction::{
        ADD, ADDI, AND, ANDI, BEQ, BGE, BGEU, BLT, BLTU, BNE, LB, LBU, LH, LHU, LW, OR, ORI, SB,
        SH, SLL, SLLI, SLT, SLTI, SLTIU, SLTU, SRA, SRAI, SRL, SRLI, SUB, SW, XOR, XORI,
    },
    execute::{
        arithmetic_executer::{
            handle_add, handle_and, handle_or, handle_sll, handle_slt, handle_sltu, handle_sra,
            handle_srl, handle_sub, handle_xor,
        },
        branch_executer::{
            handle_beq, handle_bge, handle_bgeu, handle_blt, handle_bltu, handle_bne,
        },
        load_executer::handle_load_execute,
        store_executer::{handle_sb, handle_sh, handle_sw},
    },
    execution_step::{InstructionExecutionSteps, ResultSet, Step::EXECUTE},
    risc_v_cpu::RiscVCpu,
};

fn get_register_value(cpu: &RiscVCpu, register: u8) -> u32 {
    cpu.register[register as usize]
}

pub fn execute(
    risc_v_cpu: &mut RiscVCpu,
    instruction_execution_steps: &InstructionExecutionSteps,
) -> Result<Option<ResultSet>, Box<dyn Error>> {
    if !instruction_execution_steps.next_steps.contains(&EXECUTE) {
        return Ok(None);
    }

    let instruction = &instruction_execution_steps.instruction;

    Ok(Some(match instruction {
        BGEU { rs1, rs2, imm } => handle_bgeu(risc_v_cpu, *rs1, *rs2, *imm),
        BLTU { rs1, rs2, imm } => handle_bltu(risc_v_cpu, *rs1, *rs2, *imm),
        BGE { rs1, rs2, imm } => handle_bge(risc_v_cpu, *rs1, *rs2, *imm),
        BLT { rs1, rs2, imm } => handle_blt(risc_v_cpu, *rs1, *rs2, *imm),
        BNE { rs1, rs2, imm } => handle_bne(risc_v_cpu, *rs1, *rs2, *imm),
        BEQ { rs1, rs2, imm } => handle_beq(risc_v_cpu, *rs1, *rs2, *imm),
        SB { rs1, rs2, imm } => handle_sb(risc_v_cpu, *rs1, *rs2, *imm),
        SW { rs1, rs2, imm } => handle_sw(risc_v_cpu, *rs1, *rs2, *imm),
        SH { rs1, rs2, imm } => handle_sh(risc_v_cpu, *rs1, *rs2, *imm),
        LHU { rd, rs1, imm } => handle_load_execute(risc_v_cpu, *rd, *rs1, *imm),
        LBU { rd, rs1, imm } => handle_load_execute(risc_v_cpu, *rd, *rs1, *imm),
        LW { rd, rs1, imm } => handle_load_execute(risc_v_cpu, *rd, *rs1, *imm),
        LH { rd, rs1, imm } => handle_load_execute(risc_v_cpu, *rd, *rs1, *imm),
        LB { rd, rs1, imm } => handle_load_execute(risc_v_cpu, *rd, *rs1, *imm),
        SLTIU { rd, rs1, imm } => handle_sltu(*rd, get_register_value(risc_v_cpu, *rs1), *imm),

        SLTI { rd, rs1, imm } => handle_slt(*rd, get_register_value(risc_v_cpu, *rs1), *imm),
        SRAI { rd, rs1, imm } => handle_sra(*rd, get_register_value(risc_v_cpu, *rs1), *imm),
        SRLI { rd, rs1, imm } => handle_srl(*rd, get_register_value(risc_v_cpu, *rs1), *imm),
        SLLI { rd, rs1, imm } => handle_sll(*rd, get_register_value(risc_v_cpu, *rs1), *imm),

        ANDI { rd, rs1, imm } => handle_and(*rd, get_register_value(risc_v_cpu, *rs1), *imm),

        ORI { rd, rs1, imm } => handle_or(*rd, get_register_value(risc_v_cpu, *rs1), *imm),
        XORI { rd, rs1, imm } => handle_xor(*rd, get_register_value(risc_v_cpu, *rs1), *imm),
        ADDI { rd, rs1, imm } => handle_add(*rd, get_register_value(risc_v_cpu, *rs1), *imm),
        ADD { rd, rs1, rs2 } => handle_add(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),
        SUB { rd, rs1, rs2 } => handle_sub(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),
        XOR { rd, rs1, rs2 } => handle_xor(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),
        OR { rd, rs1, rs2 } => handle_or(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),

        AND { rd, rs1, rs2 } => handle_and(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),

        SLL { rd, rs1, rs2 } => handle_sll(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),

        SRL { rd, rs1, rs2 } => handle_srl(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),

        SRA { rd, rs1, rs2 } => handle_sra(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),

        SLT { rd, rs1, rs2 } => handle_slt(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),

        SLTU { rd, rs1, rs2 } => handle_sltu(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        ),
    }))
}
