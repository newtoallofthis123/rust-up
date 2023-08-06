use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the C file name is provided
    if args.len() != 2 {
        println!("Usage: {} <C_FILE>", args[0]);
        std::process::exit(1);
    }

    let prog_name = &args[1];

    // Check if path prog_name exists
    if !Path::new(prog_name).exists() {
        println!("{} does not exist", prog_name);
        std::process::exit(1);
    }

    // Check if path prog_name is a C file
    if !prog_name.ends_with(".c") {
        println!("{} is not a C file", prog_name);
        std::process::exit(1);
    }

    // Compile the C file
    let compilation = Command::new("gcc")
        .arg(prog_name)
        .arg("-o")
        .arg(prog_name.trim_end_matches(".c"))
        .output();

    match compilation {
        Ok(output) => {
            if !output.status.success() {
                println!("Compilation failed: {:?}", output);
                std::process::exit(1);
            }
        }
        Err(error) => {
            println!("Failed to execute gcc: {:?}", error);
            std::process::exit(1);
        }
    }
}
