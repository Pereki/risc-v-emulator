use crate::cpu::execute::executer::ExecutionResult;

pub fn handle_sltu(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(if first < second { 1 } else { 0 }, target_register as u32)
}

pub fn handle_slt(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(
        if (first as i32) < (second as i32) {
            1
        } else {
            0
        },
        target_register as u32,
    )
}

pub fn handle_sra(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(((first as i32) >> second) as u32, target_register as u32)
}

pub fn handle_srl(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(first >> second, target_register as u32)
}

pub fn handle_sll(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(first << second, target_register as u32)
}

pub fn handle_and(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(first & second, target_register as u32)
}

pub fn handle_or(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(first | second, target_register as u32)
}

pub fn handle_xor(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(first ^ second, target_register as u32)
}

pub fn handle_sub(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(first - second, target_register as u32)
}

pub fn handle_add(target_register: u8, first: u32, second: u32) -> ExecutionResult {
    ExecutionResult::new(first + second, target_register as u32)
}
