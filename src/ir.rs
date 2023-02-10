use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BfIR {
    AddVal(u8),  // +
    SubVal(u8),  // -
    AddPtr(u32), // >
    SubPtr(u32), // <
    GetByte,     // ,
    PutByte,     // .
    Jz,          // [
    Jnz,         // ]
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompileErrorKind {
    UncloseLeftBracket,
    UnexpectedRightBracket,
}

impl Display for CompileErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileErrorKind::UncloseLeftBracket => write!(f, "Unclosed left bracket"),
            CompileErrorKind::UnexpectedRightBracket => write!(f, "Unclosed left bracket"),
        }
    }
}

impl Error for CompileErrorKind {}

#[derive(Debug)]
pub struct CompileError {
    line: u32,
    col: u32,
    kind: CompileErrorKind,
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at line {}:{}", self.kind, self.line, self.col)
    }
}

pub fn compile<S: AsRef<str>>(src: S) -> Result<Vec<BfIR>, CompileError> {
    let mut code: Vec<BfIR> = vec![];

    let mut stk: Vec<(u32, u32, u32)> = vec![];

    let mut line: u32 = 0;
    let mut col: u32 = 0;

    let src = src.as_ref();

    for ch in src.chars() {
        col += 1;
        match ch {
            '\n' => {
                line += 1;
                col = 0;
            }
            '+' => code.push(BfIR::AddVal(1)),
            '-' => code.push(BfIR::SubVal(1)),
            '>' => code.push(BfIR::AddPtr(1)),
            '<' => code.push(BfIR::SubPtr(1)),
            ',' => code.push(BfIR::GetByte),
            '.' => code.push(BfIR::PutByte),
            '[' => {
                let pos = code.len() as u32;
                stk.push((pos, line, col));
                code.push(BfIR::Jz)
            }
            ']' => {
                stk.pop().ok_or(CompileError {
                    line,
                    col,
                    kind: CompileErrorKind::UnexpectedRightBracket,
                })?;
                code.push(BfIR::Jnz)
            }
            _ => {}
        }
    }
    if let Some((_, line, col)) = stk.pop() {
        return Err(CompileError {
            line,
            col,
            kind: CompileErrorKind::UncloseLeftBracket,
        });
    }
    Ok(code)
}

pub fn optimize_ir(code: &mut Vec<BfIR>) {
    let mut i = 0;
    let mut pc = 0;
    let len = code.len();

    macro_rules! fold_ir {
        ($ir_type: ident, $x: expr) => {{
            let mut j = i + 1;
            while j < len {
                if let BfIR::$ir_type(d) = code[j] {
                    $x = $x.wrapping_add(d);
                } else {
                    break;
                }
                j += 1;
            }
            i = j;
            code[pc] = BfIR::$ir_type($x);
            pc += 1;
        }};
    }

    macro_rules! normal_ir {
        () => {{
            code[pc] = code[i];
            pc += 1;
            i += 1;
        }};
    }
    // 折叠IR
    while i < len {
        match code[i] {
            BfIR::AddVal(mut x) => fold_ir!(AddVal, x),
            BfIR::SubVal(mut x) => fold_ir!(SubVal, x),
            BfIR::AddPtr(mut x) => fold_ir!(AddPtr, x),
            BfIR::SubPtr(mut x) => fold_ir!(SubPtr, x),
            BfIR::GetByte => normal_ir!(),
            BfIR::PutByte => normal_ir!(),
            BfIR::Jz => normal_ir!(),
            BfIR::Jnz => normal_ir!(),
        }
    }
    code.truncate(pc);
    code.shrink_to_fit();
}

#[cfg(test)]
mod test {
    use crate::ir::{compile, BfIR, CompileErrorKind};

    use super::optimize_ir;

    #[test]
    fn test_compile() {
        assert_eq!(
            compile("+[,.]").unwrap(),
            vec![
                BfIR::AddVal(1),
                BfIR::Jz,
                BfIR::GetByte,
                BfIR::PutByte,
                BfIR::Jnz
            ]
        );

        assert_eq!(
            compile("[").unwrap_err().kind,
            CompileErrorKind::UncloseLeftBracket,
        );

        assert_eq!(
            compile("]").unwrap_err().kind,
            CompileErrorKind::UnexpectedRightBracket,
        );
    }

    #[test]
    fn test_optimize() {
        let mut code = compile("[+++++++]").unwrap();
        optimize_ir(&mut code);
        assert_eq!(code, vec![BfIR::Jz, BfIR::AddVal(7), BfIR::Jnz]);
    }
}
