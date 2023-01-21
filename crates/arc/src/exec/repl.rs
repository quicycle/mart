//! Command line arc repl backed by an Evaluator
use itertools::Itertools;
use rustyline::{
    completion::Completer, error::ReadlineError, highlight::Highlighter, hint::Hinter,
    validate::Validator, CompletionType, Config, Context, Editor, Helper,
};

use crate::exec::Evaluator;

// α β γ δ ε ζ η Λ λ ξ Σ Φ φ χ ψ
const PROMPT: &str = "ζ:: ";
const HISTORY_FILE: &str = "/tmp/ar-repl.history";

struct ArcHelper(Evaluator);

// XXX: Rustyline requires that you impl all of the below traits in order to impl `Helper` and it
//      looks like you are not able to only provide one and use that single piece of functionality.
impl Helper for ArcHelper {}
impl Highlighter for ArcHelper {}
impl Validator for ArcHelper {}
impl Hinter for ArcHelper {
    type Hint = String;
}

impl Completer for ArcHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        if line.starts_with('#') {
            Ok((0, vec!["#SIMPLIFY_MULTIVECTORS".into()]))
        } else if let Some(so_far) = line.split_whitespace().last() {
            let position = line.rfind(so_far).expect("should have been last word");
            let mut completions = self
                .0
                .env
                .keys()
                .filter(|k| k.starts_with(so_far))
                .cloned()
                .collect_vec();

            completions.extend(
                ctx.history()
                    .iter()
                    .rev()
                    .filter(|h| h.starts_with(line))
                    .map(|h| h.into()),
            );

            Ok((position, completions))
        } else {
            Ok((0, vec![]))
        }
    }
}

pub fn repl(ev: Evaluator) {
    let config = Config::builder()
        .tab_stop(4)
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .build();

    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(ArcHelper(ev)));

    if rl.load_history(HISTORY_FILE).is_err() {}

    println!("arc :: AR computation v{}", env!("CARGO_PKG_VERSION"));
    println!("(Simplification of results is off by default)\n");

    loop {
        match rl.readline(PROMPT) {
            Ok(line) if !line.is_empty() => {
                rl.add_history_entry(&line);
                if let Err(e) = rl.helper_mut().unwrap().0.eval(&line) {
                    println!("{}", e);
                }
            }

            // CTRL-C
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                continue;
            }

            // CTRL-D
            Err(ReadlineError::Eof) => {
                if let Err(e) = rl.save_history(HISTORY_FILE) {
                    println!("Error saving history: {}", e);
                }
                println!("Exiting...");
                return;
            }

            Err(err) => {
                println!("Read Error: {:?}", err);
                continue;
            }

            _ => (), // empty line
        }
    }
}
