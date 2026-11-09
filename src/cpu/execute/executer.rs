use std::error::Error;

use crate::cpu::{
    decode::instructions::Instruction::{
        self, ADD, ADDI, AND, ANDI, BEQ, BGE, BGEU, BLT, BLTU, BNE, LB, LBU, LH, LHU, LW, OR, ORI,
        SB, SH, SLL, SLLI, SLT, SLTI, SLTIU, SLTU, SRA, SRAI, SRL, SRLI, SUB, SW, XOR, XORI,
    },
    execute::{
        arithmetic_executer::{
            handle_add, handle_and, handle_or, handle_sll, handle_slt, handle_sltu, handle_sra,
            handle_srl, handle_sub, handle_xor,
        },
        branch_executer::{
            handle_beq, handle_bge, handle_bgeu, handle_blt, handle_bltu, handle_bne,
        },
        load_executer::handle_load_execute,
        store_executer::{handle_sb, handle_sh, handle_sw},
    },
    risc_v_cpu::RiscVCpu,
};

//TODO: make result, address and size Optional
#[derive(Debug)]
pub struct ExecutionResult {
    pub result: u32,
    pub address: u32,
    pub size: u32,
    pub need_write: bool,
    pub need_memory_access: bool,
}

fn get_register_value(cpu: &RiscVCpu, register: u8) -> u32 {
    cpu.register[register as usize]
}

impl ExecutionResult {
    pub fn new(result: u32, address: u32) -> Self {
        Self {
            result,
            address,
            size: 0xFF,
            need_write: true,
            need_memory_access: false,
        }
    }
}

pub fn execute(
    risc_v_cpu: &mut RiscVCpu,
    instruction: &Instruction,
) -> Result<ExecutionResult, Box<dyn Error>> {
    match instruction {
        BGEU { rs1, rs2, imm } => Ok(handle_bgeu(risc_v_cpu, *rs1, *rs2, *imm)),
        BLTU { rs1, rs2, imm } => Ok(handle_bltu(risc_v_cpu, *rs1, *rs2, *imm)),
        BGE { rs1, rs2, imm } => Ok(handle_bge(risc_v_cpu, *rs1, *rs2, *imm)),
        BLT { rs1, rs2, imm } => Ok(handle_blt(risc_v_cpu, *rs1, *rs2, *imm)),
        BNE { rs1, rs2, imm } => Ok(handle_bne(risc_v_cpu, *rs1, *rs2, *imm)),
        BEQ { rs1, rs2, imm } => Ok(handle_beq(risc_v_cpu, *rs1, *rs2, *imm)),
        SB { rs1, rs2, imm } => Ok(handle_sb(risc_v_cpu, *rs1, *rs2, *imm)),
        SW { rs1, rs2, imm } => Ok(handle_sw(risc_v_cpu, *rs1, *rs2, *imm)),
        SH { rs1, rs2, imm } => Ok(handle_sh(risc_v_cpu, *rs1, *rs2, *imm)),
        LHU { rd, rs1, imm } => Ok(handle_load_execute(risc_v_cpu, *rd, *rs1, *imm)),
        LBU { rd, rs1, imm } => Ok(handle_load_execute(risc_v_cpu, *rd, *rs1, *imm)),
        LW { rd, rs1, imm } => Ok(handle_load_execute(risc_v_cpu, *rd, *rs1, *imm)),
        LH { rd, rs1, imm } => Ok(handle_load_execute(risc_v_cpu, *rd, *rs1, *imm)),
        LB { rd, rs1, imm } => Ok(handle_load_execute(risc_v_cpu, *rd, *rs1, *imm)),
        SLTIU { rd, rs1, imm } => Ok(handle_sltu(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),

        SLTI { rd, rs1, imm } => Ok(handle_slt(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),
        SRAI { rd, rs1, imm } => Ok(handle_sra(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),
        SRLI { rd, rs1, imm } => Ok(handle_srl(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),
        SLLI { rd, rs1, imm } => Ok(handle_sll(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),

        ANDI { rd, rs1, imm } => Ok(handle_and(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),

        ORI { rd, rs1, imm } => Ok(handle_or(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),
        XORI { rd, rs1, imm } => Ok(handle_xor(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),
        ADDI { rd, rs1, imm } => Ok(handle_add(*rd, get_register_value(risc_v_cpu, *rs1), *imm)),
        ADD { rd, rs1, rs2 } => Ok(handle_add(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),
        SUB { rd, rs1, rs2 } => Ok(handle_sub(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),
        XOR { rd, rs1, rs2 } => Ok(handle_xor(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),
        OR { rd, rs1, rs2 } => Ok(handle_or(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),

        AND { rd, rs1, rs2 } => Ok(handle_and(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),

        SLL { rd, rs1, rs2 } => Ok(handle_sll(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),

        SRL { rd, rs1, rs2 } => Ok(handle_srl(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),

        SRA { rd, rs1, rs2 } => Ok(handle_sra(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),

        SLT { rd, rs1, rs2 } => Ok(handle_slt(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),

        SLTU { rd, rs1, rs2 } => Ok(handle_sltu(
            *rd,
            get_register_value(risc_v_cpu, *rs1),
            get_register_value(risc_v_cpu, *rs2),
        )),
    }
}

#[cfg(test)]
mod tests {

    use super::*; // Brings everything from the outer module into scope

    const RD_REGISTER: u8 = 12;
    const RS1_REGISTER: u8 = 10;
    const RS2_REGISTER: u8 = 11;
    const DEFAULT_VALUE_RS1: u32 = 1;
    const DEFAULT_VALUE_RS2: u32 = 1;
    const DEFAULT_VALUE_IMM: u32 = 1;

    #[test]
    fn test_sltiu_execute() {
        let instr = Instruction::SLTIU {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1 + 1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_slti_execute() {
        let instr = Instruction::SLTI {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1 + 1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_srai_execute() {
        let instr = Instruction::SRLI {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_srli_execute() {
        let instr = Instruction::SRLI {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_slli_execute() {
        let instr = Instruction::SLLI {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 2);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_ori_execute() {
        let instr = Instruction::ORI {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, DEFAULT_VALUE_RS1 | DEFAULT_VALUE_IMM);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_andi_execute() {
        let instr = Instruction::ANDI {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, DEFAULT_VALUE_IMM & DEFAULT_VALUE_RS1);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_xori_execute() {
        let instr = Instruction::XORI {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, (DEFAULT_VALUE_RS1 ^ DEFAULT_VALUE_IMM));
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_addi_execute() {
        let instr = Instruction::ADDI {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, (DEFAULT_VALUE_RS1 + DEFAULT_VALUE_IMM));
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_add_execute() {
        let instr = Instruction::ADD {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, (DEFAULT_VALUE_RS1 + DEFAULT_VALUE_RS2));
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_sub_execute() {
        let instr = Instruction::SUB {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, (DEFAULT_VALUE_RS1 - DEFAULT_VALUE_RS2));
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_xor_execute() {
        let instr = Instruction::XOR {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_or_execute() {
        let instr = Instruction::OR {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 1);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_and_execute() {
        let instr = Instruction::AND {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 1);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_sll_execute() {
        let instr = Instruction::SLL {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 2);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_sra_execute() {
        let instr = Instruction::SRA {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_slt_execute() {
        let instr = Instruction::SLT {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1 + 1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_sltu_execute() {
        let instr = Instruction::SLTU {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1 + 1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_lb_execute() {
        let instr = Instruction::LB {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, DEFAULT_VALUE_IMM + DEFAULT_VALUE_RS1);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_lh_execute() {
        let instr = Instruction::LH {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, DEFAULT_VALUE_IMM + DEFAULT_VALUE_RS1);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_lw_execute() {
        let instr = Instruction::LW {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, DEFAULT_VALUE_RS1 + DEFAULT_VALUE_IMM);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_lbu_execute() {
        let instr = Instruction::LBU {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, DEFAULT_VALUE_RS1 + DEFAULT_VALUE_IMM);
        assert_eq!(res.address, RD_REGISTER as u32);
    }

    #[test]
    fn test_lhu_execute() {
        let instr = Instruction::LHU {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: DEFAULT_VALUE_IMM,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        let res = execute(&mut cpu, &instr).unwrap();
        assert_eq!(res.result, DEFAULT_VALUE_IMM + DEFAULT_VALUE_RS1);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
}
