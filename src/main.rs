use std::error::Error;

use crate::cpu::risc_v_cpu::RiscVCpu;

mod cpu;
fn main() {
    let program = vec![0x33, 0x85, 0xC5, 0x00];
    let mut cpu = RiscVCpu::new();

    //FOR TESTING PURPOSE ONLY.
    //Load Values into the registers for add.
    cpu.register[11] = 1;
    cpu.register[12] = 1;

    cpu.load_program(&program);
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

    cpu.memory_access(&instruction);
    cpu.write_back(execute_result.register, execute_result.result);

    cpu.next_operation();

    println!("Value in Register 10 is: [{}]", cpu.register[10]);
}
