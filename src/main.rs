use std::io::{stdin, stdout, Write};
use std::mem;
use std::{env, process::exit};

use blox::jit;
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
    let mut jit = jit::JIT::new();
    let code = jit.compile(src)?;
    Ok(println!("{:?}", code))
}

unsafe fn run_code<I, O>(jit: &mut jit::JIT, code: &str, input: I) -> Result<O, String> {
    // Pass the string to the JIT, and it returns a raw pointer to machine code.
    let code_ptr = jit.compile(code)?;
    // Cast the raw pointer to a typed function pointer. This is unsafe, because
    // this is the critical point where you have to trust that the generated code
    // is safe to be called.
    let code_fn = mem::transmute::<_, fn(I) -> O>(code_ptr);
    // And now we can call it!
    Ok(code_fn(input))
}
