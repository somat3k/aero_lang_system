use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid bytecode")]
    InvalidBytecode,
    #[error("Runtime error: {0}")]
    Execution(String),
}

pub struct Bytecode {
    pub instructions: Vec<u8>,
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode {
            instructions: vec![
                0x00, // NOP
                0xFF, // HALT
            ],
        }
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), RuntimeError> {
        fs::write(path, &self.instructions)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self, RuntimeError> {
        let instructions = fs::read(path)?;
        Ok(Bytecode { instructions })
    }
}

impl Default for Bytecode {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VM {
    // Virtual machine state
    pc: usize,              // Program counter
    #[allow(dead_code)]
    stack: Vec<Value>,      // Value stack
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Unit,
}

impl VM {
    pub fn new() -> Self {
        VM {
            pc: 0,
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, bytecode: &Bytecode, _args: &[String]) -> Result<(), RuntimeError> {
        // Placeholder VM execution
        // In the future, this will be a full register-based bytecode interpreter

        println!("Hello, AERO world!");
        println!("Welcome to AERO");

        while self.pc < bytecode.instructions.len() {
            let opcode = bytecode.instructions[self.pc];

            match opcode {
                0x00 => {
                    // NOP
                    self.pc += 1;
                }
                0xFF => {
                    // HALT
                    break;
                }
                _ => {
                    return Err(RuntimeError::InvalidBytecode);
                }
            }
        }

        Ok(())
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
