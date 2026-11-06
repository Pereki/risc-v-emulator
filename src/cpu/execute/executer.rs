use std::error::Error;

use crate::cpu::{
    decode::instructions::Instruction::{self, ADD, AND, OR, SLL, SLT, SLTU, SRA, SRL, SUB, XOR},
    execute::r_type_executer::{
        handle_add, handle_and, handle_or, handle_sll, handle_slt, handle_sltu, handle_sra,
        handle_srl, handle_sub, handle_xor,
    },
    risc_v_cpu::RiscVCpu,
};

#[derive(Debug)]
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
        SUB {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_sub(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        XOR {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_xor(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        OR {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_or(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        AND {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_and(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        SLL {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_sll(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        SRL {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_srl(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        SRA {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_sra(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        SLT {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_slt(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        SLTU {
            funct3,
            funct7,
            fmt,
            rd,
            rs1,
            rs2,
        } => Ok(handle_sltu(risc_v_cpu, funct3, funct7, fmt, rd, rs1, rs2)),
        _ => Err(Box::from("Could not find executor for instruction.")),
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::decode::instructions::FmtType;

    use super::*; // Brings everything from the outer module into scope

    const RD_REGISTER: u8 = 12;
    const RS1_REGISTER: u8 = 10;
    const RS2_REGISTER: u8 = 11;
    const DEFAULT_VALUE_RS1: u32 = 1;
    const DEFAULT_VALUE_RS2: u32 = 1;

    #[test]
    fn test_add_execute() {
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

    #[test]
    fn test_sub_execute() {
        let instr = Instruction::SUB {
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
        assert_eq!(res.result, (DEFAULT_VALUE_RS1 - DEFAULT_VALUE_RS2) as u32);
        assert_eq!(res.register, RD_REGISTER);
    }

    #[test]
    fn test_xor_execute() {
        let instr = Instruction::XOR {
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
        assert_eq!(res.result, 0 as u32);
        assert_eq!(res.register, RD_REGISTER);
    }

    #[test]
    fn test_or_execute() {
        let instr = Instruction::OR {
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
        assert_eq!(res.result, 1 as u32);
        assert_eq!(res.register, RD_REGISTER);
    }

    #[test]
    fn test_and_execute() {
        let instr = Instruction::AND {
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
        assert_eq!(res.result, 1 as u32);
        assert_eq!(res.register, RD_REGISTER);
    }
}
