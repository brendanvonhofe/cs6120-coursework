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
    let mut args: Vec<(String, Type)> = vec![];
    if json.has_key("args") {
        for arg in json["args"].members() {
            args.push((
                String::from(arg["name"].as_str().unwrap()),
                parse_type(&arg["type"]),
            ))
        }
    }
    args
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
    let mut string_array: Vec<String> = vec![];
    for member in json.members() {
        string_array.push(String::from(member.as_str().unwrap()));
    }
    string_array
}

fn parse_instruction(json: &JsonValue) -> Instruction {
    let op: OpCode = parse_op_code(&json["op"]);
    let mut dst: Option<String> = None;
    let mut dst_type: Option<Type> = None;
    let mut args: Option<Vec<String>> = None;
    let mut funcs: Option<Vec<String>> = None;
    let mut labels: Option<Vec<String>> = None;
    let mut value: Option<Value> = None;
    if json.has_key("dst") {
        dst = Some(String::from(json["dest"].as_str().unwrap()));
    }
    if json.has_key("type") {
        dst_type = Some(parse_type(&json["type"]));
    }
    if json.has_key("args") {
        args = Some(parse_json_str_arr(&json["args"]));
    }
    if json.has_key("funcs") {
        funcs = Some(parse_json_str_arr(&json["funcs"]));
    }
    if json.has_key("labels") {
        labels = Some(parse_json_str_arr(&json["labels"]));
    }
    if json.has_key("value") {
        match dst_type.as_ref().unwrap() {
            Type::Bool => value = Some(Value::Bool(json["value"].as_bool().unwrap())),
            Type::Int => value = Some(Value::Int(json["value"].as_isize().unwrap())),
        }
    }
    Instruction {
        op,
        dst,
        dst_type,
        args,
        funcs,
        labels,
        value,
    }
}

fn parse_basic_blocks(json: &JsonValue) -> Vec<BasicBlock> {
    let mut blocks: Vec<BasicBlock> = vec![];
    let mut instructions: Vec<Instruction> = vec![];
    let mut name: String = String::new();

    for op in json.members() {
        if op.has_key("op") {
            let instr: Instruction = parse_instruction(&op);
            if is_terminator(&instr) {
                instructions.push(instr);
                if name.is_empty() {
                    name = format!("b{}", blocks.len());
                }
                blocks.push(BasicBlock {
                    name: name.clone(),
                    instructions: instructions.clone(),
                });
                instructions.clear();
                name.clear()
            } else {
                instructions.push(instr);
            }
        } else if op.has_key("label") {
            if !instructions.is_empty() {
                if name.is_empty() {
                    name = format!("b{}", blocks.len());
                }
                blocks.push(BasicBlock {
                    name: name.clone(),
                    instructions: instructions.clone(),
                });
                instructions.clear();
                name.clear()
            }
            name = String::from(op["label"].as_str().unwrap());
        }
    }

    if !instructions.is_empty() {
        if name.is_empty() {
            name = format!("b{}", blocks.len());
        }
        blocks.push(BasicBlock {
            name: name,
            instructions: instructions.clone(),
        });
    }

    blocks
}

fn parse_function(json: &JsonValue) -> Function {
    let mut ret_type: Option<Type> = None;
    if json.has_key("type") {
        ret_type = Some(parse_type(&json["type"]));
    }
    Function {
        name: String::from(json["name"].as_str().unwrap()),
        args: parse_function_args(&json),
        ret_type: ret_type,
        blocks: parse_basic_blocks(&json["instrs"]),
    }
}

fn parse_program(json: &JsonValue) -> Program {
    let mut functions: HashMap<String, Function> = HashMap::new();
    for func in json["functions"].members() {
        functions.insert(
            String::from(func["name"].as_str().unwrap()),
            parse_function(func),
        );
    }
    Program { functions }
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
