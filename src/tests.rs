use std::error::Error;
use verilog_ctf::simulator::run_test_program;

#[test]
fn test_nop() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI 0 42     ; Set initial value
        NOP           ; Should do nothing
        LOADI 1 0     ; Set address to 0
        STORE 1 0     ; Store R0 to memory at address in R1
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),  // After LOADI
        (4, &[42, 0, 0, 0]),  // After NOP (unchanged)
        (6, &[42, 0, 0, 0]),  // After second LOADI
        (8, &[42, 0, 0, 0]),  // After STORE (unchanged)
    ];

    run_test_program(test_program, 30, &expected_states)
}

#[test]
fn test_add() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI 0 10     ; R0 = 10
        LOADI 1 5      ; R1 = 5
        ADD 0 1        ; R0 = R0 + R1 (15)
        LOADI 2 0      ; R2 = 0 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[10, 0, 0, 0]),   // After first LOADI
        (4, &[10, 5, 0, 0]),   // After second LOADI
        (6, &[15, 5, 0, 0]),   // After ADD
        (8, &[15, 5, 0, 0]),   // After third LOADI
        (10, &[15, 5, 0, 0]),  // After STORE
    ];

    run_test_program(test_program, 40, &expected_states)
}

#[test]
fn test_sub() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI 0 20     ; R0 = 20
        LOADI 1 8      ; R1 = 8
        SUB 0 1        ; R0 = R0 - R1 (12)
        LOADI 2 0      ; R2 = 0 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[20, 0, 0, 0]),   // After first LOADI
        (4, &[20, 8, 0, 0]),   // After second LOADI
        (6, &[12, 8, 0, 0]),   // After SUB
        (8, &[12, 8, 0, 0]),   // After STORE
    ];

    run_test_program(test_program, 40, &expected_states)
}

#[test]
fn test_mul() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI 0 6      ; R0 = 6
        LOADI 1 7      ; R1 = 7
        MUL 0 1        ; R0 = R0 * R1 (42)
        LOADI 2 0      ; R2 = 0 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[6, 0, 0, 0]),    // After first LOADI
        (4, &[6, 7, 0, 0]),    // After second LOADI
        (6, &[42, 7, 0, 0]),   // After MUL
        (8, &[42, 7, 0, 0]),   // After third LOADI
        (10, &[42, 7, 0, 0]),  // After STORE
    ];

    run_test_program(test_program, 40, &expected_states)
}

#[test]
fn test_loadi() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI 0 123    ; R0 = 123
        LOADI 1 45     ; R1 = 45
        LOADI 2 67     ; R2 = 67
        LOADI 3 89     ; R3 = 89
    ";

    let expected_states = [
        (2, &[123, 0, 0, 0]),     // After first LOADI
        (4, &[123, 45, 0, 0]),    // After second LOADI
        (6, &[123, 45, 67, 0]),   // After third LOADI
        (8, &[123, 45, 67, 89]),  // After fourth LOADI
    ];

    run_test_program(test_program, 40, &expected_states)
}

#[test]
fn test_store() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI 0 42     ; R0 = 42
        LOADI 2 0      ; R2 = 0 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
        LOADI 1 123    ; R1 = 123
        LOADI 3 1      ; R3 = 1 (memory address)
        STORE 3 1      ; Store R1 to memory at address in R3
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),     // After first LOADI
        (4, &[42, 0, 0, 0]),     // After second LOADI
        (6, &[42, 0, 0, 0]),     // After first STORE
        (8, &[42, 123, 0, 0]),   // After third LOADI
        (10, &[42, 123, 0, 1]),  // After fourth LOADI
        (12, &[42, 123, 0, 1]),  // After second STORE
    ];

    run_test_program(test_program, 40, &expected_states)
}

#[test]
fn test_invalid() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI 0 42     ; R0 = 42
        INVALID        ; Should do nothing
        LOADI 1 0      ; R1 = 0 (memory address)
        STORE 1 0      ; Store R0 to memory at address in R1
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),  // After LOADI
        (4, &[42, 0, 0, 0]),  // After INVALID (unchanged)
        (6, &[42, 0, 0, 0]),  // After second LOADI
        (8, &[42, 0, 0, 0]),  // After STORE (unchanged)
    ];

    run_test_program(test_program, 30, &expected_states)
}

#[test]
fn test_load() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Load value from memory address 0 (which contains the first instruction)
        LOADI 1 0      ; R1 = 0 (memory address)
        LOAD 0 1       ; R0 = mem[R1] (should be first instruction)
        LOADI 2 0      ; R2 = 0 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[0, 0, 0, 0]),    // After LOADI
        (4, &[0x18, 0, 0, 0]), // After LOAD (loaded first instruction)
        (6, &[0x18, 0, 0, 0]), // After STORE
    ];

    run_test_program(test_program, 20, &expected_states)
}

#[test]
fn test_memory_write() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI 0 42     ; R0 = 42 (value to write)
        LOADI 1 100    ; R1 = 100 (memory address)
        STORE 1 0      ; Store R0 to memory at address in R1
        LOAD 2 1       ; R2 = mem[R1] (read back value)
        LOADI 3 0      ; R3 = 0 (memory address)
        STORE 3 2      ; Store R2 to memory at address in R3
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),     // After first LOADI
        (4, &[42, 100, 0, 0]),   // After second LOADI
        (6, &[42, 100, 0, 0]),   // After STORE
        (8, &[42, 100, 42, 0]),  // After LOAD (should read back 42)
        (10, &[42, 100, 42, 0]), // After third LOADI
        (12, &[42, 100, 42, 0]), // After second STORE
    ];

    run_test_program(test_program, 50, &expected_states)
}

#[test]
fn test_arithmetic_edge_cases() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Test max u8 + max u8 (255 + 255 = 510)
        LOADI 0 255    ; R0 = 255
        LOADI 1 255    ; R1 = 255
        ADD 0 1        ; R0 = 255 + 255 (510)
        LOADI 2 0      ; R2 = 0 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
        ; Test multiplication overflow (16 * 16 = 256)
        LOADI 0 16     ; R0 = 16
        LOADI 1 16     ; R1 = 16
        MUL 0 1        ; R0 = 16 * 16 (256)
        LOADI 2 1      ; R2 = 1 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
        ; Test subtraction (0 - 1 = 65535 in 16-bit)
        LOADI 0 0      ; R0 = 0
        LOADI 1 1      ; R1 = 1
        SUB 0 1        ; R0 = 0 - 1 (65535)
        LOADI 2 2      ; R2 = 2 (memory address)
        STORE 2 0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[255, 0, 0, 0]),    // After first LOADI
        (4, &[255, 255, 0, 0]),  // After second LOADI
        (6, &[510, 255, 0, 0]),  // After ADD
        (8, &[510, 255, 0, 0]),  // After third LOADI
        (10, &[510, 255, 0, 0]), // After first STORE
        (12, &[16, 255, 0, 0]),  // After fourth LOADI
        (14, &[16, 16, 0, 0]),   // After fifth LOADI
        (16, &[256, 16, 0, 0]),  // After MUL
        (18, &[256, 16, 1, 0]),  // After sixth LOADI
        (20, &[256, 16, 1, 0]),  // After second STORE
        (22, &[0, 16, 1, 0]),    // After seventh LOADI
        (24, &[0, 1, 1, 0]),     // After eighth LOADI
        (26, &[65535, 1, 1, 0]), // After SUB (wrap around)
        (28, &[65535, 1, 2, 0]), // After ninth LOADI
        (30, &[65535, 1, 2, 0]), // After third STORE
    ];

    run_test_program(test_program, 100, &expected_states)
} 