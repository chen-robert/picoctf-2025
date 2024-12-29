use std::error::Error;
use std::fs;
use verilog_ctf::simulator::{run_program, MEM_SIZE};
use verilog_ctf::assembler::assemble;

mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("programs/nand_checker.asm")?;

    let prog2 = "\
    LOADW r7 0xf000 
    LOADW r6 0xf000 
    LOADW r0 0x6F73 ; 'os'
    LOADW r1 0x6563 ; 'ec'
    LOADW r2 0x2E69 ; '.i'
    LOADW r3 0x6F00 ; 'o\0'
    ADD r7 r7
    ADD r7 r7
    ADD r7 r7
    ADD r7 r6

    ADD r0 r7
    ADD r1 r7
    ADD r2 r7
    ADD r3 r7

    ADD r6 r6
    ADD r6 r6

    ADD r2 r6
    FLAG

    HLT
";

    let mut mem = [0u8; MEM_SIZE];
    mem[0x2002] = 0xff;
    mem[0x2004] = 0xff;

    const HALF: u16 = 0x800;

    let mut addr = 0x3000;
    let mut idx = 0;

    let mut circuit_base = Vec::new();

    let assembly = assemble(prog2)?;
    let writes: Vec<(u16, u16)> = assembly.iter().enumerate()
        .map(|(i, &value)| (i as u16 + 0x5c / 2 - 8, value))
        .collect();

    for (a, b) in writes {
        let base = 12 * idx + HALF;
        circuit_base.push((base + 6, base + 7, base + 5));
        circuit_base.push((base + 9, base + 10, 1));
        circuit_base.push((0xfff, 0xfff & !(a), 1));
        circuit_base.push((0xfff, 0xfff & !(b), 1));

        idx += 1;
    }

    for (a, b, c) in circuit_base {
        mem[addr] = (a & 0xFF) as u8;
        mem[addr + 1] = ((a >> 8) & 0xFF) as u8;
        mem[addr + 2] = (b & 0xFF) as u8;
        mem[addr + 3] = ((b >> 8) & 0xFF) as u8;
        mem[addr + 4] = (c & 0xFF) as u8;
        mem[addr + 5] = ((c >> 8) & 0xFF) as u8;
        addr += 6; // Increment by 6 since we wrote 3 values with spacing of 2
    }

    run_program(&program, 500000, &mut mem)?;

    fn dump_memory(mem: &[u8], base_addr: usize, size: usize) {
        println!("Memory dump at 0x{:04x} (first 0x{:x} bytes):", base_addr, size);
        println!("--------------------------------");
        for i in 0..size {
            if i % 16 == 0 {
                print!("{:04x}:", base_addr + i);
            }
            print!(" {:02x}", mem[base_addr + i]);
            if i % 16 == 15 {
                println!();
            }
        }
        println!();
    }
    
    println!("Assembly bytes:");
    for (i, byte) in assembly.iter().enumerate() {
        print!("{:04x}", byte);
        print!(" ");
    }
    println!();

    dump_memory(&mem, 0x2000, 0x20);
    dump_memory(&mem, 0x3000, 0x30);
    dump_memory(&mem, 0x0, 0x80);

    Ok(())
}

