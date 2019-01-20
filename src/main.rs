extern crate pretty_env_logger;
extern crate regex;
#[macro_use] extern crate log;

#[macro_use] mod parser;

use rustyline::error::ReadlineError;
use crate::parser::*;

fn main() {
    pretty_env_logger::init();
    debug!("starting");

    let ws = || {
        discard(repeat1(regex("[\\s]")))
    };

    let opt_ws = || {
        optional(ws())
    };

    let number = || {
        flat_string(repeat1(digit()))
    };

    let operator = || {
        regex("[+/*-]")
    };

    let mut expr = Parser::new();
    let inner_expr = Box::new(
        one_of!(number(),
                  seq!(ch('('),
                          opt_ws(),
                          operator(),
                          repeat1(last_of(seq!(ws(), expr.delegate()))),
                       ch(')')))
    );

    expr.update(inner_expr);

    let parser = expr.delegate();

    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline("lisp> ");
        match readline {
            Ok(line) => {
                let result = parser(&line);
                match &result {
                    ParseResult::Value(value, remaining_input) => {
                        println!("{:#?}", result);
                        println!("{}", eval(&value));
                    }
                    ParseResult::Error(text) => {
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
/*
Value(
    List(
        [
            String(
                "("
            ),
            String(
                "+"
            ),
            List(
                [
                    String(
                        "1"
                    ),
                    String(
                        "2"
                    )
                ]
            ),
            String(
                ")"
            )
        ]
    ),
    ""
)*/

fn eval(expr: &ParseValue) -> u64 {
    match expr {
        ParseValue::String(number) => {
            return number.parse::<u64>().unwrap();
        },
        ParseValue::List(list) => {
            let op = list[1].string();
            let operands = list[2].list();
            let mut x = operands[0].string().parse::<u64>().unwrap();
            let mut i = 1;
            while i < operands.len() {
                match op {
                    "+" => {
                        x = x + eval(&operands[i]);
                    },
                    "*" => {
                        x = x * eval(&operands[i]);
                    },
                    "-" => {
                        x = x - eval(&operands[i]);
                    },
                    "/" => {
                        x = x / eval(&operands[i]);
                    },
                    _ => panic!("unknown op")
                }
                i = i + 1;
            }
            x
        }
        result => panic!()
    }
}