use aero_ast::{Function, Program};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Type mismatch: expected {expected}, found {found}")]
    Mismatch { expected: String, found: String },
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("Effect not declared: {0}")]
    UndeclaredEffect(String),
}

pub struct TypedProgram {
    pub program: Program,
}

pub fn check(program: &Program) -> Result<TypedProgram, TypeError> {
    // For now, just do a basic validation
    // In the future, this will do full type checking with effect tracking

    Ok(TypedProgram {
        program: program.clone(),
    })
}

pub fn check_function(_function: &Function) -> Result<(), TypeError> {
    // Basic validation for test functions
    // In the future, this will check function body against signature

    Ok(())
}
