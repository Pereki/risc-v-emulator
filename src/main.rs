use crate::{cpu::risc_v_cpu::RiscVCpu, logger::logger::log};

mod cpu;
mod logger;
fn main() {
    let program = vec![0x33, 0x85, 0xC5, 0x00, 0xB3, 0x06, 0xB5, 0x40];
    let mut cpu = RiscVCpu::new();
    let instructioncount = 2;

    //FOR TESTING PURPOSE ONLY.
    //Load Values into the registers for add.
    cpu.register[11] = 1;
    cpu.register[12] = 1;

    cpu.load_program(&program);

    for _ in 0..instructioncount {
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

        cpu.memory_access(&instruction);
        cpu.write_back(execute_result.register, execute_result.result);

        cpu.next_operation();
    }
}
