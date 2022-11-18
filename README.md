## Bytecode Interpreter
A very simple stack-based bytecode interpreter written in Rust.
The interpreter is a command line utility
## Supported Instructions
| Instruction     | Description                                                                              |
|-----------------|------------------------------------------------------------------------------------------|
| `LOAD_VAL val`  |  Push the number to the top of stack                                                     |
| `READ_VAR var`  |  Read the value of `var` and push it to the top of the stack                             |
| `WRITE_VAR var` |  Pop the top of the stack and Write it in `var`                                          |
| `JUMP pointer`  |  Jumps to `pointer`                                                                      |
| `JE pointer`    |  Pops the top of the stack and Jumps to `pointer` if the value is 0                      |
| `ADD`           |  Pops the first two values, compute their sum and push the result to the stack           |
| `SUB`           |  Pops the first two values, compute their difference and push the result to the stack    |
| `MUL`           |  Pops the first two values, multiply them by each other and push the result to the stack |
| `DIV`           |  Pops the first two values, compute their quotient and push the result to the stack      |
| `RET`           |  Finish the interpretation and returns the top of the stack (if a value exists)          |

## Usage
```bash
Usage: interpreter-rs --file <FILE>

Options:
  -f, --file <FILE>  ByteCode File
  -h, --help         Print help information
  -V, --version      Print version information
```
## Example
### Arithmetic Operations
```bash
## tests/test
function f() {

    x = 1                   LOAD_VAL 1
                            WRITE_VAR ‘x’

    y = 2                   LOAD_VAL 2
                            WRITE_VAR ‘y’

    return (x + 1) * y      READ_VAR ‘x’
                            LOAD_VAL 1
                            ADD

                            READ_VAR ‘y’
                            MULTIPLY

                            RETURN_VALUE
}

$ interpreter-rs --file tests/test
4
```
### Loops
```bash
## tests/loop_test
function f() {

    a = 10                  LOAD_VAL 10
                            WRITE_VAR 'a'

    b = 0                   LOAD_VAL 0
                            WRITE_VAR 'b'

    while a                 READ_VAR 'a'
        b = b + 1           JE 15
        a = a - 1           READ_VAR 'b'
                            LOAD_VAL 1
                            ADD
                            WRITE_VAR 'b'
                            READ_VAR 'a'
                            LOAD_VAL 1
                            SUB
                            WRITE_VAR 'a'
                            JUMP 4

    return b                RETURN_VALUE
}

$ interpreter-rs --file tests/test
10
```

## ToDo
- [ ] better error handling