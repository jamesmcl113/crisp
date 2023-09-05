use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Clone)]
pub enum CrispError {
    SyntaxError(String),
    MissingParen(u32, u32),
    EvalError(String),
}

impl std::error::Error for CrispError {}

impl Display for CrispError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::SyntaxError(msg) => format!("syntax error: {msg}"),
            Self::MissingParen(line, char) => format!("missing paren at line {line}, char {char}"),
            Self::EvalError(msg) => format!("error evaluating expr: {msg}"),
        };

        write!(f, "{msg}")
    }
}

#[derive(Clone)]
pub struct CrispFn(pub fn(&[CrispExpr]) -> Result<CrispExpr, CrispError>);

#[derive(Debug, Clone, PartialEq)]
pub struct CrispLambda {
    pub params: Vec<String>,
    pub body: Box<CrispExpr>,
}

impl Debug for CrispFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function")
    }
}

impl PartialEq for CrispFn {
    fn eq(&self, other: &Self) -> bool {
        let x = self as *const _;
        let y = other as *const _;
        x == y
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Number(f32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrispExpr {
    Symbol(String),
    Primitive(Primitive),
    List(Vec<CrispExpr>),
    Fn(CrispFn),
    Lambda(CrispLambda),
}

impl CrispExpr {
    pub fn is_symbol(&self) -> bool {
        match self {
            Self::Symbol(_) => true,
            _ => false,
        }
    }
}

pub type CrispResult = Result<CrispExpr, CrispError>;

impl Display for CrispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Primitive(val) => match val {
                Primitive::Bool(b) => format!("{}", b),
                Primitive::Number(n) => format!("{}", n),
            },
            Self::Symbol(name) => format!("Symbol: {name}"),
            Self::List(exps) => format!(
                "List: ({:?})",
                exps.iter()
                    .map(|expr| expr.to_string())
                    .collect::<Vec<String>>()
            ),
            Self::Fn(f) => todo!(),
            Self::Lambda(f) => todo!(),
        };

        write!(f, "{msg}")
    }
}
