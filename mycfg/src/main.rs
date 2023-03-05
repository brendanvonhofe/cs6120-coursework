use json::JsonValue;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::process;

use mycfg::parser;

const DEBUG_FILE: &str = "/Users/brendan/Desktop/cs6120/mycfg/tests/fib2seven.json";
const NORMAL_MODES: [&str; 3] = ["main", "cfg", "opt"];

fn parse_stdin() -> Result<JsonValue, Box<dyn Error>> {
    let mut contents = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut contents)?;
    let program = json::parse(&contents)?;
    return Ok(program);
}

fn parse_file(filename: &str) -> Result<JsonValue, Box<dyn Error>> {
    let contents = String::from(fs::read_to_string(filename)?);
    let program = json::parse(&contents)?;
    return Ok(program);
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let mode = args.next().unwrap_or(String::from("dbg")).to_lowercase();

    let json: JsonValue;
    if NORMAL_MODES.contains(&mode.as_str()) {
        json = parse_stdin().unwrap_or_else(|err| {
            eprintln!("Problem parsing stdin: {}", err);
            process::exit(1);
        });
    } else {
        json = parse_file(DEBUG_FILE).unwrap_or_else(|err| {
            eprintln!("Problem parsing file {}: {}", DEBUG_FILE, err);
            process::exit(1);
        });
    }

    match mode.as_str() {
        "main" => {
            print!("{}", parser::parse_program(&json));
        }
        "cfg" => {
            println!("{}", parser::parse_program(&json).graphviz().unwrap());
        }
        "opt" => {
            let mut prog = parser::parse_program(&json);
            println!("[BEFORE OPTIMIZATIONS] {}", &prog);
            for i in 0..prog.functions.len() {
                prog.functions[i] = prog.functions[i].dead_variable_elim();
                for j in 0..prog.functions[i].blocks.len() {
                    prog.functions[i].blocks[j] = prog.functions[i].blocks[j].dead_store_elim();
                }
            }
            println!("[AFTER] {}", prog);
        }
        _ => {
            println!("[DEBUG MODE] Reading program from {}\n", DEBUG_FILE);
            print!("{}", parser::parse_program(&json));
        }
    }
}
