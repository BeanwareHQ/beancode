use beancode::cli::{Arguments, Cli};
use clap::Parser;

fn main() {
    let args = Arguments::parse();
    Cli::run(args);
}
