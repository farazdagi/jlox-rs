# jlox-rs

Tree-walk interpreter for Lox language in Rust.

This is the first of two interpreters described in (absolutely astonishing and MUST *buy* and read)
book [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

The idea is to follow the book implementation (including challenges) and try to come up with an
equally concise and fast implementation in safe Rust.

## Features

- [ ] Implementation includes and passes full
  [original test suite](https://github.com/munificent/craftinginterpreters/tree/master/test)
  (ideally without rewriting original tests in any way)
- [ ] While idiomatic Rust is preferred, to the extend possible, the implementation should follow
  the description in the book. Therefore, anyone reading the book should have an easy time
  navigating this repository.
- [ ] Interactive REPL with intuitive error messages and history
