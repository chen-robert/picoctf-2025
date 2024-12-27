use std::error::Error;
use verilog_ctf::simulator::run_program;

mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let program = "\
        ; Example program
        LOADW r4 0x1000 ; circuit base
        LOADW r5 0x2000 ; expected output base
        LOADW r6 0x3000 ; circuit state

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

        NAND r0 r1 ; TODO implement
        STORE r2 r0
        
        LOADI r7 0 
        JZ r0 start

end:


    ";

    run_program(program, 100)
}

