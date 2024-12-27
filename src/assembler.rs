use crate::err;
use std::error::Error;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Add { dest: u8, src: u8 },
    Sub { dest: u8, src: u8 },
    Mul { dest: u8, src: u8 },
    LoadI { dest: u8, imm: u8 },
    Store { addr: u8, src: u8 },
    Load { dest: u8, src: u8 },
    Invalid,
}

pub fn assemble(program: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    let mut instructions = Vec::new();

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

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let inst = match parts[0].to_uppercase().as_str() {
            "NOP" => {
                assert!(parts.len() == 1, "NOP takes no arguments");
                Instruction::Nop
            },
            "ADD" | "SUB" | "MUL" | "LOAD" => {
                assert!(parts.len() == 3, "{} requires 2 register arguments", parts[0].to_uppercase());
                let dest = parts[1].parse::<u8>()?;
                let src = parts[2].parse::<u8>()?;
                assert!(dest < 4 && src < 4, "Register index must be 0-3");
                match parts[0].to_uppercase().as_str() {
                    "ADD" => Instruction::Add { dest, src },
                    "SUB" => Instruction::Sub { dest, src },
                    "MUL" => Instruction::Mul { dest, src },
                    "LOAD" => Instruction::Load { dest, src },
                    _ => unreachable!()
                }
            },
            "LOADI" => {
                assert!(parts.len() == 3, "LOADI requires a register and immediate value");
                let dest = parts[1].parse::<u8>()?;
                let imm = parts[2].parse::<u8>()?;
                assert!(dest < 4, "Register index must be 0-3");

                Instruction::LoadI { dest, imm }
            },
            "STORE" => {
                assert!(parts.len() == 3, "STORE requires a register for address and a register for value");
                let addr = parts[1].parse::<u8>()?;
                let src = parts[2].parse::<u8>()?;
                assert!(addr < 4 && src < 4, "Register index must be 0-3");
                Instruction::Store { addr, src }
            },
            "INVALID" => {
                assert!(parts.len() == 1, "INVALID takes no arguments");
                Instruction::Invalid
            },
            _ => return Err(err!("Unknown instruction")),
        };

        let encoded = match inst {
            Instruction::Nop => {
                0b0000u64                   // NOP opcode (0x0)
            },
            Instruction::Add { dest, src } => {
                ((0u64) << 8) |             // immediate
                ((src as u64) << 6) |       // reg_src
                ((dest as u64) << 4) |      // reg_dest
                (0b0001u64)                 // ADD opcode (0x1)
            },
            Instruction::Sub { dest, src } => {
                ((0u64) << 8) |             // immediate
                ((src as u64) << 6) |       // reg_src
                ((dest as u64) << 4) |      // reg_dest
                (0b0010u64)                 // SUB opcode (0x2)
            },
            Instruction::Mul { dest, src } => {
                ((0u64) << 8) |             // immediate
                ((src as u64) << 6) |       // reg_src
                ((dest as u64) << 4) |      // reg_dest
                (0b0011u64)                 // MUL opcode (0x3)
            },
            Instruction::LoadI { dest, imm } => {
                ((imm as u64) << 8) |       // immediate
                ((0u64) << 6) |             // reg_src (unused)
                ((dest as u64) << 4) |      // reg_dest
                (0b1000u64)                 // LOADI opcode (0x8)
            },
            Instruction::Store { addr, src } => {
                ((0u64) << 8) |             // immediate (unused)
                ((src as u64) << 6) |       // reg_src (value to store)
                ((addr as u64) << 4) |      // reg_dest (address)
                (0b1001u64)                 // STORE opcode (0x9)
            },
            Instruction::Load { dest, src } => {
                ((0u64) << 8) |             // immediate (unused)
                ((src as u64) << 6) |       // reg_src (address)
                ((dest as u64) << 4) |      // reg_dest
                (0b1011u64)                 // LOAD opcode (0xB)
            },
            Instruction::Invalid => {
                0b1111u64                   // INVALID opcode (0xF)
            },
        };

        instructions.push(encoded as u16);
    }

    Ok(instructions)
} 