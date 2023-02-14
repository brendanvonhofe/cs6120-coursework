use json::JsonValue;
use std::error::Error;
use std::fs;
use std::process;

use frontend::parse_program;

fn parse_file(filename: &str) -> Result<JsonValue, Box<dyn Error>> {
    let contents = String::from(fs::read_to_string(filename)?);
    let program = json::parse(&contents)?;
    return Ok(program);
}

fn main() {
    let json = parse_file("/Users/brendan/Desktop/cs6120/mycfg/test/fib2seven.json")
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing stdin: {}", err);
            process::exit(1);
        });

    let prog = parse_program(&json);

    print!("{}", prog);
}
