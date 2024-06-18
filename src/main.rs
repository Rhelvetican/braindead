use std::{env::args, fs::read_to_string, process::exit};

use anyhow::Result;
use interpreter::{token::Instruction, Interpreter};
use lexer::Lexer;

mod interpreter;
mod lexer;

fn main() -> Result<()> {
    let arguments = args().collect::<Vec<String>>();
    if arguments.len() > 2 {
        eprintln!("Usage: {} <filename>", arguments[0]);
        exit(1);
    }
    let filename = &arguments[1];
    let code = match read_to_string(filename) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("Error reading file '{}': {}", filename, error);
            exit(1);
        }
    };
    let lexer = Lexer::new(code, vec![' ', '\n', '\t']);
    let tokens = lexer.lex();
    let cmds = Instruction::into_cmd(tokens)?;
    let mut inter: Interpreter<u32> = Interpreter::new();

    inter.run(cmds)?;

    Ok(())
}
