mod lexer;
mod parser;
mod code_generator;
mod symbol_table;
mod semantic_analyzer;
mod error;
use std::{env};
use crate::error::errors::AssemblerErrors;
use crate::parser::program::{GrammarProductionParsing, Program};

fn main() -> Result<(), AssemblerErrors> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let program = Program::new(args[1].clone());
        let result = program.unwrap().parse(None);
        println!("Assembler status: {:?}", result);
        return result
    } else {
        eprintln!("Error, usage: {} <file.asm>", args[0]);
        Err(AssemblerErrors::WrongArgument)
    }
}