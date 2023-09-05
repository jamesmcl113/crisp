use std::collections::HashMap;

use crate::{
    lang::{CrispError, CrispExpr, CrispFn, CrispLambda, CrispResult, Primitive},
    parse::{parse_floats, parse_param_list},
};

pub struct CrispEnv<'a> {
    pub symbols: HashMap<String, CrispExpr>,
    pub parent: Option<&'a CrispEnv<'a>>,
}

impl<'a> CrispEnv<'a> {
    pub fn from_parent(parent: &'a CrispEnv) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: &str) -> Option<CrispExpr> {
        match self.symbols.get(name) {
            Some(val) => Some(val.clone()),
            None => match self.parent {
                Some(outer) => outer.get(name),
                None => None,
            },
        }
    }
}

impl<'a> Default for CrispEnv<'a> {
    fn default() -> Self {
        let mut symbols: HashMap<String, CrispExpr> = HashMap::new();

        symbols.insert(
            "+".to_string(),
            CrispExpr::Fn(CrispFn(
                |args: &[CrispExpr]| -> Result<CrispExpr, CrispError> {
                    let floats = parse_floats(args)?;

                    Ok(CrispExpr::Primitive(Primitive::Number(
                        floats.into_iter().fold(0., |acc, x| acc + x),
                    )))
                },
            )),
        );

        symbols.insert(
            "-".to_string(),
            CrispExpr::Fn(CrispFn(
                |args: &[CrispExpr]| -> Result<CrispExpr, CrispError> {
                    let floats = parse_floats(args)?;
                    let (first, rest) = floats.split_first().ok_or(CrispError::EvalError(
                        "- takes at least one argument".to_string(),
                    ))?;

                    Ok(CrispExpr::Primitive(Primitive::Number(
                        rest.into_iter().fold(*first, |acc, &x| acc - x),
                    )))
                },
            )),
        );

        symbols.insert(
            "*".to_string(),
            CrispExpr::Fn(CrispFn(
                |args: &[CrispExpr]| -> Result<CrispExpr, CrispError> {
                    let floats = parse_floats(args)?;

                    Ok(CrispExpr::Primitive(Primitive::Number(
                        floats.into_iter().fold(1., |acc, x| acc * x),
                    )))
                },
            )),
        );

        symbols.insert(
            ">".to_string(),
            CrispExpr::Fn(CrispFn(
                |args: &[CrispExpr]| -> Result<CrispExpr, CrispError> {
                    let floats = parse_floats(args)?;
                    let (first, rest) = floats.split_first().ok_or(CrispError::EvalError(
                        "> takes at least one argument".to_string(),
                    ))?;

                    let second = rest.first().ok_or(CrispError::EvalError(
                        "Expected a second argument".to_string(),
                    ))?;

                    Ok(CrispExpr::Primitive(Primitive::Bool(first > second)))
                },
            )),
        );

        Self {
            symbols,
            parent: None,
        }
    }
}

pub fn eval(expr: &CrispExpr, env: &mut CrispEnv) -> Result<CrispExpr, CrispError> {
    match expr {
        CrispExpr::List(list) => {
            let (first, rest) = list.split_first().ok_or(CrispError::EvalError(
                "Can't eval an empty list.".to_string(),
            ))?;

            match eval_built_in(first, rest, env) {
                Some(res) => res,
                None => {
                    let first_form = eval(first, env)?;
                    let eval_args: Result<Vec<CrispExpr>, CrispError> =
                        rest.iter().map(|arg| eval(arg, env)).collect();
                    match first_form {
                        CrispExpr::Fn(f) => f.0(&eval_args?),
                        CrispExpr::Lambda(lambda) => {
                            let mut lambda_env = CrispEnv::from_parent(&env);

                            let eval_args = eval_args?;

                            if eval_args.len() != lambda.params.len() {
                                Err(CrispError::EvalError(
                                    "Wrong number of arguments were supplied".to_string(),
                                ))
                            } else {
                                eval_args.iter().zip(lambda.params.iter()).for_each(
                                    |(val, name)| {
                                        lambda_env.symbols.insert(name.clone(), val.clone());
                                    },
                                );

                                eval(&lambda.body, &mut lambda_env)
                            }
                        }
                        _ => Err(CrispError::EvalError(
                            "First form must be a function".to_string(),
                        )),
                    }
                }
            }
        }
        CrispExpr::Symbol(name) => env
            .get(name)
            .ok_or(CrispError::EvalError(format!("Unknown symbol: {name}"))),
        CrispExpr::Primitive(_) => Ok(expr.clone()),
        _ => Err(CrispError::EvalError(expr.to_string())),
    }
}

/// Evaluate a built-in expression
fn eval_built_in(expr: &CrispExpr, args: &[CrispExpr], env: &mut CrispEnv) -> Option<CrispResult> {
    match expr {
        CrispExpr::Symbol(name) => match name.as_ref() {
            "begin" => Some(eval_begin(args, env)),
            "def" => Some(eval_def(args, env)),
            "fn" => Some(eval_lambda(args)),
            "if" => Some(eval_if(args, env)),
            "quote" => args.first().map(|list| Ok(list.clone())),
            _ => None,
        },
        _ => None,
    }
}

pub fn eval_begin(args: &[CrispExpr], env: &mut CrispEnv) -> CrispResult {
    let mut last_res: Option<CrispResult> = None;
    for expr in args {
        last_res = match eval(expr, env) {
            Err(e) => return Err(e),
            Ok(res) => Some(Ok(res)),
        }
    }

    last_res.unwrap()
}

/// Evaluate an if expression
pub fn eval_if(args: &[CrispExpr], env: &mut CrispEnv) -> CrispResult {
    if args.len() > 3 {
        return Err(CrispError::EvalError(
            "if takes exactly three arguments".to_string(),
        ));
    }

    let test_form = args
        .first()
        .ok_or(CrispError::EvalError("Expected an expression".to_string()))?;

    let test_res = match eval(test_form, env) {
        Ok(CrispExpr::Primitive(Primitive::Bool(b))) => b,
        _ => {
            return Err(CrispError::EvalError(
                "Test form must evaluate to a boolean".to_string(),
            ))
        }
    };

    let res_arg = if test_res { args.get(1) } else { args.get(2) };

    match res_arg {
        Some(expr) => eval(expr, env),
        None => Err(CrispError::EvalError(
            "missing true or false clause".to_string(),
        )),
    }
}

/// Evaluate a binding definition
pub fn eval_def(args: &[CrispExpr], env: &mut CrispEnv) -> CrispResult {
    if args.len() > 2 {
        return Err(CrispError::EvalError(
            "def takes exactly two arguments".to_string(),
        ));
    }
    let first_form = args
        .first()
        .ok_or(CrispError::EvalError("Expected a name".to_string()))?;

    if let CrispExpr::Symbol(name) = first_form {
        if env.symbols.contains_key(name) {
            return Err(CrispError::EvalError(format!(
                "Variable with name '{name}' already exists"
            )));
        }

        let second_form = args
            .get(1)
            .ok_or(CrispError::EvalError("Expected a value".to_string()))?;

        let val = eval(second_form, env)?;

        env.symbols.insert(name.clone(), val);

        Ok(first_form.clone())
    } else {
        Err(CrispError::EvalError(
            "First argument must be a symbol".to_string(),
        ))
    }
}

/// Evaluate a lambda definition
pub fn eval_lambda(args: &[CrispExpr]) -> CrispResult {
    if args.len() > 2 {
        return Err(CrispError::EvalError(
            "fn takes exactly 2 arguments".to_string(),
        ));
    }

    let params = args.first().ok_or(CrispError::EvalError(
        "Expected a param expression".to_string(),
    ))?;

    let symbol_names = match params {
        CrispExpr::List(xs) => parse_param_list(xs)?,
        _ => return Err(CrispError::EvalError("Params should be a list".to_string())),
    };

    let body = args.get(1).unwrap();

    Ok(CrispExpr::Lambda(CrispLambda {
        params: symbol_names,
        body: Box::new(body.clone()),
    }))
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
                CrispExpr::Primitive(Primitive::Number(3.)),
                CrispExpr::Primitive(Primitive::Number(4.)),
            ]),
        ]);

        assert_eq!(
            eval(&expr, &mut env),
            Ok(CrispExpr::List(vec![
                CrispExpr::Symbol("+".to_string()),
                CrispExpr::Primitive(Primitive::Number(3.)),
                CrispExpr::Primitive(Primitive::Number(4.)),
            ]),)
        );
    }

    #[test]
    fn eval_list() {
        let mut env = CrispEnv::default();
        let list = CrispExpr::List(vec![
            CrispExpr::Symbol("+".to_string()),
            CrispExpr::Primitive(Primitive::Number(3.)),
            CrispExpr::Primitive(Primitive::Number(4.)),
            CrispExpr::Primitive(Primitive::Number(5.)),
        ]);

        assert_eq!(
            eval(&list, &mut env),
            Ok(CrispExpr::Primitive(Primitive::Number(12.)))
        );
    }

    #[test]
    fn eval_number() {
        let mut env = CrispEnv::default();
        let expr = CrispExpr::Primitive(Primitive::Number(45.));
        assert_eq!(
            eval(&expr, &mut env),
            Ok(CrispExpr::Primitive(Primitive::Number(45.)))
        );
    }
}
