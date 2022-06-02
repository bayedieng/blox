use std::io::{stdin, stdout, Write};
use std::{env, process::exit};

use blox::jit::JIT;
fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return Ok(repl());
    } else if args.len() == 2 {
        return run_file(&args[1]);
    }
    exit(0)
}

fn repl() {
    loop {
        print!("> ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).unwrap();
        print!("{s}")
    }
}

fn run_file(_path: &str) -> Result<(), String> {
    let src = include_str!("../test.blox");
    let mut jit = JIT::new();
    jit.compile(src)?;

    Ok(())
}

