mod cpu;
mod memory;
mod mmu;

fn main() {
    let mut cpu = crate::cpu::Cpu::new();
    loop {
        let instruction_byte = cpu.fetch();
        let instruction = cpu.decode(instruction_byte, cpu.pc);
        println!("{:?}", instruction);
        (instruction.execute)(&mut cpu, &instruction);
    }
}
