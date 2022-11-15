use std::fs;
use clap::Parser;

enum ByteCode {
    Load,
    Read,
    Write,
    Add,
    Sub,
    Multiply,
    Divide,
    Return,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let instructions = fs::read_to_string(args.path);
}
