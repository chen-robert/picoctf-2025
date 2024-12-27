use std::error::Error;
use verilog_ctf::simulator::run_test_program;

#[test]
fn test_nop() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 42     ; Set initial value
        NOP           ; Should do nothing
        LOADI r1 0     ; Set address to 0
        STORE r1 r0     ; Store R0 to memory at address in R1
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),  // After LOADI
        (4, &[42, 0, 0, 0]),  // After NOP (unchanged)
        (6, &[42, 0, 0, 0]),  // After second LOADI
        (8, &[42, 0, 0, 0]),  // After STORE (unchanged)
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_add() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 10     ; R0 = 10
        LOADI r1 5      ; R1 = 5
        ADD r0 r1        ; R0 = R0 + R1 (15)
        LOADI r2 0      ; R2 = 0 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[10, 0, 0, 0]),   // After first LOADI
        (4, &[10, 5, 0, 0]),   // After second LOADI
        (6, &[15, 5, 0, 0]),   // After ADD
        (8, &[15, 5, 0, 0]),   // After third LOADI
        (10, &[15, 5, 0, 0]),  // After STORE
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_sub() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 20     ; R0 = 20
        LOADI r1 8      ; R1 = 8
        SUB r0 r1        ; R0 = R0 - R1 (12)
        LOADI r2 0      ; R2 = 0 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[20, 0, 0, 0]),   // After first LOADI
        (4, &[20, 8, 0, 0]),   // After second LOADI
        (6, &[12, 8, 0, 0]),   // After SUB
        (8, &[12, 8, 0, 0]),   // After STORE
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_mul() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 6      ; R0 = 6
        LOADI r1 7      ; R1 = 7
        MUL r0 r1        ; R0 = R0 * R1 (42)
        LOADI r2 0      ; R2 = 0 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[6, 0, 0, 0]),    // After first LOADI
        (4, &[6, 7, 0, 0]),    // After second LOADI
        (6, &[42, 7, 0, 0]),   // After MUL
        (8, &[42, 7, 0, 0]),   // After third LOADI
        (10, &[42, 7, 0, 0]),  // After STORE
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_loadi() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 123    ; R0 = 123
        LOADI r1 45     ; R1 = 45
        LOADI r2 67     ; R2 = 67
        LOADI r3 89     ; R3 = 89
    ";

    let expected_states = [
        (2, &[123, 0, 0, 0]),     // After first LOADI
        (4, &[123, 45, 0, 0]),    // After second LOADI
        (6, &[123, 45, 67, 0]),   // After third LOADI
        (8, &[123, 45, 67, 89]),  // After fourth LOADI
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_store() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 42     ; R0 = 42
        LOADI r2 0      ; R2 = 0 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
        LOADI r1 123    ; R1 = 123
        LOADI r3 1      ; R3 = 1 (memory address)
        STORE r3 r1      ; Store R1 to memory at address in R3
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),     // After first LOADI
        (4, &[42, 0, 0, 0]),     // After second LOADI
        (6, &[42, 0, 0, 0]),     // After first STORE
        (8, &[42, 123, 0, 0]),   // After third LOADI
        (10, &[42, 123, 0, 1]),  // After fourth LOADI
        (12, &[42, 123, 0, 1]),  // After second STORE
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_invalid() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 42     ; R0 = 42
        INVALID        ; Should do nothing
        LOADI r1 0      ; R1 = 0 (memory address)
        STORE r1 r0      ; Store R0 to memory at address in R1
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),  // After LOADI
        (4, &[42, 0, 0, 0]),  // After INVALID (unchanged)
        (6, &[42, 0, 0, 0]),  // After second LOADI
        (8, &[42, 0, 0, 0]),  // After STORE (unchanged)
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_load() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Load value from memory address 0 (which contains the first instruction)
        LOADI r1 0      ; R1 = 0 (memory address)
        LOAD r0 r1       ; R0 = mem[R1] (should be first instruction)
        LOADI r2 0      ; R2 = 0 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
    ";

    let expected_states = [
        (2, &[0, 0, 0, 0]),    // After LOADI
        (4, &[0x18, 0, 0, 0]), // After LOAD (loaded first instruction)
        (6, &[0x18, 0, 0, 0]), // After STORE
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_memory_write() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 42     ; R0 = 42 (value to write)
        LOADI r1 100    ; R1 = 100 (memory address)
        STORE r1 r0      ; Store R0 to memory at address in R1
        LOAD r2 r1       ; R2 = mem[R1] (read back value)
        LOADI r3 0      ; R3 = 0 (memory address)
        STORE r3 r2      ; Store R2 to memory at address in R3
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),     // After first LOADI
        (4, &[42, 100, 0, 0]),   // After second LOADI
        (6, &[42, 100, 0, 0]),   // After STORE
        (8, &[42, 100, 42, 0]),  // After LOAD (should read back 42)
        (10, &[42, 100, 42, 0]), // After third LOADI
        (12, &[42, 100, 42, 0]), // After second STORE
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_arithmetic_edge_cases() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Test max u8 + max u8 (255 + 255 = 510)
        LOADI r0 255    ; R0 = 255
        LOADI r1 255    ; R1 = 255
        ADD r0 r1        ; R0 = 255 + 255 (510)
        LOADI r2 0      ; R2 = 0 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
        ; Test multiplication overflow (16 * 16 = 256)
        LOADI r0 16     ; R0 = 16
        LOADI r1 16     ; R1 = 16
        MUL r0 r1        ; R0 = 16 * 16 (256)
        LOADI r2 1      ; R2 = 1 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
        ; Test subtraction (0 - 1 = 65535 in 16-bit)
        LOADI r0 0      ; R0 = 0
        LOADI r1 1      ; R1 = 1
        SUB r0 r1        ; R0 = 0 - 1 (65535)
        LOADI r2 2      ; R2 = 2 (memory address)
        STORE r2 r0      ; Store R0 to memory at address in R2
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

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_memory_edge_cases() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Test storing and loading from address 0
        LOADI r0 42     ; R0 = 42
        LOADI r1 0      ; R1 = 0 (address)
        STORE r1 r0      ; Store 42 at address 0
        LOAD r2 r1       ; Load from address 0 into R2
        ; Test storing and loading from max u8 address (255)
        LOADI r1 255    ; R1 = 255 (max address)
        LOADI r0 123    ; R0 = 123
        STORE r1 r0      ; Store 123 at address 255
        LOAD r3 r1       ; Load from address 255 into R3
    ";

    let expected_states = [
        (2, &[42, 0, 0, 0]),     // After first LOADI
        (4, &[42, 0, 0, 0]),     // After second LOADI
        (6, &[42, 0, 0, 0]),     // After first STORE
        (8, &[42, 0, 42, 0]),    // After first LOAD
        (10, &[42, 255, 42, 0]), // After third LOADI
        (12, &[123, 255, 42, 0]), // After fourth LOADI
        (14, &[123, 255, 42, 0]), // After second STORE
        (16, &[123, 255, 42, 123]), // After second LOAD
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_data_section() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; First load data from address 0x10
        LOADI r1 0x10    ; R1 = 0x10 (address)
        LOAD r0 r1        ; R0 = mem[R1]
        
        .data 0x10
        0xAB            ; Data at address 0x10
        
        .text
        ; Store the loaded value to address 0
        LOADI r2 0       ; R2 = 0 (address)
        STORE r2 r0      ; Store loaded value to address 0
    ";

    let expected_states = [
        (2, &[0, 0x10, 0, 0]),    // After first LOADI
        (4, &[0xAB, 0x10, 0, 0]), // After LOAD
        (6, &[0xAB, 0x10, 0, 0]), // After second LOADI
        (8, &[0xAB, 0x10, 0, 0]), // After STORE
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_jz() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Test JZ when register is zero
        LOADI r0 0      ; R0 = 0
        JZ r0 4        ; Should jump to instruction at PC 8 (4 * 2)
        LOADI r1 42    ; Should be skipped
        LOADI r1 123   ; Should be executed (at PC 8)
        
        ; Test JZ when register is non-zero
        LOADI r0 1     ; R0 = 1
        JZ r0 4        ; Should not jump since R0 != 0
        LOADI r2 42    ; Should be executed
    ";

    let expected_states = [
        (2, &[0, 0, 0, 0]),      // After first LOADI r0 0
        (8, &[0, 0, 0, 0]),      // After JZ (jumped to PC 8, skipping LOADI r1 42)
        (10, &[1, 0, 0, 0]),     // After LOADI r0 1
        (12, &[1, 0, 0, 0]),     // After JZ (no jump since R0 = 1)
        (14, &[1, 0, 42, 0]),    // After LOADI r2 42
    ];

    run_test_program(test_program, 500, &expected_states)
}
