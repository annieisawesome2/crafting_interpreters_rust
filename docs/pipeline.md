# Interpreter Pipeline

How a Lox program runs from source text to output.

---

## Overview

```
source string
    ↓  Scanner::scan_tokens
Vec<Token>
    ↓  Parser::parse
Vec<Stmt>
    ↓  Interpreter::interpret
output / errors
```

Entry point: `Lox::run()` in `src/lox.rs`.

---

## 1. Scanning

**File:** `src/scanner.rs`

Reads the source character by character and produces a flat list of tokens (keywords, literals, operators, identifiers, punctuation). Errors are reported through `Lox::error()`.

The scanner recognizes words like `print` and `var`, and the parser decides how they are used.

---

## 2. Parsing

**File:** `src/parser.rs`

Consumes tokens and builds a `Vec<Stmt>` — the program AST.

### Key parser helpers

| Method | Purpose |
|--------|---------|
| `match_types` | Consumes tokens if it matches one of the given types |
| `check` | Peek: not at EOF and current token has the expected type |
| `consume` | Requires a token of the given type, otherwise gives parse error |
| `advance` | Move to the next token |
| `synchronize` | Error recovery (not wired up yet) |

### Parse errors

On failure, the parser calls `Lox::error()` and returns `None` from `parse()`. Better recovery with `synchronize()` comes when more statement types are added.

---

## 3. Execution

**File:** `src/interpreter.rs`

### `interpret(statements)`

Loops over every statement in the program and calls `execute()` on each. Stops on the first runtime error.

### `execute(stmt)`

| Statement | Action |
|-----------|--------|
| `Stmt::Expression` | `evaluate(expr)` → discard value |
| `Stmt::Print` | `evaluate(expr)` → `stringify` → `println!` |

### `evaluate(expr)`

Recursively walks the `Expr` tree and returns a `LiteralValue`. Used by both statement types and (later) variable reads/writes.

---

## Behavior change from expression-only mode

Previously the REPL parsed a single expression and always printed its value.

Now:

| Input | Output |
|-------|--------|
| `print 1 + 2;` | `3` |
| `1 + 2;` | *(nothing — value discarded)* |
| `1 + 2` *(no semicolon)* | parse error |

The REPL expects **full statements** with terminating semicolons.

---

## Error handling

| Phase | Flag | Exit code (file mode) |
|-------|------|------------------------|
| Scan / parse | `had_error` | 65 |
| Runtime | `had_runtime_error` | 70 |

In REPL mode, errors are printed and the prompt continues (`reset_error()` after each line).

---

## Planned: environments

Global and local variables require an **environment** — a map from name to value. The interpreter will hold an `Environment` and:

- `Stmt::Var` → `define(name, value)`
- `Expr::Variable` → `get(name)`
- `Expr::Assign` → update existing binding

Nested `Block` statements will push/pop child environments for local scope.
