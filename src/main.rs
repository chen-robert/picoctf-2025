use std::error::Error;
use verilog_ctf::simulator::{run_program, MEM_SIZE};

mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let program = "\
        ; Example program
        LOADW r4 0x3000 ; circuit base
        LOADW r5 0x1000 ; expected output base
        LOADW r6 0x2000 ; circuit state

start:
        LOAD r0 r4
        ADDI r4 2
        LOAD r1 r4
        ADDI r4 2
        LOAD r2 r4
        ADDI r4 2

        ; if (r0 == 0 && r1 == 0) jmp end
        LOADI r7 0 
        JZ r0 nxt

        JZ r7 inner
nxt:
        JZ r1 end

inner:
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
    ";

    let mut mem = [0u8; MEM_SIZE];
    mem[0x2000] = 1;
    mem[0x2002] = 3;

    let mut addr = 0x3000;
    let circuit_base = [(0, 2, 5), (5, 2, 1)];
    for (a, b, c) in circuit_base {
        mem[addr] = a;
        mem[addr + 2] = b; 
        mem[addr + 4] = c;
        addr += 6; // Increment by 6 since we wrote 3 values with spacing of 2
    }

    run_program(program, 2000, &mut mem)?;

    println!("Circuit state (first 0x20 bytes):");
    println!("--------------------------------");
    for i in 0..0x20 {
        if i % 16 == 0 {
            print!("{:04x}:", 0x2000 + i);
        }
        print!(" {:02x}", mem[0x2000 + i]);
        if i % 16 == 15 {
            println!();
        }
    }
    println!();

    Ok(())
}

