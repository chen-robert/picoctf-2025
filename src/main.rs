use std::error::Error;
use verilog_ctf::simulator::{run_program, MEM_SIZE};

mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let program = "\
    LOADW r4 0x3000 ; circuit base
    LOADW r5 0x1000 ; expected output base
    LOADW r6 0x2000 ; circuit state

    LOADI r0 0
    ADD r0 r4
    LOADW r2 0x1000

check_start:
    LOAD r1 r0
    ADDI r0 2

    JZ r1 start

    GT r1 r2 r1
    JZ r1 end

    LOADI r1 0
    JZ r1 check_start

start:
    LOAD r0 r4
    ADDI r4 2
    LOAD r1 r4
    ADDI r4 2
    LOAD r2 r4
    ADDI r4 2

    ; if (r0 == 0 || r1 == 0 || r2 == 0) jmp end

    JZ r0 end
    JZ r1 end
    JZ r2 end

    ; double them
    ADD r0 r0
    ADD r1 r1
    ADD r2 r2

    ; mem[r2] = nand(mem[r0], mem[r1])
    ADD r0 r6
    ADD r1 r6
    ADD r2 r6

    LOAD r0 r0 
    LOAD r1 r1 

    NAND r0 r1

    STORE r2 r0
    
    LOADI r7 0 
    JZ r7 start

end:
    HLT
    ";

    let mut mem = [0u8; MEM_SIZE];
    mem[0x2002] = 0xff;
    mem[0x2004] = 0xff;

    const HALF: u16 = 0x800;

    let mut addr = 0x3000;
    let circuit_base: &[(u16, u16, u16)] = &[
        (HALF + 6, HALF + 7, HALF + 5),
        (HALF + 9, HALF + 10, 0), // will be modified
        (0xfff, 0xfff & !(1), 0),
        (0xfff, 0xfff & !(0b1001), 0)
    ];

    for (a, b, c) in circuit_base {
        mem[addr] = (a & 0xFF) as u8;
        mem[addr + 1] = ((a >> 8) & 0xFF) as u8;
        mem[addr + 2] = (b & 0xFF) as u8;
        mem[addr + 3] = ((b >> 8) & 0xFF) as u8;
        mem[addr + 4] = (c & 0xFF) as u8;
        mem[addr + 5] = ((c >> 8) & 0xFF) as u8;
        addr += 6; // Increment by 6 since we wrote 3 values with spacing of 2
    }

    run_program(program, 2000, &mut mem)?;

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

    dump_memory(&mem, 0x2000, 0x20);
    dump_memory(&mem, 0x3000, 0x20);
    dump_memory(&mem, 0x0, 0x20);

    Ok(())
}

