use crate::cpu::{execute::executer::ExecutionResult, risc_v_cpu::RiscVCpu};

pub fn handle_load_execute(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    let mut res = ExecutionResult::new(
        (cpu.register[*rs1 as usize] + *imm) as i32 as u32,
        *rd as u32,
    );
    res.need_memory_access = true;
    res.need_write = true;
    res
}

pub fn handle_sltiu(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(
        if cpu.register[*rs1 as usize] < *imm {
            1
        } else {
            0
        },
        *rd as u32,
    )
}

pub fn handle_slti(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(
        if (cpu.register[*rs1 as usize] as i32) < (*imm as i32) {
            1
        } else {
            0
        },
        *rd as u32,
    )
}

pub fn handle_srai(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(
        (cpu.register[*rs1 as usize] as i32 >> (*imm & 0x31)) as u32,
        *rd as u32,
    )
}

pub fn handle_srli(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(cpu.register[*rs1 as usize] >> (*imm & 0x31), *rd as u32)
}

pub fn handle_slli(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(cpu.register[*rs1 as usize] << (*imm & 0x31), *rd as u32)
}

pub fn handle_andi(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(cpu.register[*rs1 as usize] & *imm, *rd as u32)
}

pub fn handle_ori(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(cpu.register[*rs1 as usize] | *imm, *rd as u32)
}

pub fn handle_xori(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(cpu.register[*rs1 as usize] ^ *imm, *rd as u32)
}

pub fn handle_addi(cpu: &mut RiscVCpu, rd: &u8, rs1: &u8, imm: &u32) -> ExecutionResult {
    ExecutionResult::new(cpu.register[*rs1 as usize] + *imm, *rd as u32)
}

#[cfg(test)]
mod tests {

    use super::*; // Brings everything from the outer module into scope

    const RD_REGISTER: u8 = 12;
    const RS1_REGISTER: u8 = 10;
    const DEFAULT_VALUE_RS1: u32 = 1;
    const DEFAULT_IMM: u32 = 1;

    #[test]
    fn test_handle_add() {
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = handle_addi(&mut cpu, &RD_REGISTER, &RS1_REGISTER, &DEFAULT_IMM);
        assert_eq!(res.result, DEFAULT_VALUE_RS1 + DEFAULT_IMM);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
}
