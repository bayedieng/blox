use std::io::{stdin, stdout, Write};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl()
    } else if args.len() == 2 {
        run_file(&args[1])
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

fn run_file(_path: &str) {}
