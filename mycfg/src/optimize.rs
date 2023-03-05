use std::collections::HashMap;

use crate::core::{BasicBlock, Function};

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
}
