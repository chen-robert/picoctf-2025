use std::error::Error;
use std::fs;
use serde_json::Value;
use crate::state::State;
use crate::assembler::assemble;

const MODULE_NAME: &str = "cpu";
pub const MEM_SIZE: usize = 65536;

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

pub fn run_test_program_with_expectations(
    program: &str,
    cycles: usize,
    expected_states: Option<&[(usize, &[i32; 4])]>,
    expected_memory: Option<&[(usize, u8)]>,
    mem: &mut [u8; MEM_SIZE],
) -> Result<(), Box<dyn Error>> {
    let instructions = assemble(program)?;
    
    let mut data = [0; 100000];
    let mut current_state_idx = 0;
    
    // Load program into memory
    for (i, inst) in instructions.iter().enumerate() {
        mem[i * 2] = u8::try_from(*inst & 0xFF).unwrap();
        mem[i * 2 + 1] = u8::try_from((*inst >> 8) & 0xFF).unwrap();
    }

    let mut state = State {
        data: &mut data,
        updates: 0,
        total_updates: 0,
    };

    let file_path = "./verilog/cpu.json";
    let json_content = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&json_content)?;

    let clk = get_single_bit_from_json(&json, "clock")?;
    let state_bits = get_bits_from_json(&json, "state")?;
    let addr_bits = get_bits_from_json(&json, "addr")?;
    let inp_val_bits = get_bits_from_json(&json, "inp_val")?;
    let out_val_bits = get_bits_from_json(&json, "out_val")?;
    let program_counter_bits = get_bits_from_json(&json, "program_counter")?;
    let rst_bit = get_single_bit_from_json(&json, "reset")?;
    let write_enable_bit = get_single_bit_from_json(&json, "write_enable")?;
    let halted_bit = get_single_bit_from_json(&json, "halted")?;
    let flag_bit = get_single_bit_from_json(&json, "flag")?;

    let registers = [
        get_bits_from_json(&json, "registers[0]")?,
        get_bits_from_json(&json, "registers[1]")?,
        get_bits_from_json(&json, "registers[2]")?,
        get_bits_from_json(&json, "registers[3]")?,
    ];

    state.tick()?;
    state.flip(rst_bit)?;
    state.tick()?;
    state.flip(rst_bit)?;
    state.tick()?;

    let mut first = true;

    for _ in 0..cycles {
        state.flip(clk)?;
        state.tick()?;
            
        let current_state = state.get(state_bits.iter())?;
        let program_counter = state.get(program_counter_bits.iter())?;
            
        let mut reg_values = [0i32; 4];
        for (i, reg_bits) in registers.iter().enumerate() {
            reg_values[i] = i32::try_from(state.get(reg_bits.iter())?).unwrap();
        }
        if current_state == 0 && state.data[usize::try_from(clk).unwrap()] == 0 {
            println!("PC: {:04x} | R0: {:04x} R1: {:04x} R2: {:04x} R3: {:04x}",
                program_counter,
                reg_values[0],
                reg_values[1],
                reg_values[2],
                reg_values[3]);
        }
            
        if let Some(states) = expected_states {
            if current_state_idx < states.len() && current_state == 1 && state.data[usize::try_from(clk).unwrap()] == 0 {
                if first {
                    first = false;
                } else {
                    let (expected_pc, expected_regs) = states[current_state_idx];
                    assert!(program_counter == u64::try_from(expected_pc).unwrap());
                    for (i, (&expected, &actual)) in expected_regs.iter().zip(reg_values.iter()).enumerate() {
                        assert_eq!(expected, actual, "Register {} mismatch at PC {}", i, expected_pc);
                    }
                    current_state_idx += 1;
                }
            }
        }

        let write_enable = state.data[usize::try_from(write_enable_bit).unwrap()] == 255;
        if write_enable {
            let out_val = state.get(out_val_bits.iter())?;
            let addr = state.get(addr_bits.iter())?;
            mem[usize::try_from(addr).unwrap()] = u8::try_from(out_val & 0xFF).unwrap();
            mem[usize::try_from(addr).unwrap() + 1] = u8::try_from((out_val >> 8) & 0xFF).unwrap();

            println!("Wrote to memory: {:04x} = {:04x}", addr, out_val);
        }

        let addr = state.get(addr_bits.iter())?;
        let (first_byte, second_byte) = inp_val_bits.split_at(8);
        let low_byte = mem[usize::try_from(addr).unwrap()];
        let high_byte = mem[usize::try_from(addr).unwrap() + 1];
        state.set(first_byte.iter(), low_byte)?;
        state.set(second_byte.iter(), high_byte)?;

        if state.data[usize::try_from(halted_bit).unwrap()] == 255 {
            println!("HALTED");
            break;
        }

        if state.data[usize::try_from(flag_bit).unwrap()] == 255 {
            println!("FLAG");
        }
    }

    if let Some(states) = expected_states {
        assert_eq!(current_state_idx, states.len());
    }

    if let Some(memory) = expected_memory {
        for &(addr, expected_val) in memory {
            assert_eq!(mem[addr], expected_val, "Memory mismatch at address {:#x}", addr);
        }
    }

    println!("Total updates: {}", state.total_updates);

    Ok(())
}

pub fn run_test_program(
    program: &str,
    cycles: usize,
    expected_states: &[(usize, &[i32; 4])],
) -> Result<(), Box<dyn Error>> {
    let mut mem = [0u8; MEM_SIZE];
    run_test_program_with_expectations(program, cycles, Some(expected_states), None, &mut mem)
}

pub fn run_test_program_with_memory(
    program: &str,
    cycles: usize,
    expected_memory: &[(usize, u8)],
) -> Result<(), Box<dyn Error>> {
    let mut mem = [0u8; MEM_SIZE];
    run_test_program_with_expectations(program, cycles, None, Some(expected_memory), &mut mem)
}

pub fn run_program(program: &str, cycles: usize, mem: &mut [u8; MEM_SIZE]) -> Result<(), Box<dyn Error>> {
    println!("Running program:");
    println!("---------------");
    println!("{}", program);
    println!();
    
    run_test_program_with_expectations(program, cycles, None, None, mem)
}
