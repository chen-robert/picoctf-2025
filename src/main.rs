use std::error::Error;
use verilog_ctf::simulator::run_program;

mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let program = "\
        ; Example program
        LOADI r0 10     ; R0 = 10
        LOADI r1 5      ; R1 = 5
        ADD r0 r1        ; R0 = R0 + R1 (15)
        LOADI r2 0      ; R2 = 0 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
        SUB r0 r1        ; R0 = R0 - R1 (10)
        LOADI r2 1      ; R2 = 1 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
        MUL r0 r1        ; R0 = R0 * R1 (50)
        LOADI r2 2      ; R2 = 2 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
    ";

    run_program(program, 100)
}

