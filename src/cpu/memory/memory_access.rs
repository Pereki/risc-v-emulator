use crate::cpu::{
    decode::instructions::Instruction, execute::executer::ExecutionResult, risc_v_cpu::RiscVCpu,
};

pub fn handle_memory_access(
    cpu: &mut RiscVCpu,
    instruction: &Instruction,
    execution_result: &ExecutionResult,
) -> bool {
    let _ = match instruction {
        Instruction::SB {
            funct3: _,
            fmt: _,
            rs1: _,
            rs2: _,
            imm: _,
        } => handle_sb(cpu, &execution_result),
        Instruction::SH {
            funct3: _,
            fmt: _,
            rs1: _,
            rs2: _,
            imm: _,
        } => handle_sh(cpu, &execution_result),
        Instruction::SW {
            funct3: _,
            fmt: _,
            rs1: _,
            rs2: _,
            imm: _,
        } => handle_sw(cpu, &execution_result),
        Instruction::LHU {
            funct3: _,
            fmt: _,
            rd: _,
            rs1: _,
            imm: _,
        } => handle_lhu(cpu, &execution_result),
        Instruction::LBU {
            funct3: _,
            fmt: _,
            rd: _,
            rs1: _,
            imm: _,
        } => handle_lbu(cpu, &execution_result),
        Instruction::LW {
            funct3: _,
            fmt: _,
            rd: _,
            rs1: _,
            imm: _,
        } => handle_lw(cpu, &execution_result),
        Instruction::LH {
            funct3: _,
            fmt: _,
            rd: _,
            rs1: _,
            imm: _,
        } => handle_lh(cpu, &execution_result),
        Instruction::LB {
            funct3: _,
            fmt: _,
            rd: _,
            rs1: _,
            imm: _,
        } => handle_lb(cpu, &execution_result),
        _ => {
            return false;
        }
    };

    return true;
}

pub fn handle_sb(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) {
    cpu.memory[execution_result.address as usize] =
        (execution_result.result & execution_result.size) as u8;
}

pub fn handle_sh(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) {
    cpu.memory[execution_result.address as usize] =
        ((execution_result.result & execution_result.size) & 0xFF) as u8;

    cpu.memory[(execution_result.address + 1) as usize] =
        (((execution_result.result & execution_result.size) & 0xFF00) >> 8) as u8;
}

pub fn handle_sw(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) {
    cpu.memory[execution_result.address as usize] =
        ((execution_result.result & execution_result.size) & 0xFF) as u8;

    cpu.memory[(execution_result.address + 1) as usize] =
        (((execution_result.result & execution_result.size) & 0xFF00) >> 8) as u8;

    cpu.memory[(execution_result.address + 2) as usize] =
        (((execution_result.result & execution_result.size) & 0xFF0000) >> 16) as u8;

    cpu.memory[(execution_result.address + 3) as usize] =
        (((execution_result.result & execution_result.size) & 0xFF000000) >> 24) as u8;
}

pub fn handle_lhu(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) {
    cpu.register[execution_result.address as usize] =
        (cpu.memory[(execution_result.result + 1) as usize] as i32 as u32) << 8
            | cpu.memory[execution_result.result as usize] as i32 as u32;
}

pub fn handle_lbu(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) {
    cpu.register[execution_result.address as usize] =
        cpu.memory[execution_result.result as usize] as i32 as u32;
}

pub fn handle_lw(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) {
    cpu.register[execution_result.address as usize] =
        (cpu.memory[execution_result.result as usize + 3] as u32) << 24
            | (cpu.memory[execution_result.result as usize + 2] as u32) << 16
            | (cpu.memory[execution_result.result as usize + 1] as u32) << 8
            | cpu.memory[execution_result.result as usize] as u32;
}

pub fn handle_lh(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) {
    cpu.register[execution_result.address as usize] =
        (cpu.memory[(execution_result.result + 1) as usize] as u32) << 8
            | cpu.memory[execution_result.result as usize] as u32;
}

pub fn handle_lb(cpu: &mut RiscVCpu, execution_result: &ExecutionResult) {
    cpu.register[execution_result.address as usize] =
        cpu.memory[execution_result.result as usize] as u32;
}

#[cfg(test)]
mod tests {
    use crate::cpu::{decode::instructions::FmtType, execute::executer::execute};

    use super::*; // Brings everything from the outer module into scope

    const RD_REGISTER: u8 = 12;
    const RS1_REGISTER: u8 = 10;
    const RS2_REGISTER: u8 = 11;
    const DEFAULT_VALUE_RS1: u32 = 1;
    const DEFAULT_VALUE_RS2: u32 = 1;
    const DEFAULT_VALUE_IMM: u32 = 1;

    #[test]
    fn test_sb() {
        let instr = Instruction::SB {
            funct3: 0,
            fmt: FmtType::R,
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
            funct3: 0,
            fmt: FmtType::R,
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

        assert_eq!(value, res.result as u32);
    }

    #[test]
    fn test_sw() {
        let instr = Instruction::SW {
            funct3: 0,
            fmt: FmtType::R,
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
        assert_eq!(value, res.result as u32);
    }

    #[test]
    fn test_lw() {
        let instr = Instruction::LW {
            funct3: 0,
            fmt: FmtType::R,
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

        let value = ((cpu.memory[res.result as usize + 3] as u32) << 24)
            | ((cpu.memory[res.result as usize + 2] as u32) << 16)
            | ((cpu.memory[res.result as usize + 1] as u32) << 8)
            | (cpu.memory[(res.result) as usize] as u32);

        assert_eq!(value, cpu.register[RD_REGISTER as usize] as u32);
    }

    #[test]
    fn test_lh() {
        let instr = Instruction::LH {
            funct3: 0,
            fmt: FmtType::R,
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

        let value = ((cpu.memory[res.result as usize + 1] as u32) << 8)
            | (cpu.memory[(res.result) as usize] as u32);

        assert_eq!(value, cpu.register[RD_REGISTER as usize] as u32);
    }

    #[test]
    fn test_lhu() {
        let instr = Instruction::LHU {
            funct3: 0,
            fmt: FmtType::R,
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

        let value = ((cpu.memory[res.result as usize + 1] as u32) << 8)
            | (cpu.memory[(res.result) as usize] as u32);

        assert_eq!(value, cpu.register[RD_REGISTER as usize] as u32);
    }

    #[test]
    fn test_lb() {
        let instr = Instruction::LB {
            funct3: 0,
            fmt: FmtType::R,
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

        let value = (cpu.memory[(res.result) as usize] as u32);

        assert_eq!(value, cpu.register[RD_REGISTER as usize] as u32);
    }

    #[test]
    fn test_lbu() {
        let instr = Instruction::LBU {
            funct3: 0,
            fmt: FmtType::R,
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

        let value = (cpu.memory[(res.result) as usize] as u32);

        assert_eq!(value, cpu.register[RD_REGISTER as usize] as u32);
    }
}
