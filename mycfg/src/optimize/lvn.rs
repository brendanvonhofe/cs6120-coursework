// use std::rc::Rc;
// use std::{collections::HashMap, hash::Hash};

// use crate::core::Value::Int;
// use crate::core::{Instruction, LogicOp, OpCode};

// #[derive(Eq, Hash, PartialEq)]
// pub struct LVNValue(OpCode, usize, usize);

// pub struct LVN {
//     pub table: HashMap<Rc<LVNValue>, (usize, String)>,
//     number_map: HashMap<usize, Rc<LVNValue>>,
//     pub env: HashMap<String, usize>,
// }

// impl LVN {
//     pub fn new() -> LVN {
//         LVN {
//             table: HashMap::new(),
//             number_map: HashMap::new(),
//             env: HashMap::new(),
//         }
//     }

//     pub fn insert_table(&mut self, val: Rc<LVNValue>, var: &str) -> (usize, String) {
//         let num = self.table.len();
//         let lvn = (num, var.to_string());
//         self.table.insert(Rc::clone(&val), lvn.clone());
//         self.number_map.insert(num, Rc::clone(&val));
//         return lvn;
//     }

//     pub fn insert_env(&mut self, var: String, num: usize) {
//         self.env.insert(var, num);
//     }

//     pub fn canonicalize_val(&self, instr: &Instruction) -> Option<Rc<LVNValue>> {
//         let two_arg_val = || -> Option<Rc<LVNValue>> {
//             let (num_one, num_two) = (
//                 *self.env.get(&instr.args.as_ref().unwrap()[0]).unwrap(),
//                 *self.env.get(&instr.args.as_ref().unwrap()[1]).unwrap(),
//             );
//             if num_one > num_two {
//                 return Some(Rc::new(LVNValue(instr.op.clone(), num_two, num_one)));
//             } else {
//                 return Some(Rc::new(LVNValue(instr.op.clone(), num_one, num_two)));
//             }
//         };
//         match &instr.op {
//             OpCode::Arithmetic(_) => {
//                 return two_arg_val();
//             }
//             OpCode::Comparison(_) => {
//                 return two_arg_val();
//             }
//             OpCode::Logic(logic_op) => match logic_op {
//                 LogicOp::Not => {
//                     return Some(Rc::new(LVNValue(
//                         instr.op.clone(),
//                         *self.env.get(&instr.args.as_ref().unwrap()[0]).unwrap(),
//                         0,
//                     )));
//                 }
//                 _ => {
//                     return two_arg_val();
//                 }
//             },
//             OpCode::Const => {
//                 if let Int(val) = &instr.value.unwrap() {
//                     return Some(Rc::new(LVNValue(instr.op.clone(), val, 0)));
//                 } else {
//                     panic!("Const with bool value");
//                 }
//             }
//             _ => {
//                 return None;
//             }
//         }
//     }

//     pub fn replace_args(&self, args_option: &Option<Vec<String>>) -> Option<Vec<String>> {
//         if let Some(args) = args_option {
//             return Some(
//                 args.iter()
//                     .map(|arg| -> String {
//                         let num: &usize = self.env.get(arg).unwrap();
//                         let value = self.number_map.get(num).unwrap();
//                         return self.table.get(value).unwrap().1.to_string();
//                     })
//                     .collect(),
//             );
//         } else {
//             return None;
//         }
//     }
// }
