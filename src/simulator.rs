use std::error::Error;
use std::fs;
use serde_json::Value;
use crate::state::State;
use crate::assembler::assemble;

const MODULE_NAME: &str = "counter";

pub fn get_bits_from_json(json: &Value, signal_name: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let ports = json["modules"][MODULE_NAME]["netnames"]
        .as_object()
        .ok_or_else(|| err!("Expected cells to be an object"))?;

    let bits = ports[signal_name]["bits"].as_array()
        .ok_or_else(|| err!("Expected bits to be an array"))?;

    bits.iter()
        .map(|v| {
            if v.is_string() && v.as_str() == Some("0") {
                Ok(0)
            } else {
                let n = v.as_i64()
                    .ok_or_else(|| err!("Expected bit value to be an integer"))?;
                assert!(n != 0 && n != 1, "Unexpected 0 or 1 as non-string value");
                Ok(n as i32)
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn get_single_bit_from_json(json: &Value, signal_name: &str) -> Result<i32, Box<dyn Error>> {
    let bits = get_bits_from_json(json, signal_name)?;
    
    if bits.len() != 1 {
        return Err(err!("Expected single bit signal"));
    }

    Ok(bits[0])
}

pub fn run_test_program(program: &str, cycles: usize, expected_states: &[(usize, &[u16; 4])]) -> Result<(), Box<dyn Error>> {
    let instructions = assemble(program)?;
    
    let mut data = [0; 100000];
    let mut mem = [0u8; u16::MAX as usize + 1];
    let mut current_state_idx = 0;
    
    // Load program into memory
    for (i, inst) in instructions.iter().enumerate() {
        // Instructions are 16-bit, split into 2 bytes
        mem[i * 2] = *inst as u8;            // Lower byte
        mem[i * 2 + 1] = (*inst >> 8) as u8; // Upper byte
    }

    println!("Program loaded into memory:");
    for i in 0..instructions.len() * 2 {
        println!("mem[{}]: {:02x}", i, mem[i]);
    }
    println!();

    let mut state = State {
        data: &mut data,
        updates: 0
    };

    let file_path = "./verilog/output.json";
    let json_content = fs::read_to_string(file_path)
        .map_err(|_| err!("Failed to read JSON file"))?;
    let json: Value = serde_json::from_str(&json_content)
        .map_err(|_| err!("Failed to parse JSON"))?;

    let clk = get_single_bit_from_json(&json, "clock")?;
    let state_bits = get_bits_from_json(&json, "state")?;
    let addr_bits = get_bits_from_json(&json, "addr")?;
    let inp_val_bits = get_bits_from_json(&json, "inp_val")?;
    let out_val_bits = get_bits_from_json(&json, "out_val")?;
    let program_counter_bits = get_bits_from_json(&json, "program_counter")?;
    let rst_bit = get_single_bit_from_json(&json, "reset")?;
    let opcode_bits = get_bits_from_json(&json, "opcode")?;
    let write_enable_bit = get_single_bit_from_json(&json, "write_enable")?;

    // Get register bits
    let registers = [
        get_bits_from_json(&json, "registers[0]")?,
        get_bits_from_json(&json, "registers[1]")?,
        get_bits_from_json(&json, "registers[2]")?,
        get_bits_from_json(&json, "registers[3]")?,
    ];

    // Reset sequence
    state.tick()?;
    state.flip(rst_bit)?;
    state.tick()?;
    state.flip(rst_bit)?;
    state.tick()?;

    println!("Starting program execution:");
    println!("---------------------------");

    for _ in 0..cycles {
        state.flip(clk)?;
        state.tick()?;
            
        let current_state = state.get(state_bits.iter())?;
        let program_counter = state.get(program_counter_bits.iter())?;
        let out_val = state.get(out_val_bits.iter())?;
            
        // Get current register values
        let mut reg_values = [0u16; 4];
        for (i, reg_bits) in registers.iter().enumerate() {
            reg_values[i] = state.get(reg_bits.iter())? as u16;
        }

        // Only check state when clock is high
        if state.data[clk as usize] == 255 {
            println!("PC: {:<3} State: {:<4b} Out: {:<5} Registers: {:?}", 
                program_counter, 
                current_state,
                out_val,
                reg_values
            );

        }
            
        // Check if we need to verify state at this point
        if current_state_idx < expected_states.len() && current_state == 0 && state.data[clk as usize] == 0 {
            let (expected_pc, expected_regs) = expected_states[current_state_idx];

            assert!(program_counter == expected_pc as u64, "Program counter mismatch");

            // Assert registers match expected
            for (i, (&expected, &actual)) in expected_regs.iter().zip(reg_values.iter()).enumerate() {
                assert_eq!(expected, actual, 
                    "Register mismatch at PC {}: R{} expected {}, got {}", 
                    program_counter, i, expected, actual);
            }
            current_state_idx += 1;
        }

        let addr = state.get(addr_bits.iter())?;
        assert_eq!(inp_val_bits.len(), 16, "inp_val_bits must be 16 bits (2 bytes)");
        let (first_byte, second_byte) = inp_val_bits.split_at(8);

        // Get opcode from instruction
        let opcode = state.get(opcode_bits.iter())?;

        // Handle memory writes for STORE instruction
        let write_enable = state.data[write_enable_bit as usize] == 255;
        if write_enable {
            let out_val = state.get(out_val_bits.iter())?;
            let addr = state.get(addr_bits.iter())?;
            println!("Memory write: addr={}, out_val={}", addr, out_val);
            // Store in little-endian order (lower byte first)
            mem[addr as usize] = (out_val & 0xFF) as u8;
            mem[addr as usize + 1] = ((out_val >> 8) & 0xFF) as u8;
        }


        // Always update inp_val with current memory value
        let addr = state.get(addr_bits.iter())?;
        // Read in little-endian order (lower byte first)
        let low_byte = mem[addr as usize];
        let high_byte = mem[addr as usize + 1];
        let value = ((high_byte as u16) << 8) | (low_byte as u16);
        //println!("Memory read: addr={}, value={}", addr, value);
        state.set(first_byte.iter(), low_byte)?;
        state.set(second_byte.iter(), high_byte)?;
    }

    Ok(())
}

pub fn run_program(program: &str, cycles: usize) -> Result<(), Box<dyn Error>> {
    println!("Running program:");
    println!("---------------");
    println!("{}", program);
    println!();
    
    // Run with empty expected states since we're not testing
    run_test_program(program, cycles, &[])
} 