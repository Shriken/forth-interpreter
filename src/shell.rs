use readline;
use readline::readline;

use std::error::Error;

use forth::state::State;

pub fn run_shell(state: &mut State) -> Result<(), Box<Error>> {
    loop {
        match readline("> ") {
            Ok(line) => if let Err(e) = state.run_line(line.to_string()) {
                println!("error: {}", e);
            },
            Err(readline::Error::EndOfFile) => break,
            Err(e) => return Err(::std::convert::From::from(e)),
        }
    }

    println!("");
    Ok(())
}
