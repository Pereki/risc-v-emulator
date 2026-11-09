use crate::cpu::{execution_step::ResultSet, risc_v_cpu::RiscVCpu};

pub fn handle_beq(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    if cpu.register[rs1 as usize] == cpu.register[rs2 as usize] {
        cpu.program_counter = (cpu.program_counter as i8 + imm as i8) as u8;
    }

    ResultSet::new(0, 0)
}

pub fn handle_bne(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    if cpu.register[rs1 as usize] != cpu.register[rs2 as usize] {
        cpu.program_counter = (cpu.program_counter as i8 + imm as i8) as u8;
    }

    ResultSet::new(0, 0)
}

pub fn handle_blt(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    if cpu.register[rs1 as usize] < cpu.register[rs2 as usize] {
        cpu.program_counter = (cpu.program_counter as i8 + imm as i8) as u8;
    }

    ResultSet::new(0, 0)
}

pub fn handle_bge(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    if cpu.register[rs1 as usize] >= cpu.register[rs2 as usize] {
        cpu.program_counter = (cpu.program_counter as i8 + imm as i8) as u8;
    }

    ResultSet::new(0, 0)
}

//TODO: look up what bltu should really do and if this is the correct handling
pub fn handle_bltu(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    if cpu.register[rs1 as usize] < cpu.register[rs2 as usize] {
        cpu.program_counter = (cpu.program_counter as i8 + imm as i8) as u8;
    }

    ResultSet::new(0, 0)
}

//TODO: look up what bgeu should really do and if this is the correct handling
pub fn handle_bgeu(cpu: &mut RiscVCpu, rs1: u8, rs2: u8, imm: u32) -> ResultSet {
    if cpu.register[rs1 as usize] >= cpu.register[rs2 as usize] {
        cpu.program_counter = (cpu.program_counter as i8 + imm as i8) as u8;
    }

    ResultSet::new(0, 0)
}
