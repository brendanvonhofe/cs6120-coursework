mod blockgen;

use std::collections::HashMap;
use std::vec;

use json;
use json::JsonValue;

use crate::core::{
    ArithmeticOp, BasicBlock, ComparisonOp, ControlOp, Function, Instruction, LogicOp, MiscOp,
    OpCode, Program, Type, Value,
};
use crate::parser::blockgen::BlockGen;

const TERMINATORS: [OpCode; 3] = [
    OpCode::Control(ControlOp::Jmp),
    OpCode::Control(ControlOp::Br),
    OpCode::Control(ControlOp::Ret),
];

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
            if TERMINATORS.contains(&instr.op) {
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

pub fn parse_program(json: &JsonValue) -> Program {
    Program {
        functions: json["functions"]
            .members()
            .map(|func| -> Function { parse_function(func) })
            .collect(),
    }
}

pub fn control_flow_graph(func: &Function) -> HashMap<String, Vec<String>> {
    let mut cfg: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..func.blocks.len() - 1 {
        let block = &func.blocks[i];
        let last = &block.instructions[block.instructions.len() - 1];
        if last.op == OpCode::Control(ControlOp::Jmp) || last.op == OpCode::Control(ControlOp::Br) {
            cfg.insert(block.name.clone(), last.labels.as_ref().unwrap().clone());
        } else {
            cfg.insert(block.name.clone(), vec![func.blocks[i + 1].name.clone()]);
        }
    }
    cfg.insert(func.blocks[func.blocks.len() - 1].name.clone(), vec![]);

    return cfg;
}
