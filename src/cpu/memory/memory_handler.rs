use crate::cpu::{execute::executer::ExecutionResult, risc_v_cpu::RiscVCpu};

pub fn handle_sb(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) -> ExecutionResult {
    cpu.memory[execution_result.address as usize] = (execution_result.result) as u8;

    let mut exec_res = ExecutionResult::new(0, 0);
    exec_res.need_write = false;
    exec_res
}

pub fn handle_sh(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) -> ExecutionResult {
    cpu.memory[execution_result.address as usize] = ((execution_result.result) & 0xFF) as u8;

    cpu.memory[(execution_result.address + 1) as usize] =
        (((execution_result.result) & 0xFF00) >> 8) as u8;

    let mut exec_res = ExecutionResult::new(0, 0);
    exec_res.need_write = false;
    exec_res
}

pub fn handle_sw(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) -> ExecutionResult {
    cpu.memory[execution_result.address as usize] = ((execution_result.result) & 0xFF) as u8;

    cpu.memory[(execution_result.address + 1) as usize] =
        (((execution_result.result) & 0xFF00) >> 8) as u8;

    cpu.memory[(execution_result.address + 2) as usize] =
        (((execution_result.result) & 0xFF0000) >> 16) as u8;

    cpu.memory[(execution_result.address + 3) as usize] =
        (((execution_result.result) & 0xFF000000) >> 24) as u8;

    let mut exec_res = ExecutionResult::new(0, 0);
    exec_res.need_write = false;
    exec_res
}

pub fn handle_lhu(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) -> ExecutionResult {
    ExecutionResult::new(
        (cpu.memory[(execution_result.result + 1) as usize] as i32 as u32) << 8
            | cpu.memory[execution_result.result as usize] as i32 as u32,
        execution_result.address,
    )
}

pub fn handle_lbu(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) -> ExecutionResult {
    ExecutionResult::new(
        cpu.memory[execution_result.result as usize] as i32 as u32,
        execution_result.address,
    )
}

pub fn handle_lw(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) -> ExecutionResult {
    ExecutionResult::new(
        (cpu.memory[execution_result.result as usize + 3] as u32) << 24
            | (cpu.memory[execution_result.result as usize + 2] as u32) << 16
            | (cpu.memory[execution_result.result as usize + 1] as u32) << 8
            | cpu.memory[execution_result.result as usize] as u32,
        execution_result.address,
    )
}

pub fn handle_lh(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) -> ExecutionResult {
    ExecutionResult::new(
        (cpu.memory[(execution_result.result + 1) as usize] as u32) << 8
            | cpu.memory[execution_result.result as usize] as u32,
        execution_result.address,
    )
}

pub fn handle_lb(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) -> ExecutionResult {
    ExecutionResult::new(
        cpu.memory[execution_result.result as usize] as u32,
        execution_result.address,
    )
}
