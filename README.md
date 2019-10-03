# rust-closure-parcom

This is a closure based implementation of a rust parser combinator based on the Haskell [parsec](https://hackage.haskell.org/package/parsec) paper.

# syntax example

```rust

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
```
