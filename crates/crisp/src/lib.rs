use eval::{eval, CrispEnv};
use lang::CrispResult;
use parse::parse;

pub mod eval;
pub mod lang;
pub mod parse;

pub fn lexer(s: &str) -> Vec<String> {
    s.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}

pub fn run_program(prog: &str, env: &mut CrispEnv) -> CrispResult {
    let tokens = lexer(prog);
    let res = parse(&tokens)?;

    eval(&res.0, env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_basic() {
        let text = "(3 4 5)";
        let tokens = lexer(text);

        assert_eq!(tokens, vec!["(", "3", "4", "5", ")"]);
    }
}
