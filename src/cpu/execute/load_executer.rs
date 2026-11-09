use crate::cpu::{execution_step::ResultSet, risc_v_cpu::RiscVCpu};

pub fn handle_load_execute(cpu: &mut RiscVCpu, rd: u8, rs1: u8, imm: u32) -> ResultSet {
    ResultSet::new((cpu.register[rs1 as usize] + imm) as i32 as u32, rd as u32)
}
