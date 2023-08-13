use crate::{
    eval::{self, eval, CrispEnv},
    parse::{CrispError, CrispExpr, CrispResult},
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
