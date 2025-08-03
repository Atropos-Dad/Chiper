mod opcodes;
mod display;
mod timer;
mod cpu;
mod memory;
mod reg;
mod font;


fn main() {
    println!("Chiper!");
    let mut cpu = cpu::CPU::new();

    println!("Running CHIP-8 emulator...");

    while true {
        cpu.tick();
    }
}

