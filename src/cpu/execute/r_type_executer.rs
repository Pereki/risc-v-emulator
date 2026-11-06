use crate::cpu::{
    decode::instructions::FmtType, execute::executer::ExecutionResult, risc_v_cpu::RiscVCpu,
};

pub fn handle_sltu(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        if cpu.register[rs1.clone() as usize] < cpu.register[rs2.clone() as usize] {
            1
        } else {
            0
        },
        rd.clone(),
    );
}

pub fn handle_slt(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        if (cpu.register[rs1.clone() as usize] as i32) < (cpu.register[rs2.clone() as usize] as i32)
        {
            1
        } else {
            0
        },
        rd.clone(),
    );
}

pub fn handle_sra(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        (cpu.register[rs1.clone() as usize] as i32 >> cpu.register[rs2.clone() as usize]) as u32,
        rd.clone(),
    );
}

pub fn handle_srl(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        cpu.register[rs1.clone() as usize] >> cpu.register[rs2.clone() as usize],
        rd.clone(),
    );
}

pub fn handle_sll(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        cpu.register[rs1.clone() as usize] << cpu.register[rs2.clone() as usize],
        rd.clone(),
    );
}

pub fn handle_and(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        cpu.register[rs1.clone() as usize] & cpu.register[rs2.clone() as usize],
        rd.clone(),
    );
}

pub fn handle_or(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        cpu.register[rs1.clone() as usize] | cpu.register[rs2.clone() as usize],
        rd.clone(),
    );
}

pub fn handle_xor(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        cpu.register[rs1.clone() as usize] ^ cpu.register[rs2.clone() as usize],
        rd.clone(),
    );
}

pub fn handle_sub(
    cpu: &mut RiscVCpu,
    _: &u8,
    _: &u8,
    _: &FmtType,
    rd: &u8,
    rs1: &u8,
    rs2: &u8,
) -> ExecutionResult {
    return ExecutionResult::new(
        cpu.register[rs1.clone() as usize] - cpu.register[rs2.clone() as usize],
        rd.clone(),
    );
}

pub fn handle_add(
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
}
