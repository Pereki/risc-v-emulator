use std::error::Error;

use crate::cpu::{
    decode::instructions::Instruction::{
        self, ADD, ADDI, AND, ANDI, BEQ, BGE, BGEU, BLT, BLTU, BNE, LB, LBU, LH, LHU, LW, OR, ORI,
        SB, SH, SLL, SLLI, SLT, SLTI, SLTIU, SLTU, SRA, SRAI, SRL, SRLI, SUB, SW, XOR, XORI,
    },
    execute::{
        b_type_executer::{
            handle_beq, handle_bge, handle_bgeu, handle_blt, handle_bltu, handle_bne,
        },
        i_type_executer::{
            handle_addi, handle_andi, handle_load_execute, handle_ori, handle_slli, handle_slti,
            handle_sltiu, handle_srai, handle_srli, handle_xori,
        },
        r_type_executer::{
            handle_add, handle_and, handle_or, handle_sll, handle_slt, handle_sltu, handle_sra,
            handle_srl, handle_sub, handle_xor,
        },
        s_type_executer::{handle_sb, handle_sh, handle_sw},
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
        BGEU {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_bgeu(risc_v_cpu, rs1, rs2, imm)),
        BLTU {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_bltu(risc_v_cpu, rs1, rs2, imm)),
        BGE {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_bge(risc_v_cpu, rs1, rs2, imm)),
        BLT {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_blt(risc_v_cpu, rs1, rs2, imm)),
        BNE {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_bne(risc_v_cpu, rs1, rs2, imm)),
        BEQ {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_beq(risc_v_cpu, rs1, rs2, imm)),
        SB {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_sb(risc_v_cpu, rs1, rs2, imm)),
        SW {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_sw(risc_v_cpu, rs1, rs2, imm)),
        SH {
            funct3: _,
            fmt: _,
            rs1,
            rs2,
            imm,
        } => Ok(handle_sh(risc_v_cpu, rs1, rs2, imm)),
        LHU {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_load_execute(risc_v_cpu, rd, rs1, imm)),
        LBU {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_load_execute(risc_v_cpu, rd, rs1, imm)),
        LW {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_load_execute(risc_v_cpu, rd, rs1, imm)),
        LH {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_load_execute(risc_v_cpu, rd, rs1, imm)),
        LB {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_load_execute(risc_v_cpu, rd, rs1, imm)),
        SLTIU {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_sltiu(risc_v_cpu, rd, rs1, imm)),
        SLTI {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_slti(risc_v_cpu, rd, rs1, imm)),
        SRAI {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_srai(risc_v_cpu, rd, rs1, imm)),
        SRLI {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_srli(risc_v_cpu, rd, rs1, imm)),
        SLLI {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_slli(risc_v_cpu, rd, rs1, imm)),

        ANDI {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_andi(risc_v_cpu, rd, rs1, imm)),
        ORI {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_ori(risc_v_cpu, rd, rs1, imm)),
        XORI {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_xori(risc_v_cpu, rd, rs1, imm)),
        ADDI {
            funct3: _,
            fmt: _,
            rd,
            rs1,
            imm,
        } => Ok(handle_addi(risc_v_cpu, rd, rs1, imm)),
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
    const DEFAULT_VALUE_IMM: u32 = 1;

    #[test]
    fn test_sltiu_execute() {
        let instr = Instruction::SLTIU {
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::I,
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
            funct3: 0,
            fmt: FmtType::I,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
        assert_eq!(res.result, (DEFAULT_VALUE_RS1 + DEFAULT_VALUE_RS2));
        assert_eq!(res.address, RD_REGISTER as u32);
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
        assert_eq!(res.result, (DEFAULT_VALUE_RS1 - DEFAULT_VALUE_RS2));
        assert_eq!(res.address, RD_REGISTER as u32);
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
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
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
        assert_eq!(res.result, 1);
        assert_eq!(res.address, RD_REGISTER as u32);
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
        assert_eq!(res.result, 1);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_sll_execute() {
        let instr = Instruction::SLL {
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
        assert_eq!(res.result, 2);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_sra_execute() {
        let instr = Instruction::SRA {
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
        assert_eq!(res.result, 0);
        assert_eq!(res.address, RD_REGISTER as u32);
    }
    #[test]
    fn test_slt_execute() {
        let instr = Instruction::SLT {
            funct3: 0,
            funct7: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            funct7: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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
