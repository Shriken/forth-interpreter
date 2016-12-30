use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

mod shell;
mod forth;

fn main() {
    // Get the program arguments
    let args: Vec<String>;
    {
        let mut arg_iter = env::args();
        let name = arg_iter.next().unwrap();
        args = arg_iter.collect();

        if args.len() > 1 {
            usage(name);
            process::exit(-1);
        }
    }

    let mut state = forth::State::new();

    // Interpret input
    if args.len() == 0 {
        if let Err(err) = shell::run_shell(&mut state) {
            error(&err);
        }
    } else if args.len() == 1 {
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
    }
}

fn error(err: &Error) {
    println!("error: {}", err);
    process::exit(-1);
}

fn usage(name: String) {
    println!("usage: {} [file]", name);
}
