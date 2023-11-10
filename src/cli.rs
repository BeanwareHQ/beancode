use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {}

pub struct Cli;

impl Cli {
    pub fn run(args: Arguments) {
        println!("The beanmaths transpiler.");
    }
}
