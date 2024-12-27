use crate::err;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Add { dest: u8, src: u8 },
    Sub { dest: u8, src: u8 },
    Mul { dest: u8, src: u8 },
    LoadI { dest: u8, imm: u8 },
    Store { addr: u8, src: u8 },
    Load { dest: u8, src: u8 },
    Jz { reg: u8, addr: u8 },
    Invalid,
}

fn parse_register(reg_str: &str) -> Result<u8, Box<dyn Error>> {
    let reg_str = reg_str.to_uppercase();
    if !reg_str.starts_with('R') {
        return Err(err!("Register must start with 'r' or 'R'"));
    }
    let num_str = &reg_str[1..];
    let reg_num = num_str.parse::<u8>()?;
    if reg_num >= 4 {
        return Err(err!("Register index must be 0-3"));
    }
    Ok(reg_num)
}

fn parse_immediate(imm_str: &str) -> Result<u8, Box<dyn Error>> {
    if imm_str.starts_with("0x") {
        u8::from_str_radix(imm_str.trim_start_matches("0x"), 16)
    } else {
        imm_str.parse::<u8>()
    }.map_err(|_| err!("Invalid immediate value"))
}

fn parse_data_address(addr_str: &str) -> Result<usize, Box<dyn Error>> {
    usize::from_str_radix(addr_str.trim_start_matches("0x"), 16)
        .map_err(|_| err!("Invalid hex address"))
}

fn parse_data_value(value_str: &str) -> Result<u16, Box<dyn Error>> {
    u16::from_str_radix(value_str.trim_start_matches("0x"), 16)
        .map_err(|_| err!("Invalid hex data"))
}

fn parse_instruction(parts: &[&str]) -> Result<Instruction, Box<dyn Error>> {
    match parts[0].to_uppercase().as_str() {
        "NOP" => {
            assert!(parts.len() == 1, "NOP takes no arguments");
            Ok(Instruction::Nop)
        }
        "ADD" | "SUB" | "MUL" | "LOAD" => {
            assert!(parts.len() == 3, "{} requires 2 register arguments", parts[0].to_uppercase());
            let dest = parse_register(parts[1])?;
            let src = parse_register(parts[2])?;
            match parts[0].to_uppercase().as_str() {
                "ADD" => Ok(Instruction::Add { dest, src }),
                "SUB" => Ok(Instruction::Sub { dest, src }),
                "MUL" => Ok(Instruction::Mul { dest, src }),
                "LOAD" => Ok(Instruction::Load { dest, src }),
                _ => unreachable!()
            }
        }
        "LOADI" => {
            assert!(parts.len() == 3, "LOADI requires a register and immediate value");
            let dest = parse_register(parts[1])?;
            let imm = parse_immediate(parts[2])?;
            Ok(Instruction::LoadI { dest, imm })
        }
        "STORE" => {
            assert!(parts.len() == 3, "STORE requires a register for address and a register for value");
            let addr = parse_register(parts[1])?;
            let src = parse_register(parts[2])?;
            Ok(Instruction::Store { addr, src })
        }
        "JZ" => {
            assert!(parts.len() == 3, "JZ requires a register and an address");
            let reg = parse_register(parts[1])?;
            let addr = parse_immediate(parts[2])?;
            Ok(Instruction::Jz { reg, addr })
        }
        "INVALID" => {
            assert!(parts.len() == 1, "INVALID takes no arguments");
            Ok(Instruction::Invalid)
        }
        _ => Err(err!("Unknown instruction"))
    }
}

fn encode_instruction(inst: Instruction) -> u16 {
    let encoded = match inst {
        Instruction::Nop => {
            0b0000u64                   // NOP opcode (0x0)
        }
        Instruction::Add { dest, src } => {
            ((0u64) << 8) |             // immediate
            ((src as u64) << 6) |       // reg_src
            ((dest as u64) << 4) |      // reg_dest
            (0b0001u64)                 // ADD opcode (0x1)
        }
        Instruction::Sub { dest, src } => {
            ((0u64) << 8) |             // immediate
            ((src as u64) << 6) |       // reg_src
            ((dest as u64) << 4) |      // reg_dest
            (0b0010u64)                 // SUB opcode (0x2)
        }
        Instruction::Mul { dest, src } => {
            ((0u64) << 8) |             // immediate
            ((src as u64) << 6) |       // reg_src
            ((dest as u64) << 4) |      // reg_dest
            (0b0011u64)                 // MUL opcode (0x3)
        }
        Instruction::LoadI { dest, imm } => {
            ((imm as u64) << 8) |       // immediate
            ((0u64) << 6) |             // reg_src (unused)
            ((dest as u64) << 4) |      // reg_dest
            (0b1000u64)                 // LOADI opcode (0x8)
        }
        Instruction::Store { addr, src } => {
            ((0u64) << 8) |             // immediate (unused)
            ((src as u64) << 6) |       // reg_src (value to store)
            ((addr as u64) << 4) |      // reg_dest (address)
            (0b1001u64)                 // STORE opcode (0x9)
        }
        Instruction::Load { dest, src } => {
            ((0u64) << 8) |             // immediate (unused)
            ((src as u64) << 6) |       // reg_src (address)
            ((dest as u64) << 4) |      // reg_dest
            (0b1011u64)                 // LOAD opcode (0xB)
        }
        Instruction::Jz { reg, addr } => {
            ((addr as u64) << 8) |      // immediate (jump address)
            ((reg as u64) << 6) |       // reg_src (register to check)
            ((0u64) << 4) |             // reg_dest (unused)
            (0b1100u64)                 // JZ opcode (0xC)
        }
        Instruction::Invalid => {
            0b1111u64                   // INVALID opcode (0xF)
        }
    };
    encoded as u16
}

fn check_data_overlap(instruction_range: std::ops::Range<usize>, data_sections: &HashMap<usize, u16>) -> Result<(), Box<dyn Error>> {
    for (addr, _) in data_sections {
        if instruction_range.contains(addr) {
            let msg = format!("Data section at address 0x{:x} overlaps with instructions", addr);
            return Err(err!(msg));
        }
    }
    Ok(())
}

fn merge_instructions_and_data(instructions: Vec<u16>, data_sections: HashMap<usize, u16>) -> Result<Vec<u16>, Box<dyn Error>> {
    let mut final_memory = vec![0u16; instructions.len()];
    final_memory.copy_from_slice(&instructions);

    for (addr, value) in data_sections {
        if addr % 2 != 0 {
            return Err(err!("Data must be aligned to 2-byte boundaries"));
        }
        let idx = addr / 2;
        if idx >= final_memory.len() {
            final_memory.resize(idx + 1, 0);
        }
        final_memory[idx] = value;
    }

    Ok(final_memory)
}

pub fn assemble(program: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    let mut instructions = Vec::new();
    let mut data_sections: HashMap<usize, u16> = HashMap::new();
    let mut in_data_section = false;
    let mut current_data_addr = 0;

    for line in program.lines() {
        // Remove comments
        let line = match line.find(';') {
            Some(idx) => &line[..idx],
            None => line,
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Check for data section directive
        if line.starts_with(".data") {
            in_data_section = true;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(err!(".data directive requires an address"));
            }
            current_data_addr = parse_data_address(parts[1])?;
            continue;
        }

        if line == ".text" {
            in_data_section = false;
            continue;
        }

        if in_data_section {
            let value = parse_data_value(line)?;
            data_sections.insert(current_data_addr, value);
            current_data_addr += 2; // Each data value is 2 bytes
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let inst = parse_instruction(&parts)?;
        let encoded = encode_instruction(inst);
        instructions.push(encoded);
    }

    // Check for overlaps between instructions and data sections
    let instruction_range = 0..(instructions.len() * 2); // Each instruction is 2 bytes
    check_data_overlap(instruction_range, &data_sections)?;

    // Merge instructions and data sections
    merge_instructions_and_data(instructions, data_sections)
} 