#![feature(if_let_guard)]
use clap::Parser;
use std::{collections::HashMap, fs};

// Pointer is just an indice in a vec
type Pointer = usize;
type Instructions = Vec<ByteCode>;

#[derive(Debug)]
enum ByteCode {
    Load(i64),
    Read(String),
    Write(String),
    Jump(Pointer),
    Je(Pointer),
    Add,
    Sub,
    Mul,
    Div,
    Ret,
}

impl ByteCode {
    fn parse_instruction(bytecode: &str) -> Result<ByteCode, String> {
        match bytecode.split(' ').collect::<Vec<_>>().as_slice() {
            &["LOAD_VAL", num] if let Ok(num) = num.parse() => Ok(ByteCode::Load(num)),
            &["WRITE_VAR", var] => Ok(ByteCode::Write(var.into())),
            &["READ_VAR", var] => Ok(ByteCode::Read(var.into())),
            &["JUMP", pointer] if let Ok(p) = pointer.parse() => Ok(ByteCode::Jump(p)),
            &["JE", pointer] if let Ok(p) = pointer.parse()=> Ok(ByteCode::Je(p)),
            &["ADD"] => Ok(ByteCode::Add),
            &["SUB"] => Ok(ByteCode::Sub),
            &["MULTIPLY"] => Ok(ByteCode::Mul),
            &["DIVIDE"] => Ok(ByteCode::Div),
            &["RETURN_VALUE"] => Ok(ByteCode::Ret),
            invalid => Err(format!("Invalid {:?}", invalid.join(" "))),
        }
    }

    fn parse(program: String) -> Instructions {
        program
            .lines()
            .filter(|l| !l.is_empty())
            .map(|instruction| {
                instruction
                    .to_string()
                    .retain(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9'));
                Self::parse_instruction(instruction).unwrap()
            })
            .collect()
    }

    pub fn interpret(program: String) -> Option<i64> {
        let mut stack = Vec::new();
        let mut variables = HashMap::new();
        let mut ip = 0;

        let instructions = Self::parse(program);

        loop {
            match instructions.get(ip).unwrap() {
                ByteCode::Load(x) => {
                    stack.push(*x);
                    ip += 1;
                }
                ByteCode::Read(var) => {
                    stack.push(*variables.get(&var).unwrap());
                    ip += 1;
                }
                ByteCode::Write(var) => {
                    variables.insert(var, stack.pop().unwrap());
                    ip += 1;
                }
                ByteCode::Jump(pointer) => {
                    ip = *pointer;
                }
                ByteCode::Je(pointer) => {
                    if stack.pop().unwrap() == 0 {
                        ip = *pointer;
                    } else {
                        ip += 1;
                    }
                }
                ByteCode::Add => {
                    let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(x + y);
                    ip += 1;
                }
                ByteCode::Sub => {
                    let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(y - x);
                    ip += 1;
                }
                ByteCode::Mul => {
                    let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(x * y);
                    ip += 1;
                }
                ByteCode::Div => {
                    let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(x / y);
                    ip += 1;
                }
                ByteCode::Ret => {
                    return stack.pop();
                }
            };
        }
    }
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

    let result = ByteCode::interpret(program).unwrap();
    println!("{result}");
}
