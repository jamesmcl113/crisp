mod eval;
mod parse;

pub fn lexer(s: &str) -> Vec<String> {
    s.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
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
