#![allow(dead_code)]

use crate::lang::{CrispError, CrispExpr, CrispResult, Primitive};

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

pub fn parse_param_list(params: &[CrispExpr]) -> Result<Vec<String>, CrispError> {
    parse_while(params, parse_symbol)
        .map_err(|_| CrispError::EvalError("Param list must contain only symbols".to_string()))
}

fn parse_symbol(token: &CrispExpr) -> Result<String, CrispError> {
    match token {
        CrispExpr::Symbol(name) => Ok(name.clone()),
        _ => Err(CrispError::EvalError("Expected a symbol".to_string())),
    }
}

fn parse_float(expr: &CrispExpr) -> Result<f32, CrispError> {
    match expr {
        CrispExpr::Primitive(Primitive::Number(x)) => Ok(*x),
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
        Ok(f) => Ok(CrispExpr::Primitive(Primitive::Number(f))),
        Err(_) => match token {
            "true" => Ok(CrispExpr::Primitive(Primitive::Bool(true))),
            "false" => Ok(CrispExpr::Primitive(Primitive::Bool(false))),
            _ => Ok(CrispExpr::Symbol(token.to_string())),
        },
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
                CrispExpr::Primitive(Primitive::Number(3.)),
                CrispExpr::Primitive(Primitive::Number(5.)),
                CrispExpr::Primitive(Primitive::Number(7.))
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
                CrispExpr::Primitive(Primitive::Number(5.)),
                CrispExpr::Primitive(Primitive::Number(7.))
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
                    CrispExpr::Primitive(Primitive::Number(-1.)),
                    CrispExpr::Primitive(Primitive::Number(10.)),
                    CrispExpr::Primitive(Primitive::Number(4.))
                ]),
                CrispExpr::Primitive(Primitive::Number(6.)),
                CrispExpr::Primitive(Primitive::Number(7.))
            ])
        );
    }
}
