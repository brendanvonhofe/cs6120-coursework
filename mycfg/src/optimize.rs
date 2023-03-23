mod lvn;

use std::collections::HashMap;
use std::rc::Rc;

use crate::core::{BasicBlock, Function, Instruction, MiscOp, OpCode};
// use crate::optimize::lvn::LVN;

impl Function {
    pub fn dead_variable_elim(&self) -> Function {
        let mut last = (*self).clone();
        loop {
            let used_vars: Vec<String> = last
                .blocks
                .iter()
                .flat_map(|block| {
                    block.instructions.iter().flat_map(|x| -> Vec<String> {
                        match &x.args {
                            Some(list) => list.to_vec(),
                            None => vec![],
                        }
                    })
                })
                .collect();

            let func = Function {
                name: last.name.clone(),
                args: last.args.clone(),
                ret_type: last.ret_type.clone(),
                blocks: last
                    .blocks
                    .iter()
                    .map(|block| BasicBlock {
                        name: block.name.clone(),
                        instructions: block
                            .instructions
                            .iter()
                            .filter(|&x| -> bool {
                                if let Some(dst) = &x.dst {
                                    if used_vars.contains(&dst) {
                                        return true;
                                    }
                                    return false;
                                }
                                return true;
                            })
                            .map(|x| x.clone())
                            .collect(),
                    })
                    .collect(),
            };

            if func == last {
                break;
            }
            last = func;
        }
        return last;
    }
}

impl BasicBlock {
    pub fn dead_store_elim(&self) -> BasicBlock {
        let mut last = self.clone();
        loop {
            let mut block = last.clone();
            let mut unused_defs: HashMap<&String, usize> = HashMap::new();

            for (i, instr) in last.instructions.iter().enumerate() {
                // Check for variable uses
                if let Some(args) = &instr.args {
                    for var in args.iter() {
                        if unused_defs.contains_key(&var) {
                            unused_defs.remove(var);
                        }
                    }
                }
                // Check for variable definitions
                if let Some(dst) = &instr.dst {
                    if unused_defs.contains_key(dst) {
                        block.instructions.remove(unused_defs[dst]);
                    }
                    unused_defs.insert(dst, i);
                }
            }

            if block == last {
                break;
            }
            last = block;
        }
        return last;
    }

    // pub fn local_value_numbering(&self) -> BasicBlock {
    //     let mut lvn: LVN = LVN::new();

    //     let will_be_overwritten = |dest: &str, start: usize| -> bool {
    //         for i in start..self.instructions.len() {
    //             if let Some(new_dest) = &self.instructions[i].dst {
    //                 if new_dest == dest {
    //                     return true;
    //                 }
    //             }
    //         }
    //         return false;
    //     };

    //     return BasicBlock {
    //         name: self.name.clone(),
    //         instructions: self
    //             .instructions
    //             .iter()
    //             .enumerate()
    //             .map(|(i, instr)| -> Instruction {
    //                 if let Some(value) = lvn.canonicalize_val(instr) {
    //                     let mut entry: (usize, String);
    //                     let new_instr: Instruction;

    //                     if lvn.table.contains_key(&value) {
    //                         entry = lvn.table.get(&value).unwrap().clone();
    //                         new_instr = Instruction {
    //                             op: OpCode::Misc(MiscOp::Id),
    //                             dst: instr.dst.clone(),
    //                             dst_type: instr.dst_type.clone(),
    //                             args: Some(vec![entry.1.to_string()]),
    //                             funcs: None,
    //                             labels: None,
    //                             value: None,
    //                         };
    //                     } else {
    //                         entry =
    //                             lvn.insert_table(Rc::clone(&value), &instr.dst.as_ref().unwrap());

    //                         if will_be_overwritten(&entry.1, i) {
    //                             entry.1 = entry.1 + "_prime";
    //                         }

    //                         new_instr = Instruction {
    //                             op: instr.op.clone(),
    //                             dst: Some(entry.1.to_string()),
    //                             dst_type: instr.dst_type.clone(),
    //                             args: lvn.replace_args(&instr.args),
    //                             funcs: instr.funcs.clone(),
    //                             labels: instr.labels.clone(),
    //                             value: instr.value.clone(),
    //                         };
    //                     }
    //                     lvn.insert_env(entry.1.to_string(), entry.0.try_into().unwrap());
    //                     return new_instr;
    //                 } else {
    //                     let mut new_instr = instr.clone();
    //                     new_instr.args = lvn.replace_args(&instr.args);
    //                     return new_instr;
    //                 }
    //             })
    //             .collect(),
    //     };
    // }
}
