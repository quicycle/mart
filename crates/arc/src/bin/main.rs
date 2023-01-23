use clap::Parser;

use arc::exec::{repl, Evaluator};

/// An evaluator for AR calculations. If a filename or expression is not passed on the command
/// line, arc will start in REPL mode allowing you to interactively enter and evaluate expressions.
#[derive(Debug, Parser)]
#[clap(name = "arc", version = "1.0")]
struct Options {
    /// Provide a command to evaluate on the command line
    #[clap(short = 'c')]
    command: Option<String>,

    /// Enter an interactive repl after evaluating the provided input
    #[clap(short = 'i')]
    interactive: bool,

    /// path to a *.arc file to evaluate
    fname: Option<String>,
}

fn main() {
    let opts: Options = Options::parse();
    let mut ev = Evaluator::new();

    if let Some(ref command) = opts.command {
        if let Err(e) = ev.eval(command) {
            println!("{}", e);
        }
    } else if let Some(ref fname) = opts.fname {
        if let Err(e) = ev.eval_file(fname) {
            println!("{}", e);
        }
    }

    if (opts.fname.is_none() && opts.command.is_none()) || opts.interactive {
        repl(ev)
    }
}
