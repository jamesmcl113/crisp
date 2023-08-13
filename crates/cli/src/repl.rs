use rustyline::validate::MatchingBracketValidator;
use rustyline::{Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers};
use rustyline::{Completer, Helper, Highlighter, Hinter, Validator};

use crisp::{eval::CrispEnv, parse::CrispExpr};

use std::error::Error;

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
}

pub fn run(env: &mut CrispEnv) -> Result<(), Box<dyn Error>> {
    let h = InputValidator {
        brackets: MatchingBracketValidator::new(),
    };

    let mut rl = Editor::new()?;
    rl.set_helper(Some(h));
    rl.bind_sequence(
        KeyEvent(KeyCode::Char('s'), Modifiers::CTRL),
        EventHandler::Simple(Cmd::Insert(1, "\n\t".to_string())),
    );

    loop {
        let input = rl.readline("> ")?;
        let res = crisp::run_program(&input, env)?;

        println!("{res:?}");
    }
}
