use crate::cpu::risc_v_cpu::RiscVCpu;

mod cpu;
fn main() {
    let program = vec![0x33, 0x00, 0x00, 0x00];
    let mut cpu = RiscVCpu::new();

    cpu.load_program(&program);
    let res = cpu.fetch();
    cpu.decode(res);
}
