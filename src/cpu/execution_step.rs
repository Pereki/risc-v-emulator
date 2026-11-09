use crate::cpu::decode::instructions::Instruction;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Step {
    EXECUTE,
    MEMORY_ACCESS,
    WRITEBACK,
}

#[derive(Debug)]
pub struct ResultSet {
    pub result: u32,
    pub target: u32,
}

impl ResultSet {
    pub fn new(result: u32, target: u32) -> Self {
        Self { result, target }
    }
}

#[derive(Debug)]
pub struct InstructionExecutionSteps {
    pub instruction: Instruction,
    pub next_steps: Vec<Step>,
    pub arithmetic_result: Option<ResultSet>,
    pub memory_access_result: Option<ResultSet>,
}

impl InstructionExecutionSteps {
    pub fn new(
        instruction: Instruction,
        next_steps: Vec<Step>,
        arithmetic_result: Option<ResultSet>,
        memory_access_result: Option<ResultSet>,
    ) -> Self {
        Self {
            instruction,
            next_steps,
            arithmetic_result,
            memory_access_result,
        }
    }
}
