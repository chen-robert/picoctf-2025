use crate::err;
use std::error::Error;
use std::collections::HashMap;
use std::convert::TryFrom;

type LabelMap = HashMap<String, usize>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    Add { dest: u8, src: u8 },
    AddI { dest: u8, imm: u8 },
    Nand { dest: u8, src: u8 },
    LoadI { dest: u8, imm: u8 },
    Store { addr: u8, src: u8 },
    Load { dest: u8, src: u8 },
    Jz { reg: u8, addr: u8 },
    LoadW { dest: u8, imm: u16 },
    Gt { dest: u8, src1: u8, src2: u8 },
    Flag,
    Invalid,
}

fn parse_register(reg_str: &str) -> Result<u8, Box<dyn Error>> {
    let reg_str = reg_str.to_uppercase();
    assert!(reg_str.starts_with('R'), "Register must start with 'r' or 'R': '{}'", reg_str);
    let num_str = &reg_str[1..];
    let reg_num = num_str.parse::<u8>()?;
    assert!(reg_num < 8, "Register index must be 0-15: '{}'", reg_str);
    Ok(reg_num)
}

fn parse_immediate(imm_str: &str, labels: &LabelMap) -> Result<u8, Box<dyn Error>> {
    // First check if it's a label
    if let Some(&addr) = labels.get(imm_str) {
        // Convert byte address to instruction count (each instruction is 2 bytes)
        // The jump target should be addr/2 since each instruction is 2 bytes
        return Ok(u8::try_from(addr).unwrap());
    }
    
    // Otherwise parse as number
    if imm_str.starts_with("0x") {
        u8::from_str_radix(imm_str.trim_start_matches("0x"), 16)
    } else {
        imm_str.parse::<u8>()
    }.map_err(|_| format!("Invalid immediate value: {}", imm_str).into())
}

fn parse_data_address(addr_str: &str) -> Result<usize, Box<dyn Error>> {
    usize::from_str_radix(addr_str.trim_start_matches("0x"), 16)
        .map_err(|_| err!("Invalid hex address"))
}

fn parse_data_value(value_str: &str) -> Result<u16, Box<dyn Error>> {
    u16::from_str_radix(value_str.trim_start_matches("0x"), 16)
        .map_err(|_| err!("Invalid hex data"))
}

fn parse_instruction(parts: &[&str], labels: &LabelMap) -> Result<Instruction, Box<dyn Error>> {
    match parts[0].to_uppercase().as_str() {
        "NOP" => {
            assert!(parts.len() == 1, "NOP takes no arguments");
            Ok(Instruction::Nop)
        }
        "ADD" | "LOAD" | "NAND" => {
            assert!(parts.len() == 3, "{} requires 2 register arguments", parts[0].to_uppercase());
            let dest = parse_register(parts[1])?;
            let src = parse_register(parts[2])?;
            match parts[0].to_uppercase().as_str() {
                "ADD" => Ok(Instruction::Add { dest, src }),
                "LOAD" => Ok(Instruction::Load { dest, src }),
                "NAND" => Ok(Instruction::Nand { dest, src }),
                _ => unreachable!()
            }
        }
        "ADDI" => {
            assert!(parts.len() == 3, "ADDI requires a register and immediate value");
            let dest = parse_register(parts[1])?;
            let imm = parse_immediate(parts[2], labels)?;
            Ok(Instruction::AddI { dest, imm })
        }
        "LOADI" => {
            assert!(parts.len() == 3, "LOADI requires a register and immediate value");
            let dest = parse_register(parts[1])?;
            let imm = parse_immediate(parts[2], labels)?;
            Ok(Instruction::LoadI { dest, imm })
        }
        "LOADW" => {
            assert!(parts.len() == 3, "LOADW requires a register and 16-bit immediate value");
            let dest = parse_register(parts[1])?;
            // Parse immediate as 16-bit value
            let imm_str = parts[2];
            let imm = if imm_str.starts_with("0x") {
                u16::from_str_radix(imm_str.trim_start_matches("0x"), 16)
            } else {
                imm_str.parse::<u16>()
            }.map_err(|_| format!("Invalid 16-bit immediate value: {}", imm_str))?;
            Ok(Instruction::LoadW { dest, imm })
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
            let addr = parse_immediate(parts[2], labels)?;
            Ok(Instruction::Jz { reg, addr })
        }
        "GT" => {
            assert!(parts.len() == 4, "GT requires 3 register arguments");
            let dest = parse_register(parts[1])?;
            let src1 = parse_register(parts[2])?;
            let src2 = parse_register(parts[3])?;
            Ok(Instruction::Gt { dest, src1, src2 })
        },
        "FLAG" => {
            assert!(parts.len() == 1, "FLAG takes no arguments");
            Ok(Instruction::Flag)
        },
        "HLT" => {
            assert!(parts.len() == 1, "HLT takes no arguments");
            Ok(Instruction::Invalid)
        }
        _ => Err(err!(format!("Unknown instruction: {}", parts[0])))
    }
}

fn encode_instruction(inst: Instruction) -> u16 {
    let encoded = match inst {
        Instruction::Nop => {
            0b0000u64                   // NOP opcode (0x0)
        }
        Instruction::Add { dest, src } => {
            ((0u64) << 8) |             // immediate (unused)
            ((src as u64) << 8) |       // reg_src (4 bits)
            ((dest as u64) << 4) |      // reg_dest (4 bits)
            (0b0001u64)                 // ADD opcode (0x1)
        }
        Instruction::AddI { dest, imm } => {
            ((imm as u64) << 8) |       // immediate
            ((0u64) << 8) |             // reg_src (unused)
            ((dest as u64) << 4) |      // reg_dest (4 bits)
            (0b0100u64)                 // ADDI opcode (0x4)
        }
        Instruction::Nand { dest, src } => {
            ((0u64) << 8) |             // immediate (unused)
            ((src as u64) << 8) |       // reg_src (4 bits)
            ((dest as u64) << 4) |      // reg_dest (4 bits)
            (0b0110u64)                 // NAND opcode (0x6)
        }
        Instruction::LoadI { dest, imm } => {
            ((imm as u64) << 8) |       // immediate
            ((0u64) << 8) |             // reg_src (unused)
            ((dest as u64) << 4) |      // reg_dest (4 bits)
            (0b1000u64)                 // LOADI opcode (0x8)
        }
        Instruction::Store { addr, src } => {
            ((0u64) << 8) |             // immediate (unused)
            ((src as u64) << 8) |       // reg_src (value to store)
            ((addr as u64) << 4) |      // reg_dest (address)
            (0b1001u64)                 // STORE opcode (0x9)
        }
        Instruction::Load { dest, src } => {
            ((0u64) << 8) |             // immediate (unused)
            ((src as u64) << 8) |       // reg_src (address)
            ((dest as u64) << 4) |      // reg_dest
            (0b1011u64)                 // LOAD opcode (0xB)
        }
        Instruction::Jz { reg, addr } => {
            ((addr as u64) << 8) |      // immediate (jump address)
            ((0u64) << 8) |             // reg_src (unused)
            ((reg as u64) << 4) |       // reg_dest (register to check)
            (0b1100u64)                 // JZ opcode (0xC)
        }
        Instruction::LoadW { dest, imm: _ } => {
            ((0u64) << 8) |             // immediate (unused in first word)
            ((0u64) << 8) |             // reg_src (unused)
            ((dest as u64) << 4) |      // reg_dest (4 bits)
            (0b1101u64)                 // LOADW opcode (0xD)
        }
        Instruction::Gt { dest, src1, src2 } => {
            ((src2 as u64) << 12) |      // reg_src2 (3 bits) [14:12]
            ((src1 as u64) << 8) |       // reg_src1 (3 bits) [10:8]
            ((dest as u64) << 4) |       // reg_dest (3 bits) [6:4]
            (0b0111u64)                  // GT opcode (0x7)
        },
        Instruction::Flag => {
            0b1110u64                   // FLAG opcode (0xE)
        },
        Instruction::Invalid => {
            0b1111u64                   // INVALID/HLT opcode (0xF)
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
    let mut labels: LabelMap = HashMap::new();
    let mut in_data_section = false;
    let mut current_data_addr = 0;
    let mut current_instruction_addr = 0;

    // First pass: collect labels
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
            current_data_addr += 2; // Each data value is 2 bytes
            continue;
        }

        // Check for label (ends with :)
        if line.ends_with(':') {
            let label = line[..line.len()-1].trim().to_string();
            labels.insert(label, current_instruction_addr);
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if !parts.is_empty() {
            // Account for LOADW taking 2 words
            if parts[0].to_uppercase() == "LOADW" {
                current_instruction_addr += 4;
            } else {
                current_instruction_addr += 2;
            }
        }
    }

    // Reset for second pass
    in_data_section = false;
    current_data_addr = 0;

    // Second pass: assemble instructions with label resolution
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

        // Skip label definitions in second pass
        if line.ends_with(':') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let inst = parse_instruction(&parts, &labels)?;
        let encoded = encode_instruction(inst.clone());
        instructions.push(encoded);
        
        // For LOADW, add the immediate value as a second word
        if let Instruction::LoadW { imm, .. } = inst {
            instructions.push(imm);
        }
    }

    // Check for overlaps between instructions and data sections
    let instruction_range = 0..(instructions.len() * 2); // Each instruction is 2 bytes
    check_data_overlap(instruction_range, &data_sections)?;

    // Merge instructions and data
    merge_instructions_and_data(instructions, data_sections)
} 