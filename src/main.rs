#![feature(if_let_guard)]
use clap::Parser;
use std::{collections::HashMap, fs, vec};

enum ByteCode {
    Load(i64),
    Read(String),
    Write(String),
    Add,
    Sub,
    Mul,
    Div,
    Ret,
}

impl ByteCode {
    pub fn parse(bytecode: &str) -> Result<ByteCode, String> {
        match bytecode.split(' ').collect::<Vec<_>>().as_slice() {
            &["LOAD_VAL", num] if let Ok(num) = num.parse() => Ok(ByteCode::Load(num)),
            &["WRITE_VAR", var] => Ok(ByteCode::Write(var.into())),
            &["READ_VAR", var] => Ok(ByteCode::Read(var.into())),
            &["ADD"] => Ok(ByteCode::Add),
            &["SUB"] => Ok(ByteCode::Sub),
            &["MULTIPLY"] => Ok(ByteCode::Mul),
            &["DIVIDE"] => Ok(ByteCode::Div),
            &["RETURN_VALUE"] => Ok(ByteCode::Ret),
            invalid => Err(format!("Invalid {:?}", invalid.join(" "))),
        }
    }
}

pub fn interpret(program: String) -> Option<i64> {
    let mut stack = vec![];
    let mut variables = HashMap::new();

    let instructions: Vec<_> = program
        .lines()
        .filter(|l| !l.is_empty())
        .map(|i| {
            i.to_string()
                .retain(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9'));
            ByteCode::parse(i).unwrap()
        })
        .collect();

    for instruction in instructions {
        match instruction {
            ByteCode::Load(x) => stack.push(x),
            ByteCode::Read(var) => stack.push(*variables.get(&var).unwrap()),
            ByteCode::Write(var) => {
                variables.insert(var, stack.pop().unwrap());
            }
            ByteCode::Add => {
                let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(x + y);
            }
            ByteCode::Sub => {
                let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(x - y);
            }
            ByteCode::Mul => {
                let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(x * y);
            }
            ByteCode::Div => {
                let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(x / y);
            }
            ByteCode::Ret => {
                return Some(stack.pop().unwrap());
            }
        };
    }
    None
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// ByteCode File
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let program = fs::read_to_string(args.file).expect("Cannot Read the file");

    let result = interpret(program).unwrap();
    println!("{result}");
}
