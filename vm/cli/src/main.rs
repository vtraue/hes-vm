use anyhow::{Context, Result, bail, ensure};

use clap::{Parser, Subcommand};
use colored::Colorize;
use console::graphics::App;
use interpreter::slow_vm::{DebugEnv, LocalValue, Vm};
use itertools::Itertools;
use parser::{
    info::FunctionType,
    reader::{BytecodeReader, ExportDesc, is_wasm_bytecode},
};
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
    Run { name: String },
    Console,
}

pub fn read_and_validate_file(file: &mut impl BytecodeReader) -> Result<ValidateResult> {
    if is_wasm_bytecode(file).context("Failed to determine file type")? {
        read_and_validate(file).context("Unable to validate bytecode")
    } else {
        let mut code = String::new();
        file.read_to_string(&mut code)
            .context("Unable to read wat source code")?;
        read_and_validate_wat(code).context("Error while reading")
    }
}

pub fn execute_run_command(
    func_name: &str,
    params: impl IntoIterator<Item = LocalValue> + Clone,
    file: &mut File,
) -> Result<()> {
    let validate_result = read_and_validate_file(file).context("Unable to parse file")?;
    let mut env = DebugEnv {};
    let mut vm =
        Vm::init_from_validation_result(&validate_result).context("Unable to instantiate")?;

    if let Some(exported) = validate_result.bytecode.get_exports_as_map() {
        let func = exported.get_function_id(func_name);
        ensure!(
            func.is_some(),
            "This function does not exist or is not exported by the module"
        );
        let func_id = func.unwrap();

        println!("code id: {}", func_id);

        vm.set_func(func_id, params.clone())
            .context("Unable to load function")?;

        let result = vm
            .run_func(&validate_result.bytecode, &validate_result.info, &mut env)
            .context("Error while executing {func_name}")?;

        vm.set_func(func_id, params)
            .context("Unable to load function")?;
        let result = vm
            .run_func(&validate_result.bytecode, &validate_result.info, &mut env)
            .context("Error while executing {func_name}")?;

        if result.len() == 1 {
            println!("{}", result[0])
        } else {
            println!("[{}]", result.iter().map(|r| r.to_string()).format(", "));
        }
        Ok(())
    } else {
        bail!("This module does not export any functions")
    }
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Validate => {
            let mut file = File::open(args.path)?;
            let _validate_result = read_and_validate_file(&mut file)?;
            println!("OK!");
            Ok(())
        }
        Commands::Console => App::run(args.path).context("Unable to run console"),

        Commands::Run { name } => {
            let mut file = File::open(args.path).unwrap();
            execute_run_command(&name, Vec::new(), &mut file)
        }
        _ => bail!("Unknown command: {:?}", args.command),
    }
}
