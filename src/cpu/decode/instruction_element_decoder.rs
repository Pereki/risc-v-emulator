pub struct DeconstructedInstruction {
    pub opcode: u32,
    pub funct3: u32,
    pub funct7: u32,
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub i_type_imm: u32,
    pub b_type_imm: u32,
    pub s_type_imm: u32,
}

impl DeconstructedInstruction {
    pub fn new(instruction: u32) -> Self {
        Self {
            opcode: instruction & 0x7F,
            funct3: (instruction & 0x7000) >> 12,
            funct7: (instruction & 0xFC000000) >> 25,
            rd: (instruction & 0xF80) >> 7,
            rs1: (instruction & 0xF8000) >> 15,
            rs2: (instruction & 0x1F00000) >> 20,
            i_type_imm: (instruction & 0xFFF00000) >> 20,
            b_type_imm: get_b_type_imm(&instruction),
            s_type_imm: get_s_type_imm(&instruction),
        }
    }
}

fn get_s_type_imm(instruction: &u32) -> u32 {
    (instruction & 0xF80) >> 7 | ((instruction & 0xFE000000) >> 25 << 5)
}

fn get_b_type_imm(instruction: &u32) -> u32 {
    (((instruction & 0x80) >> 7) << 11)
        | (((instruction & 0xf00) >> 8) << 1)
        | (((instruction & 0x7E000000) >> 25) << 5)
        | ((instruction & 0x80000000 >> 31) << 12)
}
