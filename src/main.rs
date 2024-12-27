use std::error::Error;
use verilog_ctf::simulator::run_program;

mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let program = "\
        ; Example program
        LOADI 0 10     ; R0 = 10
        LOADI 1 5      ; R1 = 5
        ADD 0 1        ; R0 = R0 + R1 (15)
        LOADI 2 0      ; R2 = 0 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
        SUB 0 1        ; R0 = R0 - R1 (10)
        LOADI 2 1      ; R2 = 1 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
        MUL 0 1        ; R0 = R0 * R1 (50)
        LOADI 2 2      ; R2 = 2 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
    ";

    run_program(program, 100)
}

