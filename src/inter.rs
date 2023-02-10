use std::{
    io::{Read, Write},
    path::Path,
    slice,
};

use crate::{
    error::{self, Result, RuntimeError},
    ir::{self, BfIR},
};

pub struct Interpreter {
    pc: u32,
    ptr: u32,
    code: Vec<BfIR>,
    memory: Box<[u8]>,
    input: Box<dyn Read>,
    output: Box<dyn Write>,
}

const MEM_SIZE: usize = 4 * 1024 * 1024;

impl Interpreter {
    pub fn new(
        path: &Path,
        input: Box<dyn Read>,
        output: Box<dyn Write>,
        optimize: bool,
    ) -> Result<Self> {
        let src = std::fs::read_to_string(path)?;
        let mut ir = ir::compile(&src)?;
        drop(src);

        if optimize {
            ir::optimize_ir(&mut ir);
        }
        let memory = vec![0; MEM_SIZE].into_boxed_slice();
        Ok(Self {
            pc: 0,
            ptr: 0,
            code: ir,
            memory,
            input,
            output,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let code_len = self.code.len() as u32;
        while self.pc < code_len {
            // println!("Code: {:?}, PC: {}", self.code[self.pc], self.pc);
            match &self.code[self.pc as usize] {
                BfIR::AddVal(x) => {
                    self.memory[self.ptr as usize] = self.memory[self.ptr as usize].wrapping_add(*x)
                }
                BfIR::SubVal(x) => {
                    self.memory[self.ptr as usize] = self.memory[self.ptr as usize].wrapping_sub(*x)
                }
                BfIR::AddPtr(x) => {
                    // len < ptr + x
                    if self.memory.len() as u32 - self.ptr <= *x {
                        return Err(error::VMError::Runtime(RuntimeError::PointerOverflow));
                    }
                    self.ptr = self.ptr + *x;
                }
                BfIR::SubPtr(x) => {
                    if self.ptr < *x {
                        return Err(error::VMError::Runtime(RuntimeError::PointerOverflow));
                    }
                    self.ptr -= x;
                }
                BfIR::GetByte => {
                    let mut buf = [0_u8];
                    match self.input.read(&mut buf) {
                        Ok(0) => (),
                        Ok(1) => self.memory[self.ptr as usize] = buf[0],
                        Err(e) => return Err(error::VMError::IO(e)),
                        _ => unreachable!(),
                    }
                }
                BfIR::PutByte => {
                    let val = self.memory[self.ptr as usize];
                    match self.output.write_all(slice::from_ref(&val)) {
                        Ok(_) => (),
                        Err(e) => return Err(error::VMError::IO(e)),
                    }
                }
                BfIR::Jz(pos) => {
                    if self.memory[self.ptr as usize] == 0 {
                        self.pc = *pos;
                    }
                }
                BfIR::Jnz(pos) => {
                    self.pc = *pos - 1;
                }
            }
            self.pc += 1;
        }
        Ok(())
    }
}
