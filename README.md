# crisp
Simple Lisp interpreter based on the Clojure spec. Inspired by [this](https://stopa.io/post/222) tutorial.

To start the REPL:
```
$ cargo run
> (+ 3 4)
Primitive(Number(7.0))
> (> 5 6)
Primitive(Bool(false))
```

To run a program, pass a `.crisp` file. Use the 'begin' keyword to evaluate multiple expressions - a basic example can be found in [test.crisp](test.crisp).

Note that this is WIP so not all the basic arithmetic and logical operators have been implemented.
