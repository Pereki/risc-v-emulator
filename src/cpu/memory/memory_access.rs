use std::error::Error;

use crate::cpu::{
    decode::instructions::Instruction,
    execute::executer::ExecutionResult,
    memory::memory_handler::{
        handle_lb, handle_lbu, handle_lh, handle_lhu, handle_lw, handle_sb, handle_sh, handle_sw,
    },
    risc_v_cpu::RiscVCpu,
};

pub fn handle_memory_access(
    cpu: &mut RiscVCpu,
    instruction: &Instruction,
    execution_result: &ExecutionResult,
) -> Option<Result<ExecutionResult, Box<dyn Error>>> {
    if !execution_result.need_memory_access {
        return None;
    }

    match instruction {
        Instruction::SB {
            rs1: _,
            rs2: _,
            imm: _,
        } => Some(Ok(handle_sb(cpu, execution_result))),
        Instruction::SH {
            rs1: _,
            rs2: _,
            imm: _,
        } => Some(Ok(handle_sh(cpu, execution_result))),
        Instruction::SW {
            rs1: _,
            rs2: _,
            imm: _,
        } => Some(Ok(handle_sw(cpu, execution_result))),
        Instruction::LHU {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lhu(cpu, execution_result))),
        Instruction::LBU {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lbu(cpu, execution_result))),
        Instruction::LW {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lw(cpu, execution_result))),
        Instruction::LH {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lh(cpu, execution_result))),
        Instruction::LB {
            rd: _,
            rs1: _,
            imm: _,
        } => Some(Ok(handle_lb(cpu, execution_result))),
        _ => Some(Err(Box::from("Did not find viable Instruction."))),
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::execute::executer::execute;

    use super::*; // Brings everything from the outer module into scope

    const RD_REGISTER: u8 = 12;
    const RS1_REGISTER: u8 = 10;
    const RS2_REGISTER: u8 = 11;
    const DEFAULT_VALUE_RS1: u32 = 1;
    const DEFAULT_VALUE_RS2: u32 = 1;

    #[test]
    fn test_sb() {
        let instr = Instruction::SB {
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
            imm: 0,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2 + 12;
        let res = execute(&mut cpu, &instr).unwrap();
        handle_memory_access(&mut cpu, &instr, &res);

        assert_eq!(cpu.memory[res.address as usize], res.result as u8);
    }

    #[test]
    fn test_sh() {
        let instr = Instruction::SH {
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
            imm: 0,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2 + 12;
        let res = execute(&mut cpu, &instr).unwrap();
        handle_memory_access(&mut cpu, &instr, &res);

        let value = ((cpu.memory[res.address as usize + 1] as u32) << 8)
            | (cpu.memory[(res.address) as usize] as u32);

        assert_eq!(value, res.result);
    }

    #[test]
    fn test_sw() {
        let instr = Instruction::SW {
            rs1: RS1_REGISTER,
            rs2: RS2_REGISTER,
            imm: 0,
        };
        let mut cpu = RiscVCpu::new();
        cpu.register[RS1_REGISTER as usize] = DEFAULT_VALUE_RS1;
        cpu.register[RS2_REGISTER as usize] = DEFAULT_VALUE_RS2 + 12;
        let res = execute(&mut cpu, &instr).unwrap();
        handle_memory_access(&mut cpu, &instr, &res);

        let value = ((cpu.memory[res.address as usize + 3] as u32) << 24)
            | ((cpu.memory[res.address as usize + 2] as u32) << 16)
            | ((cpu.memory[res.address as usize + 1] as u32) << 8)
            | (cpu.memory[(res.address) as usize] as u32);
        assert_eq!(value, res.result);
    }

    #[test]
    fn test_lw() {
        let instr = Instruction::LW {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: 0,
        };
        let mut cpu = RiscVCpu::new();
        cpu.memory[100] = 0x55;
        cpu.memory[101] = 0x11;
        cpu.memory[102] = 0x22;
        cpu.memory[103] = 0x33;

        cpu.register[RS1_REGISTER as usize] = 100;
        let res = execute(&mut cpu, &instr).unwrap();

        let memory_result = handle_memory_access(&mut cpu, &instr, &res)
            .unwrap()
            .unwrap();

        assert_eq!(memory_result.address, RD_REGISTER as u32);
        assert_eq!(memory_result.result, 857870677);
    }

    #[test]
    fn test_lh() {
        let instr = Instruction::LH {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: 0,
        };
        let mut cpu = RiscVCpu::new();
        cpu.memory[100] = 0x55;
        cpu.memory[101] = 0x11;
        cpu.memory[102] = 0x22;
        cpu.memory[103] = 0x33;

        cpu.register[RS1_REGISTER as usize] = 100;
        let res = execute(&mut cpu, &instr).unwrap();
        handle_memory_access(&mut cpu, &instr, &res);

        let memory_result = handle_memory_access(&mut cpu, &instr, &res)
            .unwrap()
            .unwrap();

        assert_eq!(memory_result.address, RD_REGISTER as u32);
        assert_eq!(memory_result.result, 4437);
    }

    #[test]
    fn test_lhu() {
        let instr = Instruction::LHU {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: 0,
        };
        let mut cpu = RiscVCpu::new();
        cpu.memory[100] = 0x55;
        cpu.memory[101] = 0x11;
        cpu.memory[102] = 0x22;
        cpu.memory[103] = 0x33;

        cpu.register[RS1_REGISTER as usize] = 100;
        let res = execute(&mut cpu, &instr).unwrap();

        let memory_result = handle_memory_access(&mut cpu, &instr, &res)
            .unwrap()
            .unwrap();

        assert_eq!(memory_result.address, RD_REGISTER as u32);
        assert_eq!(memory_result.result, 4437);
    }

    #[test]
    fn test_lb() {
        let instr = Instruction::LB {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: 0,
        };
        let mut cpu = RiscVCpu::new();
        cpu.memory[100] = 0x55;
        cpu.memory[101] = 0x11;
        cpu.memory[102] = 0x22;
        cpu.memory[103] = 0x33;

        cpu.register[RS1_REGISTER as usize] = 100;
        let res = execute(&mut cpu, &instr).unwrap();
        let memory_result = handle_memory_access(&mut cpu, &instr, &res)
            .unwrap()
            .unwrap();

        assert_eq!(memory_result.address, RD_REGISTER as u32);
        assert_eq!(memory_result.result, 85);
    }

    #[test]
    fn test_lbu() {
        let instr = Instruction::LBU {
            rd: RD_REGISTER,
            rs1: RS1_REGISTER,
            imm: 0,
        };
        let mut cpu = RiscVCpu::new();
        cpu.memory[100] = 0x55;
        cpu.memory[101] = 0x11;
        cpu.memory[102] = 0x22;
        cpu.memory[103] = 0x33;

        cpu.register[RS1_REGISTER as usize] = 100;
        let res = execute(&mut cpu, &instr).unwrap();
        let memory_result = handle_memory_access(&mut cpu, &instr, &res)
            .unwrap()
            .unwrap();

        assert_eq!(memory_result.address, RD_REGISTER as u32);
        assert_eq!(memory_result.result, 85);
    }
}
