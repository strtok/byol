extern crate pretty_env_logger;
extern crate regex;
#[macro_use] extern crate log;

#[macro_use] mod parser;

use rustyline::error::ReadlineError;
use crate::parser::*;

fn main() {
    pretty_env_logger::init();
    debug!("starting");

    let whitespace = || {
        repeat(regex("[\\s]"))
    };
    let parser = seq!(ch('('), whitespace(), ch(')'));

    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline("lisp> ");
        match readline {
            Ok(line) => {
                let result = parser(&line);
                match result {
                    ParseResult::Value {value: _, remaining_input: _} => {
                        info!("success!");
                    }
                    ParseResult::Error{text} => {
                        info!("error: {}", text);
                    }
                }
            },
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
