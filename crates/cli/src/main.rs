mod repl;

use std::env;
use std::error::Error;
use std::fs;

use crisp::eval::{eval, CrispEnv};
use crisp::parse::parse;
use crisp::parse::CrispResult;
use crisp::{lexer, run_program};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(file) = args.get(1) {
        let contents = fs::read_to_string(file)?;
        let output = interpret(&contents)?;

        println!("{}", output.to_string());
    } else {
        let mut env = CrispEnv::default();
        repl::run(&mut env)?;
    }

    Ok(())
}

fn interpret(expr: &str) -> CrispResult {
    let mut env = CrispEnv::default();
    run_program(expr, &mut env)
}
