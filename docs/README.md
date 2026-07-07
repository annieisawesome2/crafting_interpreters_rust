# Documentation

Reference notes for this Lox interpreter, following [Crafting Interpreters](https://craftinginterpreters.com/).

## Contents

| Document | Description |
|----------|-------------|
| [Grammar](grammar.md) | EBNF rules — implemented and planned |
| [AST](ast.md) | Syntax tree nodes for expressions and statements |
| [Pipeline](pipeline.md) | How source code flows through scan → parse → execute |

## Current milestone

Chapter 8.1 — **Statements and State** (in progress)

- Statements: expression statements and `print`
- Expressions: literals, grouping, unary, binary (through equality)
- Not yet: `var` declarations, variable access, assignment, blocks, local scope

## Running

```bash
# REPL — enter full statements with semicolons
cargo run

# Run a script
cargo run -- src/test.lox
```
