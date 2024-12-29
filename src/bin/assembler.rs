use std::env;
use std::fs;
use std::error::Error;
use std::io::Write;
use verilog_ctf::assembler::assemble;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Read input file
    let input = fs::read_to_string(input_path)?;

    // Assemble the program
    let assembled = assemble(&input)?;

    // Write output as binary
    let mut output_file = fs::File::create(output_path)?;
    for word in assembled {
        output_file.write_all(&word.to_le_bytes())?;
    }

    Ok(())
} 