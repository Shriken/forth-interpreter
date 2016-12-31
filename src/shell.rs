use std::error::Error;
use std::io;
use std::io::BufRead;
use std::io::Write;

use forth;

pub fn run_shell(state: &mut forth::State) -> Result<(), Box<Error>> {
    let stdin = io::stdin();
    let locked = stdin.lock();

    prompt();
    for l in locked.lines() {
        let line = l?;
        if line == "exit" {
            break;
        }

        try!(state.run_line(line));
        prompt();
    }
    println!("");

    //locked.unlock();
    Ok(())
}

fn prompt() {
    print!("> ");
    let mut stdout = io::stdout();
    loop {
        if let Ok(_) = stdout.flush() {
            break;
        }
    }
}
