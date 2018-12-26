extern crate pretty_env_logger;
#[macro_use] extern crate log;

use rustyline::error::ReadlineError;

fn main() {
    pretty_env_logger::init();
    debug!("starting");

    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline("lisp> ");
        match readline {
            Ok(line) => println!("line: {:?}", line),
            Err(ReadlineError::Interrupted) => {
                break;
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                error!("error: {:?}", err);
                break;
            }
        }
    }
}
