use json;
use json::JsonValue;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Read;
use std::process;

const TERMINATORS: [&str; 3] = ["jmp", "br", "ret"];

struct BasicBlock<'a> {
    name: String,
    instrs: Vec<&'a JsonValue>,
    len: usize,
}

impl<'a> fmt::Debug for BasicBlock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.name)?;
        for instr in self.instrs.iter() {
            write!(f, "  {}", instr)?;
        }
        Ok(())
    }
}

fn finalize_block(label: Option<String>, instrs: Vec<&JsonValue>, id: u32) -> BasicBlock {
    let name;
    match label {
        Some(s) => name = s.clone(),
        None => name = format!("block_{}", id),
    };
    return BasicBlock {
        name: name,
        len: instrs.len(),
        instrs: instrs,
    };
}

fn basic_blocks(func: &JsonValue) -> Option<Vec<BasicBlock>> {
    let mut blocks: Vec<BasicBlock> = vec![];
    let mut block: Vec<&JsonValue> = vec![];
    let mut cur_label: Option<String> = Option::None;
    let mut idx = 0;

    for op in func["instrs"].members() {
        if op.has_key("op") {
            block.push(op);
            if TERMINATORS.contains(&op["op"].as_str()?) {
                blocks.push(finalize_block(cur_label, block.clone(), idx));
                cur_label = None;
                idx += 1;
            } else {
                continue;
            }
        } else if op.has_key("label") {
            if !block.is_empty() {
                blocks.push(finalize_block(cur_label, block.clone(), idx));
                idx += 1;
            }
            cur_label = Some(String::from(op["label"].as_str()?));
        }
        block.clear();
    }

    if !block.is_empty() {
        blocks.push(finalize_block(cur_label, block.clone(), idx));
    }

    return Some(blocks);
}

fn parse() -> Result<JsonValue, Box<dyn Error>> {
    let mut contents = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut contents)?;
    let program = json::parse(&contents)?;
    return Ok(program);
}

fn control_flow_graph<'a>(
    blocks: &'a Vec<BasicBlock<'a>>,
) -> Option<HashMap<&'a str, Vec<&'a str>>> {
    let mut cfg: HashMap<&str, Vec<&str>> = HashMap::new();

    for i in 0..blocks.len() - 1 {
        let b = &blocks[i];
        let last = b.instrs[b.len - 1];
        if last["op"].as_str()? == "jmp" {
            cfg.insert(&b.name, vec![last["labels"].members().next()?.as_str()?]);
        } else if last["op"].as_str()? == "br" {
            let mut labels = last["labels"].members();
            cfg.insert(
                &b.name,
                vec![labels.next()?.as_str()?, labels.next()?.as_str()?],
            );
        } else {
            cfg.insert(&b.name, vec![&blocks[i + 1].name]);
        }
    }
    cfg.insert(&blocks[blocks.len() - 1].name, vec![]);

    return Some(cfg);
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
