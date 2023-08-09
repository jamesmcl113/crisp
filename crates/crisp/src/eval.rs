use std::collections::HashMap;

use crate::parse::{parse_floats, CrispError, CrispExpr, CrispFn};

pub struct CrispEnv {
    pub symbols: HashMap<String, CrispExpr>,
}

impl Default for CrispEnv {
    fn default() -> Self {
        let mut symbols: HashMap<String, CrispExpr> = HashMap::new();

        symbols.insert(
            "+".to_string(),
            CrispExpr::Fn(CrispFn(
                |args: &[CrispExpr]| -> Result<CrispExpr, CrispError> {
                    let floats = parse_floats(args)?;

                    Ok(CrispExpr::Number(
                        floats.into_iter().fold(0., |acc, x| acc + x),
                    ))
                },
            )),
        );

        symbols.insert(
            "quote".to_string(),
            CrispExpr::Fn(CrispFn(
                |args: &[CrispExpr]| -> Result<CrispExpr, CrispError> {
                    args.first().cloned().ok_or(CrispError::SyntaxError(
                        "quote takes a single argument".to_string(),
                    ))
                },
            )),
        );

        Self { symbols }
    }
}

pub fn eval(expr: &CrispExpr, env: &mut CrispEnv) -> Result<CrispExpr, CrispError> {
    match expr {
        CrispExpr::List(list) => {
            let (first, rest) = list.split_first().ok_or(CrispError::EvalError(
                "Can't eval an empty list.".to_string(),
            ))?;

            let first_form = eval(first, env)?;
            match first_form {
                CrispExpr::Fn(f) => f.0(rest),
                _ => Err(CrispError::EvalError(
                    "First form must be a function".to_string(),
                )),
            }
        }
        CrispExpr::Symbol(name) => env
            .symbols
            .get(name)
            .cloned()
            .ok_or(CrispError::EvalError(format!("Unknown symbol: {name}"))),
        CrispExpr::Number(_) => Ok(expr.clone()),
        _ => Err(CrispError::EvalError(expr.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_quoted_list() {
        let mut env = CrispEnv::default();
        let expr = CrispExpr::List(vec![
            CrispExpr::Symbol("quote".to_string()),
            CrispExpr::List(vec![
                CrispExpr::Symbol("+".to_string()),
                CrispExpr::Number(3.),
                CrispExpr::Number(4.),
            ]),
        ]);

        assert_eq!(
            eval(&expr, &mut env),
            Ok(CrispExpr::List(vec![
                CrispExpr::Symbol("+".to_string()),
                CrispExpr::Number(3.),
                CrispExpr::Number(4.),
            ]))
        );
    }

    #[test]
    fn eval_list() {
        let mut env = CrispEnv::default();
        let list = CrispExpr::List(vec![
            CrispExpr::Symbol("+".to_string()),
            CrispExpr::Number(3.),
            CrispExpr::Number(4.),
            CrispExpr::Number(5.),
        ]);

        assert_eq!(eval(&list, &mut env), Ok(CrispExpr::Number(12.)));
    }

    #[test]
    fn eval_number() {
        let mut env = CrispEnv::default();
        let expr = CrispExpr::Number(45.);
        assert_eq!(eval(&expr, &mut env), Ok(CrispExpr::Number(45.)));
    }
}
