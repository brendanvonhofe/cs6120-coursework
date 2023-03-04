use crate::core::{BasicBlock, Instruction};

pub struct BlockGen {
    pub blocks: Vec<BasicBlock>,
    pub instructions: Vec<Instruction>,
    pub name: String,
}

impl BlockGen {
    pub fn finalize_block(&mut self) {
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

    pub fn push_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    pub fn set_cur_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn yield_blocks(&self) -> Vec<BasicBlock> {
        self.blocks.clone()
    }
}
