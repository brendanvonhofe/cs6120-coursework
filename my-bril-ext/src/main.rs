use json;
use json::JsonValue;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;
use std::process;

const TERMINATORS: [&str; 3] = ["jmp", "br", "ret"];

fn basic_blocks(func: &JsonValue) -> Option<Vec<Vec<&JsonValue>>> {
    let mut blocks: Vec<Vec<&JsonValue>> = vec![];
    let mut block: Vec<&JsonValue> = vec![];

    for op in func["instrs"].members() {
        if op.has_key("op") {
            block.push(op);
            if TERMINATORS.contains(&op["op"].as_str()?) {
                blocks.push(block.clone());
                block.clear();
            }
        } else if op.has_key("label") {
            if !block.is_empty() {
                blocks.push(block.clone());
                block.clear();
            }
            block.push(op);
        }
    }

    if !block.is_empty() {
        blocks.push(block.clone());
    }

    return Some(blocks);
}

fn block_name(block: &Vec<&JsonValue>, i: usize) -> Option<String> {
    if block[0].has_key("label") {
        return Some(String::from(block[0]["label"].as_str()?));
    }
    Some(String::from(format!("block_{}", i)))
}

fn control_flow_graph(blocks: &Vec<Vec<&JsonValue>>) -> Option<HashMap<String, Vec<String>>> {
    let mut cfg: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..blocks.len() - 1 {
        let b = &blocks[i];
        let last = b[b.len() - 1];
        if last["op"].as_str()? == "jmp" || last["op"].as_str()? == "br" {
            cfg.insert(
                block_name(b, i)?,
                Vec::from_iter(
                    last["labels"]
                        .members()
                        .map(|x| -> String { String::from(x.as_str().unwrap()) }),
                ),
            );
        } else {
            cfg.insert(block_name(b, i)?, vec![block_name(&blocks[i + 1], i)?]);
        }
    }
    cfg.insert(
        block_name(&blocks[blocks.len() - 1], blocks.len() - 1)?,
        vec![],
    );

    return Some(cfg);
}

fn parse() -> Result<JsonValue, Box<dyn Error>> {
    let mut contents = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut contents)?;
    let program = json::parse(&contents)?;
    return Ok(program);
}

fn main() {
    let program = parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing stdin: {}", err);
        process::exit(1);
    });

    for func in program["functions"].members() {
        let blocks = basic_blocks(func).unwrap_or_else(|| {
            eprintln!("Problem parsing program");
            process::exit(1);
        });
        println!("BLOCKS");
        for block in blocks.iter() {
            println!("{:?}\n", block);
        }
        println!("-----\n");
        println!("CFG\n{:?}\n---\n", control_flow_graph(&blocks).unwrap());
    }
}
