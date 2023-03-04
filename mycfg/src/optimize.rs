use crate::core::BasicBlock;

impl BasicBlock {
    pub fn dead_variable_elim(&self) -> BasicBlock {
        let used_vars: Vec<String> = self
            .instructions
            .iter()
            .map(|x| -> Vec<String> {
                match &x.args {
                    Some(list) => list.to_vec(),
                    None => vec![],
                }
            })
            .flat_map(|x| x)
            .collect();
        return BasicBlock {
            name: self.name.clone(),
            instructions: self
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
        };
    }
}
