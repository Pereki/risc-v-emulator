use crate::cpu::{execute::executer::ExecutionResult, risc_v_cpu::RiscVCpu};

pub fn handle_sb(cpu: &mut RiscVCpu, rs1: &u8, rs2: &u8, imm: &u32) -> ExecutionResult {
    let res = ExecutionResult::new(
        cpu.register[rs2.clone() as usize] & 0xFF,
        cpu.register[rs1.clone() as usize] + imm.clone(),
    );
    return res;
}

pub fn handle_sh(cpu: &mut RiscVCpu, rs1: &u8, rs2: &u8, imm: &u32) -> ExecutionResult {
    let mut res = ExecutionResult::new(
        cpu.register[rs2.clone() as usize] & 0xFFFF,
        cpu.register[rs1.clone() as usize] + imm.clone(),
    );
    res.size = 0xFFFF;
    return res;
}

pub fn handle_sw(cpu: &mut RiscVCpu, rs1: &u8, rs2: &u8, imm: &u32) -> ExecutionResult {
    let mut res = ExecutionResult::new(
        cpu.register[rs2.clone() as usize] & 0xFFFFFFFF,
        cpu.register[rs1.clone() as usize] + imm.clone(),
    );
    res.size = 0xFFFFFFFF;
    return res;
}
