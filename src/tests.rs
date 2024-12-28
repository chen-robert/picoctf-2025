use std::error::Error;
use verilog_ctf::simulator::{run_test_program, run_test_program_with_memory};

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
        LOADW r1 0x100  ; R1 = 0x100 (memory address)
        STORE r1 r0     ; This store should execute (42 -> 0x100)
        HLT            ; Should halt the processor
        LOADI r0 99    ; Should not execute
        STORE r1 r0     ; Should not execute this store
    ";

    run_test_program_with_memory(test_program, 500, &[
        (0x100, 42)    // Memory at address 0x100 should be 42 from the first store
    ])
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
    ";

    let expected_states = [
        (2, &[255, 0, 0, 0]),    // After first LOADI
        (4, &[255, 255, 0, 0]),  // After second LOADI
        (6, &[510, 255, 0, 0]),  // After ADD
        (8, &[510, 255, 0, 0]),  // After third LOADI
        (10, &[510, 255, 0, 0]), // After first STORE
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
        JZ r0 8        ; Should jump to instruction at PC 8 (4 * 2)
        LOADI r1 42    ; Should be skipped
        LOADI r1 123   ; Should be executed (at PC 8)
        
        ; Test JZ when register is non-zero
        LOADI r0 1     ; R0 = 1
        JZ r0 8        ; Should not jump since R0 != 0
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

#[test]
fn test_labels() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Test forward and backward jumps with labels
        LOADI r0 0      ; R0 = 0
        JZ r0 skip      ; Should jump to skip label
        LOADI r1 42     ; Should be skipped
skip:   
        LOADI r1 123    ; Should be executed
        LOADI r0 1      ; R0 = 1
        JZ r0 skip      ; Should not jump since R0 != 0
        LOADI r2 skip     ; Should be executed
    ";

    let expected_states = [
        (2, &[0, 0, 0, 0]),      // After first LOADI r0 0
        (6, &[0, 0, 0, 0]),      // After JZ (jumped to skip label)
        (8, &[0, 123, 0, 0]),   // After LOADI r1 123
        (10, &[1, 123, 0, 0]),   // After LOADI r0 1
        (12, &[1, 123, 0, 0]),   // After JZ (no jump since R0 = 1)
        (14, &[1, 123, 6, 0]),  // After LOADI r2 42
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_fibonacci() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Initialize registers
        LOADI r0 0      ; First number (0)
        LOADI r1 1      ; Second number (1)
        LOADI r2 0x80   ; Base address

        STORE r2 r0

        LOADI r3 2
        ADD r2 r3

        STORE r2 r1

start:
        ADD r1 r0
        LOAD r0 r2

        LOADI r3 2
        ADD r2 r3

        STORE r2 r1

        LOADI r3 0xc0
        GT r3 r3 r2

        JZ r3 end

        LOADI r3 0
        JZ r3 start
end:
    ";

    // Calculate first 32 Fibonacci numbers mod 65536
    let mut expected_memory = Vec::new();
    let mut a: u16 = 0;  // F(0)
    let mut b: u16 = 1;  // F(1)
    
    for i in 0..32 {
        // Each 16-bit value needs to be stored as two 8-bit values in little-endian order
        let addr = 0x80 + i * 2;  // Each number takes 2 bytes
        expected_memory.push((addr, (a & 0xFF) as u8));  // Lower byte
        expected_memory.push((addr + 1, ((a >> 8) & 0xFF) as u8));  // Upper byte
        let c = a.wrapping_add(b);  // Next Fibonacci number mod 65536
        a = b;
        b = c;
    }

    run_test_program_with_memory(test_program, 5000, &expected_memory)
}

#[test]
fn test_memory_high_regs() -> Result<(), Box<dyn Error>> {
    let program = "\
        LOADI r4 42     ;Load value 42 into r4
        LOADI r5 80     ;Load address 80 into r5
        STORE r5 r4     ;Store r4's value at address in r5
        LOADI r6 0      ;Clear r6
        LOAD r6 r5      ;Load value from address in r5 into r6
        LOADI r7 81     ;Load address 81 into r7
        STORE r7 r6     ;Store r6's value at address in r7
    ";

    run_test_program_with_memory(program, 100, &[
        (80, 42),   // First store
        (81, 42)    // Second store
    ])
}

#[test]
fn test_loadw() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Test loading 16-bit values
        LOADW r0 0x1234   ; Load 0x1234 into r0
        LOADW r1 1000     ; Load decimal 1000 into r1
        LOADW r2 0xFFFF   ; Load max value into r2
        LOADI r3 0        ; Set up memory address
        STORE r3 r0       ; Store r0 to verify value
    ";

    let expected_states = [
        (4, &[0x1234, 0, 0, 0]),      // After first LOADW
        (8, &[0x1234, 1000, 0, 0]),   // After second LOADW
        (12, &[0x1234, 1000, 0xFFFF, 0]), // After third LOADW
        (14, &[0x1234, 1000, 0xFFFF, 0]), // After LOADI
        (16, &[0x1234, 1000, 0xFFFF, 0]), // After STORE
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_addi() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 5      ; r0 = 5
        ADDI r0 3       ; r0 = 8
        LOADI r1 10     ; r1 = 10
        ADDI r1 255     ; r1 = 265
    ";

    let expected_states = [
        (2, &[5, 0, 0, 0]),    // After first LOADI
        (4, &[8, 0, 0, 0]),    // After ADDI
        (6, &[8, 10, 0, 0]),   // After second LOADI
        (8, &[8, 265, 0, 0]),  // After second ADDI (no overflow since 16-bit)
    ];

    run_test_program(test_program, 100, &expected_states)
}

#[test]
fn test_nand() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADW r0 0xFF00  ; r0 = 0xFF00
        LOADW r1 0xF0F0  ; r1 = 0xF0F0
        NAND r0 r1       ; r0 = ~(0xFF00 & 0xF0F0) = ~0xF000 = 0x0FFF
    ";

    let expected_states = [
        (4, &[0xFF00, 0, 0, 0]),     // After first LOADW
        (8, &[0xFF00, 0xF0F0, 0, 0]), // After second LOADW
        (10, &[0x0FFF, 0xF0F0, 0, 0]), // After NAND
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_gt() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        ; Test GT when r0 > r1 (should store 1)
        LOADI r0 10     ; r0 = 10
        LOADI r1 5      ; r1 = 5
        GT r2 r0 r1     ; r2 = (r0 > r1) ? 1 : 0
        
        ; Test GT when r0 <= r1 (should store 0)
        LOADI r0 5      ; r0 = 5
        LOADI r1 10     ; r1 = 10
        GT r2 r0 r1     ; r3 = (r0 > r1) ? 1 : 0
        
        ; Test GT when r0 = r1 (should store 0)
        LOADI r0 5      ; r0 = 5
        LOADI r1 5      ; r1 = 5
        GT r3 r0 r1     ; r4 = (r0 > r1) ? 1 : 0
    ";

    let expected_states = [
        (2, &[10, 0, 0, 0]),    // After first LOADI r0
        (4, &[10, 5, 0, 0]),    // After first LOADI r1
        (6, &[10, 5, 1, 0]),    // After first GT (10 > 5 = true)
        (8, &[5, 5, 1, 0]),     // After second LOADI r0
        (10, &[5, 10, 1, 0]),   // After second LOADI r1
        (12, &[5, 10, 0, 0]),   // After second GT (5 > 10 = false)
        (14, &[5, 10, 0, 0]),    // After third LOADI r0
        (16, &[5, 5, 0, 0]),    // After third LOADI r1
        (18, &[5, 5, 0, 0]),    // After third GT (5 > 5 = false)
    ];

    run_test_program(test_program, 500, &expected_states)
}

#[test]
fn test_flag() -> Result<(), Box<dyn Error>> {
    let test_program = "\
        LOADI r0 42     ; R0 = 42
        LOADW r1 0x100  ; R1 = 0x100 (memory address)
        FLAG           ; Set the flag
        STORE r1 r0     ; Store 42 to memory
        HLT            ; Stop execution
    ";

    run_test_program_with_memory(test_program, 500, &[
        (0x100, 42)    // Memory at address 0x100 should be 42
    ])
}
