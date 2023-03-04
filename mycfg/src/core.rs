mod display;

#[derive(Clone, Debug)]
pub enum Type {
    Int,
    Bool,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(isize),
    Bool(bool),
}

#[derive(Clone, PartialEq, Debug)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ComparisonOp {
    Eq,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LogicOp {
    Not,
    And,
    Or,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ControlOp {
    Jmp,
    Br,
    Call,
    Ret,
}

#[derive(Clone, PartialEq, Debug)]
pub enum MiscOp {
    Id,
    Print,
    Nop,
}

#[derive(Clone, PartialEq, Debug)]
pub enum OpCode {
    Const,
    Arithmetic(ArithmeticOp),
    Comparison(ComparisonOp),
    Logic(LogicOp),
    Control(ControlOp),
    Misc(MiscOp),
}

#[derive(Clone)]
pub struct Instruction {
    pub op: OpCode,
    pub dst: Option<String>,
    pub dst_type: Option<Type>,
    pub args: Option<Vec<String>>,
    pub funcs: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub value: Option<Value>,
}

#[derive(Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

pub struct Function {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub ret_type: Option<Type>,
    pub blocks: Vec<BasicBlock>,
}

pub struct Program {
    pub functions: Vec<Function>,
}
