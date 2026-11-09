use crate::cpu::{execution_step::ResultSet, risc_v_cpu::RiscVCpu};

pub fn handle_sb(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    ResultSet::new(
        cpu.register[rs2 as usize] & 0xFF,
        cpu.register[rs1 as usize] + imm,
    )
}

pub fn handle_sh(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    ResultSet::new(
        cpu.register[rs2 as usize] & 0xFFFF,
        cpu.register[rs1 as usize] + imm,
    )
}

pub fn handle_sw(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    ResultSet::new(cpu.register[rs2 as usize], cpu.register[rs1 as usize] + imm)
}
