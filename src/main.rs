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

    let number = || {
      repeat(digit())
    };

    let operator = || {
        regex("[+/*-]")
    };

    let exbx = parser::boxed();
    let exbx_clone = exbx.clone();
    let expr = seq!(move |input: &str| {
                let parser = exbx_clone.borrow();
                parser(input)
            }
        );

    exbx.replace(Box::new(
        one_of!(number(),
                   seq!(ch('('), ch(')')))
    ));

    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline("lisp> ");
        match readline {
            Ok(line) => {
                let result = expr(&line);
                match result {
                    ParseResult::Value {value: _, remaining_input: _} => {
                        println!("success!");
                    }
                    ParseResult::Error{text} => {
                        println!("error: {}", text);
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
