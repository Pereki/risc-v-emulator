use crate::cpu::{execute::executer::ExecutionResult, risc_v_cpu::RiscVCpu};

pub fn handle_sb(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ExecutionResult {
    let mut res = ExecutionResult::new(
        cpu.register[rs2 as usize] & 0xFF,
        cpu.register[rs1 as usize] + imm,
    );
    res.need_write = false;
    res.need_memory_access = true;
    res
}

pub fn handle_sh(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ExecutionResult {
    let mut res = ExecutionResult::new(
        cpu.register[rs2 as usize] & 0xFFFF,
        cpu.register[rs1 as usize] + imm,
    );
    res.size = 0xFFFF;
    res.need_write = false;
    res.need_memory_access = true;
    res
}

pub fn handle_sw(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ExecutionResult {
    let mut res =
        ExecutionResult::new(cpu.register[rs2 as usize], cpu.register[rs1 as usize] + imm);
    res.size = 0xFFFFFFFF;
    res.need_write = false;
    res.need_memory_access = true;
    res
}
