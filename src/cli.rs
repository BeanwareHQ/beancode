use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

use crate::backend;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    path: PathBuf,
}

pub struct Cli;

impl Cli {
    pub fn run(args: Arguments) {
        println!("{}\n", "The beancode transpiler.".bold());
        let file_contents = std::fs::read_to_string(args.path).unwrap();

        let output = backend::lexer::Lexer::new(file_contents).lex().unwrap();

        print!("[");
        for (idx, tok) in output.iter().enumerate() {
            print!("{:?}", tok.variant);

            if idx < output.len() - 1 {
                print!(", ")
            } else {
                println!("]");
            };
        }
    }
}
