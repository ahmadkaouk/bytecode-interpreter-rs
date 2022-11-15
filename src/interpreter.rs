pub struct Interpreter {
    variables: HashMap<String, u64>,
    stack: Vec<u64>,
}

impl Interpreter {
    /// Execute all the instructions, and return the result
    pub fn execute(instructions: &[String]) {
    }

    /// Execute one instruction at a time
    pub fn run(instruction: &str);
}