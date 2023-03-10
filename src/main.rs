mod error;
mod inter;
mod ir;
mod jit;

use std::{
    io::{stdin, stdout},
    path::PathBuf,
};

use clap::Parser;
use inter::Interpreter;
use jit::VM;

#[derive(Debug, clap::Parser)]
#[clap(version)]
struct Opt {
    #[clap(name = "FILE")]
    file_path: PathBuf,
    #[clap(short = 'o', long = "optimize", help = "Optimize code")]
    optimize: bool,
    #[clap(short = 'i', long = "interpreter", help = "Interpreter mode")]
    interpreter: bool,
}

fn main() {
    let opt = Opt::parse();

    let stdin = stdin();
    let stdout = stdout();

    let ret = if opt.interpreter {
        Interpreter::new(
            &opt.file_path,
            Box::new(stdin.lock()),
            Box::new(stdout.lock()),
            opt.optimize,
        )
        .and_then(|mut vm| vm.run())
    } else {
        VM::new(
            &opt.file_path,
            Box::new(stdin.lock()),
            Box::new(stdout.lock()),
            opt.optimize,
        )
        .and_then(|mut vm| vm.run())
    };

    if let Err(ref e) = ret {
        eprintln!("bfjit: {}", e);
    }
    std::process::exit(ret.is_err() as i32);
}
