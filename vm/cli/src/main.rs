use anyhow::{Context, Result, bail};

use clap::{Parser, Subcommand};
use colored::Colorize;
use interpreter::slow_vm::{LocalValue, Vm, make_test_env};
use itertools::Itertools;
use parser::reader::{BytecodeReader, is_wasm_bytecode};
use std::{
    fs::File,
    io::{Cursor, Read, Seek},
};
use validator::validator::{ValidateResult, read_and_validate, read_and_validate_wat};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Validate,
    Run { function_id: usize },
}

pub fn read_and_validate_file(file: &mut impl BytecodeReader) -> Result<ValidateResult> {
    if is_wasm_bytecode(file).context("Failed to determine file type")? {
        read_and_validate(file).context("Unable to validate bytecode")
    } else {
        let mut code = String::new();
        file.read_to_string(&mut code)
            .context("Unable to read wat source code")?;
        eprintln!("{}", code);
        read_and_validate_wat(code).context("Error while reading")
    }
}

pub fn execute_run_command(
    func_id: usize,
    params: impl IntoIterator<Item = LocalValue>,
    file: &mut File,
) -> Result<()> {
    let validate_result = read_and_validate_file(file).context("Unable to parse file")?;
    let mut vm = Vm::init_from_validation_result(&validate_result, make_test_env())
        .context("Unable to instantiate")?;
    let result = vm
        .run_func(
            &validate_result.bytecode,
            &validate_result.info,
            func_id,
            params,
        )
        .context("Execution error")?;
    if result.len() == 1 {
        println!("{}", result[0])
    } else {
        println!("[{}]", result.iter().map(|r| r.to_string()).format(", "));
    }
    Ok(())
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.path).unwrap();
    match args.command {
        Commands::Validate => {
            let _validate_result = read_and_validate_file(&mut file)?;
            Ok(())
        }
        Commands::Run { function_id } => execute_run_command(function_id, Vec::new(), &mut file),
        _ => bail!("Unknown command: {:?}", args.command),
    }
}
