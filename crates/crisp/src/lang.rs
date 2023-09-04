use crate::{
    eval::{eval, CrispEnv},
    parse::{parse_param_list, CrispError, CrispExpr, CrispLambda, CrispResult},
};

pub fn def(args: &[CrispExpr], env: &mut CrispEnv) -> CrispResult {
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

pub fn lambda(args: &[CrispExpr], env: &mut CrispEnv) -> CrispResult {
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
