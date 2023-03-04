use crate::core::{BasicBlock, Function};

impl Function {
    pub fn dead_variable_elim(&self) -> Function {
        let used_vars: Vec<String> = self
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

        return Function {
            name: self.name.clone(),
            args: self.args.clone(),
            ret_type: self.ret_type.clone(),
            blocks: self
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
    }
}
