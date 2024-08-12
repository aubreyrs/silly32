mod cpu;
mod instruction;

use cpu::Silly32;

fn main() {
    let mut cpu = Silly32::new(256);
    let program = vec![
        0x09000000,
        0x0101000A,
        0x01020014,
        0x03010300,
        0x07010100,
        0x08010200,
        0x06010005,
        0x01030000,
        0x06030008,
        0x0104000A,
        0x0105000B,
    ];
    cpu.load_program(&program);
    cpu.run();

    println!("cpu state owo: {:?}", cpu);
}
