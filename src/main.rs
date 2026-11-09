use crate::{cpu::risc_v_cpu::RiscVCpu, logger::logger::log};

mod cpu;
mod logger;

fn main() {
    //stupid AI test program (corrected by hand, because modern llms doesnt seem to understand hex
    //or bin)
    let program = vec![
        0x13, 0x05, 0xa0, 0x00, 0x93, 0x05, 0x40, 0x01, 0x33, 0x06, 0xb5, 0x00, 0x23, 0x2a, 0xc0,
        0x02, 0x83, 0x26, 0x40, 0x03, 0xe3, 0x0c, 0xC6, 0xfe,
    ];
    let mut cpu = RiscVCpu::new();
    let maxpc = program.len();

    cpu.load_program(&program);

    while cpu.program_counter <= maxpc as u8 {
        let res = cpu.fetch();
        let instruction = match cpu.decode(res) {
            Ok(v) => v,
            Err(e) => {
                println!("Decoding Error: {}", e);
                return;
            }
        };

        let execute_result = match cpu.execute(&instruction) {
            Ok(v) => v,
            Err(e) => {
                println!("Execution error: {}", e);
                return;
            }
        };

        log(format!(
            "Instruction {:?} result is {:?}",
            instruction, execute_result
        )
        .as_str());

        let new_or_old_result = match cpu.memory_access(&instruction, &execute_result) {
            Some(res) => match res {
                Ok(result) => result,
                Err(e) => {
                    println!("Memory Access Error: {}", e);
                    return;
                }
            },
            None => execute_result,
        };
        cpu.write_back(
            new_or_old_result.address as u8,
            new_or_old_result.result,
            new_or_old_result.need_write,
        );

        cpu.next_operation();
    }

    cpu.print_cpu_state();
}
