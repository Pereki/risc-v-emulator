use std::error::Error;

use crate::cpu::{
    decode::instructions::{
        FmtType,
        Instruction::{self, ADD},
    },
    risc_v_cpu::RiscVCpu,
};

pub struct ExecutionResult {
    pub result: u32,
    pub register: u8,
}

impl ExecutionResult {
    pub fn new(result: u32, register: u8) -> Self {
        Self { result, register }
    }
}

pub fn execute(
    risc_v_cpu: &mut RiscVCpu,
    instruction: &Instruction,
) -> Result<ExecutionResult, Box<dyn Error>> {
    match instruction {
        ADD {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_add(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        _ => Err(Box::from("Could not find executor for instruction.")),
    }
}

fn handle_add(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        cpu.register[rs1.clone() as usize] + cpu.register[rs2.clone() as usize],
        rd.clone(),
    );
}

#[cfg(test)]
mod tests {
    use super::*; // Brings everything from the outer module into scope

    const RD_REGISTER: u8 = 12;
    const RS1_REGISTER: u8 = 10;
    const RS2_REGISTER: u8 = 11;
    const DEFAULT_VALUE_RS1: u32 = 1;
    const DEFAULT_VALUE_RS2: u32 = 1;

    #[test]
    fn test_handle_add() {
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = handle_add(
            &mut cpu,
            &1,
            &1,
            &FmtType::R,
            &RD_REGISTER,
            &RS1_REGISTER,
            &RS2_REGISTER,
        );
        assert_eq!(res.result, DEFAULT_VALUE_RS1 + DEFAULT_VALUE_RS2);
        assert_eq!(res.register, RD_REGISTER);
    }

    #[test]
    fn test_execute() {
        let instr = Instruction::ADD {
            funct3: 0,
            funct7: 0,
            fmt: FmtType::R,
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, (DEFAULT_VALUE_RS1 + DEFAULT_VALUE_RS2) as u32);
        assert_eq!(res.register, RD_REGISTER);
    }
}
