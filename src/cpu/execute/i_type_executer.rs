use crate::cpu::{execute::executer::ExecutionResult, risc_v_cpu::RiscVCpu};

pub fn execute_addi(cpu: &mut RiscVCpu, r1: &u8, rd: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(cpu.register[r1.clone() as usize] + imm, rd.clone())
}
