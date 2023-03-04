use std::fmt;

use crate::core::OpCode::*;
use crate::core::Value::*;
use crate::core::{
    BasicBlock, ControlOp, Function, Instruction, LogicOp, MiscOp, Program, Type, Value,
};
use crate::parser::control_flow_graph;

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, function) in self.functions.iter().enumerate() {
            write!(f, "{}", function)?;
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
        write!(f, ".{}:\n", self.name)?;
        for instr in self.instructions.iter() {
            write!(f, "    {}\n", instr)?;
        }
        Ok(())
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.op {
            Const => {
                write!(
                    f,
                    "{}: {:?} = {:?} {};",
                    self.dst.as_ref().unwrap(),
                    self.dst_type.as_ref().unwrap(),
                    Const,
                    self.value.as_ref().unwrap()
                )?;
            }
            Arithmetic(aop) => {
                write!(
                    f,
                    "{}: {:?} = {:?} {} {};",
                    self.dst.as_ref().unwrap(),
                    self.dst_type.as_ref().unwrap(),
                    aop,
                    self.args.as_ref().unwrap()[0],
                    self.args.as_ref().unwrap()[1]
                )?;
            }
            Comparison(compop) => {
                write!(
                    f,
                    "{}: {:?} = {:?} {} {};",
                    self.dst.as_ref().unwrap(),
                    self.dst_type.as_ref().unwrap(),
                    compop,
                    self.args.as_ref().unwrap()[0],
                    self.args.as_ref().unwrap()[1]
                )?;
            }
            Logic(lop) => match lop {
                LogicOp::Not => {
                    write!(
                        f,
                        "{}: {:?} = {:?} {};",
                        self.dst.as_ref().unwrap(),
                        self.dst_type.as_ref().unwrap(),
                        lop,
                        self.args.as_ref().unwrap()[0],
                    )?;
                }
                _ => {
                    write!(
                        f,
                        "{}: {:?} = {:?} {} {};",
                        self.dst.as_ref().unwrap(),
                        self.dst_type.as_ref().unwrap(),
                        lop,
                        self.args.as_ref().unwrap()[0],
                        self.args.as_ref().unwrap()[1]
                    )?;
                }
            },
            Control(cop) => match cop {
                ControlOp::Jmp => {
                    write!(
                        f,
                        "{:?} .{};",
                        ControlOp::Jmp,
                        self.labels.as_ref().unwrap()[0]
                    )?;
                }
                ControlOp::Br => {
                    write!(
                        f,
                        "{:?} {} .{} .{};",
                        ControlOp::Br,
                        self.args.as_ref().unwrap()[0],
                        self.labels.as_ref().unwrap()[0],
                        self.labels.as_ref().unwrap()[1]
                    )?;
                }
                ControlOp::Call => {
                    if let Some(dst) = &self.dst {
                        write!(f, "{}: {:?} = ", dst, self.dst_type.as_ref().unwrap())?;
                    }
                    write!(
                        f,
                        "{:?} @{} {};",
                        ControlOp::Call,
                        self.funcs.as_ref().unwrap()[0],
                        self.args.as_ref().unwrap().join(" ")
                    )?;
                }
                ControlOp::Ret => {
                    if let Some(args) = &self.args {
                        write!(f, "ret {};", args[0])?;
                    } else {
                        write!(f, "ret;")?;
                    }
                }
            },
            Misc(mop) => match &mop {
                MiscOp::Id => {
                    write!(f, "{:?} {};", MiscOp::Id, self.args.as_ref().unwrap()[0])?;
                }
                MiscOp::Print => {
                    write!(
                        f,
                        "{:?} {};",
                        MiscOp::Print,
                        self.args.as_ref().unwrap().join(" ")
                    )?;
                }
                MiscOp::Nop => {
                    write!(f, "{:?};", MiscOp::Nop)?;
                }
            },
        }
        Ok(())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Int(int) => {
                write!(f, "{}", int)?;
            }
            Bool(bool) => {
                if *bool {
                    write!(f, "true")?;
                } else {
                    write!(f, "false")?;
                }
            }
        }
        Ok(())
    }
}

impl Program {
    pub fn print_graphviz(&self) {
        for func in self.functions.iter() {
            println!("digraph {} {{", func.name);
            let cfg = control_flow_graph(func);
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
}
