use json::JsonValue;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::process;

use mycfg::core;
use mycfg::parser;

const DEBUG_FILE: &str = "/Users/brendan/Desktop/cs6120/mycfg/tests/fib2seven.json";

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

fn print_graphviz(prog: core::Program) {
    for func in prog.functions.iter() {
        println!("digraph {} {{", func.name);
        let cfg = parser::control_flow_graph(func);
        let mut sorted_keys: Vec<&String> = cfg.keys().collect();
        sorted_keys.sort();
        for &key in &sorted_keys {
            println!("  {};", key);
        }
        for &key in &sorted_keys {
            for succ in cfg[key].iter() {
                println!("  {key} -> {succ};");
            }
        }
        println!("}}");
        break;
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let mode = args.next();
    match mode {
        Some(s) => match s.to_lowercase().as_str() {
            "cfg" => {
                let json = parse_stdin().unwrap_or_else(|err| {
                    eprintln!("Problem parsing stdin: {}", err);
                    process::exit(1);
                });

                let prog = parser::parse_program(&json);

                print_graphviz(prog);
            }
            "dbg" => {
                let json = parse_file(DEBUG_FILE).unwrap_or_else(|err| {
                    eprintln!("Problem parsing file {}: {}", DEBUG_FILE, err);
                    process::exit(1);
                });

                let prog = parser::parse_program(&json);

                print!("{}", prog);
            }
            "main" => {
                let json = parse_stdin().unwrap_or_else(|err| {
                    eprintln!("Problem parsing stdin: {}", err);
                    process::exit(1);
                });

                let prog = parser::parse_program(&json);

                print!("{}", prog);
            }
            "opt" => {
                let json = parse_stdin().unwrap_or_else(|err| {
                    eprintln!("Problem parsing stdin: {}", err);
                    process::exit(1);
                });

                let prog = parser::parse_program(&json);

                for mut func in prog.functions.into_iter() {
                    println!("Before: {}", func);
                    for i in 0..func.blocks.len() {
                        func.blocks[i] = func.blocks[i].dead_variable_elim();
                    }
                    println!("After: {}", func);
                }
            }
            _ => {
                let json = parse_file(DEBUG_FILE).unwrap_or_else(|err| {
                    eprintln!("Problem parsing file {}: {}", DEBUG_FILE, err);
                    process::exit(1);
                });

                let prog = parser::parse_program(&json);

                print!("{}", prog);
            }
        },
        None => {
            let json = parse_file(DEBUG_FILE).unwrap_or_else(|err| {
                eprintln!("Problem parsing file {}: {}", DEBUG_FILE, err);
                process::exit(1);
            });

            let prog = parser::parse_program(&json);

            print!("{}", prog);
        }
    }
}
