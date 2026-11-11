use crate::{cpu::risc_v_cpu::RiscVCpu, logger::logger::log};

mod cpu;
mod logger;

fn main() {
    //stupid AI test program (corrected by hand, because modern llms doesnt seem to understand hex
    //or bin)
    let program = vec![
        0x13, 0x05, 0xa0, 0x00, 0x93, 0x05, 0x40, 0x01, 0x33, 0x06, 0xb5, 0x00, 0x23, 0x2a, 0xc0,
        0x02, 0x83, 0x26, 0x40, 0x03,
        //0xe3, 0x0c, 0xC6, 0xfe,
    ];
    let mut cpu = RiscVCpu::new();
    let maxpc = program.len();

    cpu.load_program(&program);

    while cpu.program_counter < maxpc as u8 {
        let res = cpu.fetch();
        let mut instruction = match cpu.decode(res) {
            Ok(v) => v,
            Err(e) => {
                println!("Decoding Error: {}", e);
                return;
            }
        };

        instruction.arithmetic_result = match cpu.execute(&instruction) {
            Ok(e) => e,
            Err(err) => {
                println!("Execution error: {}", err);
                return;
            }
        };

        log(format!(
            "Instruction {:?} result is {:?}",
            instruction, instruction.arithmetic_result
        )
        .as_str());

        instruction.memory_access_result = match cpu.memory_access(&instruction) {
            Ok(res) => res,

            Err(e) => {
                println!("Memory Access Error: {}", e);
                return;
            }
        };

        match cpu.write_back(&instruction) {
            Ok(_) => {}
            Err(e) => {
                println!("Writeback Error: {}", e);
            }
        }

        cpu.next_operation();
    }

    cpu.print_cpu_state();
}
