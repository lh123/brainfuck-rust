use std::fmt::Display;

#[derive(Debug)]
pub enum RuntimeError {
    IO(std::io::Error),
    PointerOverflow,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::IO(io) => write!(f, "IO: {}", io),
            RuntimeError::PointerOverflow => write!(f, "Pointer overflow"),
        }
    }
}

impl std::error::Error for RuntimeError {}

#[derive(Debug)]
pub enum VMError {
    IO(std::io::Error),
    Compile(crate::ir::CompileError),
    Runtime(RuntimeError),
}

impl Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMError::IO(err) => write!(f, "IO: {}", err),
            VMError::Compile(err) => write!(f, "Compile: {}", err),
            VMError::Runtime(err) => write!(f, "Runtime: {}", err),
        }
    }
}

impl From<RuntimeError> for VMError {
    fn from(value: RuntimeError) -> Self {
        VMError::Runtime(value)
    }
}

impl From<std::io::Error> for VMError {
    fn from(value: std::io::Error) -> Self {
        VMError::IO(value)
    }
}

impl From<crate::ir::CompileError> for VMError {
    fn from(value: crate::ir::CompileError) -> Self {
        VMError::Compile(value)
    }
}

impl std::error::Error for VMError {}

pub type Result<T> = std::result::Result<T, VMError>;
