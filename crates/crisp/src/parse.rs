#![allow(dead_code)]

use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub enum CrispError {
    SyntaxError(String),
    MissingParen(u32, u32),
    EvalError(String),
}

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
pub enum CrispExpr {
    Symbol(String),
    Number(f32),
    List(Vec<CrispExpr>),
    Fn(CrispFn),
}

pub type CrispResult = Result<CrispExpr, CrispError>;

impl Display for CrispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Number(num) => format!("Number: {num}"),
            Self::Symbol(name) => format!("Symbol: {name}"),
            Self::List(exps) => format!(
                "List: ({:?})",
                exps.iter()
                    .map(|expr| expr.to_string())
                    .collect::<Vec<String>>()
            ),
            Self::Fn(f) => todo!(),
        };

        write!(f, "{msg}")
    }
}

pub fn parse<'a>(tokens: &'a [String]) -> Result<(CrispExpr, &'a [String]), CrispError> {
    let (first, rest) = tokens.split_first().ok_or(CrispError::MissingParen(1, 0))?;

    match first.as_str() {
        "(" => parse_list(rest),
        ")" => Err(CrispError::SyntaxError("Unexpected ')'".to_string())),
        _ => Ok((parse_atom(first)?, rest)),
    }
}

fn parse_list<'a>(tokens: &'a [String]) -> Result<(CrispExpr, &'a [String]), CrispError> {
    let mut exps: Vec<CrispExpr> = vec![];
    let mut xs = tokens;
    loop {
        let (next, rest) = xs
            .split_first()
            .ok_or(CrispError::SyntaxError("Expected a ')'".to_string()))?;

        if next == ")" {
            return Ok((CrispExpr::List(exps), rest));
        }

        let (expr, rest) = parse(xs)?;
        exps.push(expr);
        xs = rest;
    }
}

pub fn parse_floats(tokens: &[CrispExpr]) -> Result<Vec<f32>, CrispError> {
    parse_while(tokens, parse_float)
}

fn parse_float(expr: &CrispExpr) -> Result<f32, CrispError> {
    match expr {
        CrispExpr::Number(x) => Ok(*x),
        _ => Err(CrispError::EvalError("Expected a number".to_string())),
    }
}

fn parse_while<T>(
    tokens: &[CrispExpr],
    predicate: fn(&CrispExpr) -> Result<T, CrispError>,
) -> Result<Vec<T>, CrispError> {
    tokens.iter().map(|x| predicate(x)).collect()
}

fn parse_atom(token: &str) -> Result<CrispExpr, CrispError> {
    let float = token.parse::<f32>();

    match float {
        Ok(f) => Ok(CrispExpr::Number(f)),
        Err(_) => Ok(CrispExpr::Symbol(token.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn parse_basic() {
        let tokens = lexer("(3 5 7)");
        let (expr, rest) = parse(&tokens).unwrap();

        assert!(rest.is_empty());

        assert_eq!(
            expr,
            CrispExpr::List(vec![
                CrispExpr::Number(3.),
                CrispExpr::Number(5.),
                CrispExpr::Number(7.)
            ])
        );
    }

    #[test]
    fn parse_fn_call() {
        let tokens = lexer("(+ 5 7)");
        let (expr, rest) = parse(&tokens).unwrap();

        assert!(rest.is_empty());

        assert_eq!(
            expr,
            CrispExpr::List(vec![
                CrispExpr::Symbol("+".to_string()),
                CrispExpr::Number(5.),
                CrispExpr::Number(7.)
            ])
        );
    }

    #[test]
    fn parse_nested_lists() {
        let tokens = lexer("((-1 10 4) 6 7)");
        let (expr, rest) = parse(&tokens).unwrap();

        assert!(rest.is_empty());

        assert_eq!(
            expr,
            CrispExpr::List(vec![
                CrispExpr::List(vec![
                    CrispExpr::Number(-1.),
                    CrispExpr::Number(10.),
                    CrispExpr::Number(4.)
                ]),
                CrispExpr::Number(6.),
                CrispExpr::Number(7.)
            ])
        );
    }
}
