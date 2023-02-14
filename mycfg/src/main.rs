use std::process;

use frontend::parse_program;
use frontend::parse_stdin;

fn main() {
    let json = parse_stdin().unwrap_or_else(|err| {
        eprintln!("Problem parsing stdin: {}", err);
        process::exit(1);
    });

    let prog = parse_program(&json);

    print!("{}", prog);
}
