extern crate readline;

mod shell;
mod forth;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

use forth::State;

fn main() {
    // Get the program arguments
    let name: String;
    let args: Vec<String>;
    {
        let mut arg_iter = env::args();
        name = arg_iter.next().unwrap();
        args = arg_iter.collect();
    }

    if args.len() == 1 && args[0] == "-h" {
        usage(name);
        process::exit(0);
    }

    // Interpret input
    let mut state = State::new();
    match args.len() {
        0 => {
            if let Err(err) = shell::run_shell(&mut state) {
                error(&*err);
            }
        },

        1 => {
            let file: File;
            match File::open(&args[0]) {
                Ok(f) => file = f,
                Err(err) => {
                    error(&err);
                    return;
                }
            }

            let reader = BufReader::new(file);
            for result in reader.lines() {
                match result {
                    Ok(line) =>
                        if let Err(err) = state.run_line(line) {
                            error(&err);
                        },
                    Err(err) => error(&err),
                }
            }
        },

        _ => {
            if args.len() > 1 {
                usage(name);
                process::exit(-1);
            }
        },
    }
}

fn error(err: &Error) {
    println!("error: {}", err);
    process::exit(-1);
}

fn usage(name: String) {
    println!("usage:");
    println!("\t{} : Run the interpreter.", name);
    println!("\t{} <file> : Run the given file.", name);
}
