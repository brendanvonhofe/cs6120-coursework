use json;
use json::JsonValue;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
// use std::fs;
use std::io::Read;
use std::process;
use std::{io, vec};

const TERMINATORS: [OpCode; 3] = [
    OpCode::Control(ControlOp::Jmp),
    OpCode::Control(ControlOp::Br),
    OpCode::Control(ControlOp::Ret),
];

#[derive(Clone, Debug)]
enum Type {
    Int,
    Bool,
}

#[derive(Clone, Debug)]
enum Value {
    Int(isize),
    Bool(bool),
}

#[derive(Clone, PartialEq, Debug)]
enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, PartialEq, Debug)]
enum ComparisonOp {
    Eq,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Clone, PartialEq, Debug)]
enum LogicOp {
    Not,
    And,
    Or,
}

#[derive(Clone, PartialEq, Debug)]
enum ControlOp {
    Jmp,
    Br,
    Call,
    Ret,
}

#[derive(Clone, PartialEq, Debug)]
enum MiscOp {
    Id,
    Print,
    Nop,
}

#[derive(Clone, PartialEq, Debug)]
enum OpCode {
    Const,
    Arithmetic(ArithmeticOp),
    Comparison(ComparisonOp),
    Logic(LogicOp),
    Control(ControlOp),
    Misc(MiscOp),
}

#[derive(Clone)]
struct Instruction {
    op: OpCode,
    dst: Option<String>,
    dst_type: Option<Type>,
    args: Option<Vec<String>>,
    funcs: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    value: Option<Value>,
}

#[derive(Clone)]
struct BasicBlock {
    name: String,
    instructions: Vec<Instruction>,
}

struct Function {
    name: String,
    args: Vec<(String, Type)>,
    ret_type: Option<Type>,
    blocks: Vec<BasicBlock>,
}

struct Program {
    functions: HashMap<String, Function>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, function) in self.functions.keys().enumerate() {
            write!(f, "{}", self.functions[function])?;
            if i != self.functions.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret_str = "void";
        if let Some(ret_type) = &self.ret_type {
            match ret_type {
                Type::Int => ret_str = "Int",
                Type::Bool => ret_str = "Bool",
            }
        }
        write!(f, "@{}(", self.name)?;
        for (i, (arg_name, arg_type)) in self.args.iter().enumerate() {
            write!(f, "{}: {:?}", arg_name, arg_type)?;
            if i != self.args.len() - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "): {} {{\n", ret_str)?;
        for block in self.blocks.iter() {
            write!(f, "{}", block)?;
        }
        write!(f, "}}\n")?;
        Ok(())
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ".{}\n", self.name)?;
        for instr in self.instructions.iter() {
            write!(f, "    {}\n", instr)?;
        }
        Ok(())
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.op)?;
        Ok(())
    }
}

// fn control_flow_graph(blocks: &Vec<Vec<&JsonValue>>) -> Option<HashMap<String, Vec<String>>> {
//     let mut cfg: HashMap<String, Vec<String>> = HashMap::new();

//     for i in 0..blocks.len() - 1 {
//         let b = &blocks[i];
//         let last = b[b.len() - 1];
//         if last["op"].as_str()? == "jmp" || last["op"].as_str()? == "br" {
//             cfg.insert(
//                 block_name(b, i)?,
//                 Vec::from_iter(
//                     last["labels"]
//                         .members()
//                         .map(|x| -> String { String::from(x.as_str().unwrap()) }),
//                 ),
//             );
//         } else {
//             cfg.insert(block_name(b, i)?, vec![block_name(&blocks[i + 1], i)?]);
//         }
//     }
//     cfg.insert(
//         block_name(&blocks[blocks.len() - 1], blocks.len() - 1)?,
//         vec![],
//     );

//     return Some(cfg);
// }

fn is_terminator(instr: &Instruction) -> bool {
    if TERMINATORS.contains(&instr.op) {
        true
    } else {
        false
    }
}

fn parse_function_args(json: &JsonValue) -> Vec<(String, Type)> {
    if json.has_key("args") {
        json["args"]
            .members()
            .map(|arg| -> (String, Type) {
                (
                    String::from(arg["name"].as_str().unwrap()),
                    parse_type(&arg["type"]),
                )
            })
            .collect()
    } else {
        vec![]
    }
}

fn parse_type(json: &JsonValue) -> Type {
    if json.as_str().unwrap() == "int" {
        Type::Int
    } else if json.as_str().unwrap() == "bool" {
        Type::Bool
    } else {
        panic!("Invalid json passed to resolveType: {:#?}", json);
    }
}

fn parse_op_code(json: &JsonValue) -> OpCode {
    match json.as_str().unwrap() {
        "const" => OpCode::Const,
        "add" => OpCode::Arithmetic(ArithmeticOp::Add),
        "sub" => OpCode::Arithmetic(ArithmeticOp::Sub),
        "mul" => OpCode::Arithmetic(ArithmeticOp::Mul),
        "div" => OpCode::Arithmetic(ArithmeticOp::Div),
        "eq" => OpCode::Comparison(ComparisonOp::Eq),
        "lt" => OpCode::Comparison(ComparisonOp::Lt),
        "gt" => OpCode::Comparison(ComparisonOp::Gt),
        "le" => OpCode::Comparison(ComparisonOp::Le),
        "ge" => OpCode::Comparison(ComparisonOp::Ge),
        "not" => OpCode::Logic(LogicOp::Not),
        "and" => OpCode::Logic(LogicOp::And),
        "or" => OpCode::Logic(LogicOp::Or),
        "jmp" => OpCode::Control(ControlOp::Jmp),
        "br" => OpCode::Control(ControlOp::Br),
        "call" => OpCode::Control(ControlOp::Call),
        "ret" => OpCode::Control(ControlOp::Ret),
        "id" => OpCode::Misc(MiscOp::Id),
        "print" => OpCode::Misc(MiscOp::Print),
        "nop" => OpCode::Misc(MiscOp::Nop),
        _ => {
            panic!("Invalid JSON passed to parse_op_code: {}", json);
        }
    }
}

fn parse_json_str_arr(json: &JsonValue) -> Vec<String> {
    json.members()
        .map(|member| -> String { String::from(member.as_str().unwrap()) })
        .collect()
}

fn parse_instruction(json: &JsonValue) -> Instruction {
    Instruction {
        op: parse_op_code(&json["op"]),
        dst: if json.has_key("dest") {
            Some(String::from(json["dest"].as_str().unwrap()))
        } else {
            None
        },
        dst_type: if json.has_key("type") {
            Some(parse_type(&json["type"]))
        } else {
            None
        },
        args: if json.has_key("args") {
            Some(parse_json_str_arr(&json["args"]))
        } else {
            None
        },
        funcs: if json.has_key("funcs") {
            Some(parse_json_str_arr(&json["funcs"]))
        } else {
            None
        },
        labels: if json.has_key("labels") {
            Some(parse_json_str_arr(&json["labels"]))
        } else {
            None
        },
        value: if json.has_key("value") {
            match json["value"].is_boolean() {
                true => Some(Value::Bool(json["value"].as_bool().unwrap())),
                false => Some(Value::Int(json["value"].as_isize().unwrap())),
            }
        } else {
            None
        },
    }
}

struct BlockGen {
    blocks: Vec<BasicBlock>,
    instructions: Vec<Instruction>,
    name: String,
}

impl BlockGen {
    fn finalize_block(&mut self) {
        if !self.instructions.is_empty() {
            self.blocks.push(BasicBlock {
                name: if self.name.is_empty() {
                    format!("b{}", self.blocks.len())
                } else {
                    self.name.clone()
                },
                instructions: self.instructions.clone(),
            });
            self.instructions.clear();
            self.name.clear()
        }
    }

    fn push_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    fn set_cur_name(&mut self, name: String) {
        self.name = name;
    }

    fn yield_blocks(&self) -> Vec<BasicBlock> {
        self.blocks.clone()
    }
}

fn parse_basic_blocks(json: &JsonValue) -> Vec<BasicBlock> {
    let mut block_gen: BlockGen = BlockGen {
        blocks: vec![],
        instructions: vec![],
        name: String::new(),
    };

    for op in json.members() {
        if op.has_key("op") {
            let instr: Instruction = parse_instruction(&op);
            block_gen.push_instruction(instr.clone());
            if is_terminator(&instr) {
                block_gen.finalize_block();
            }
        } else if op.has_key("label") {
            block_gen.finalize_block();
            block_gen.set_cur_name(String::from(op["label"].as_str().unwrap()));
        }
    }
    block_gen.finalize_block();

    block_gen.yield_blocks()
}

fn parse_function(json: &JsonValue) -> Function {
    Function {
        name: String::from(json["name"].as_str().unwrap()),
        args: parse_function_args(&json),
        ret_type: if json.has_key("type") {
            Some(parse_type(&json["type"]))
        } else {
            None
        },
        blocks: parse_basic_blocks(&json["instrs"]),
    }
}

fn parse_program(json: &JsonValue) -> Program {
    Program {
        functions: json["functions"]
            .members()
            .map(|func| -> (String, Function) {
                (
                    String::from(func["name"].as_str().unwrap()),
                    parse_function(func),
                )
            })
            .collect(),
    }
}

fn parse_stdin() -> Result<JsonValue, Box<dyn Error>> {
    let mut contents = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut contents)?;
    let program = json::parse(&contents)?;
    return Ok(program);
}

// fn parse_file(filename: &str) -> Result<JsonValue, Box<dyn Error>> {
//     let contents = String::from(fs::read_to_string(filename)?);
//     let program = json::parse(&contents)?;
//     return Ok(program);
// }

fn main() {
    let json = parse_stdin().unwrap_or_else(|err| {
        eprintln!("Problem parsing stdin: {}", err);
        process::exit(1);
    });

    // let json = parse_file("/Users/brendan/Desktop/cs6120/mycfg/test/fib2seven.json")
    //     .unwrap_or_else(|err| {
    //         eprintln!("Problem parsing stdin: {}", err);
    //         process::exit(1);
    //     });

    let prog = parse_program(&json);

    print!("{}", prog);

    // for func in program["functions"].members() {
    //     let blocks = basic_blocks(func).unwrap_or_else(|| {
    //         eprintln!("Problem parsing program");
    //         process::exit(1);
    //     });

    //     println!("digraph {} {{", func["name"].as_str().unwrap());
    //     let cfg = control_flow_graph(&blocks).unwrap();
    //     let mut sorted_keys: Vec<&String> = cfg.keys().collect();
    //     sorted_keys.sort();
    //     for &key in &sorted_keys {
    //         println!("  {};", key);
    //     }
    //     for &key in &sorted_keys {
    //         for succ in cfg[key].iter() {
    //             println!("  {key} -> {succ};");
    //         }
    //     }
    //     println!("}}");
    //     break;
    // }
}
