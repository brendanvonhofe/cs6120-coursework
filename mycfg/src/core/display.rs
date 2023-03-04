use std::fmt;

use crate::core::{BasicBlock, Function, Instruction, Program, Type};

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
