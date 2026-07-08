# Lox Grammar

This document describes the Lox grammar as implemented in this project, plus rules from upcoming chapters.

Convention: `*` = zero or more, `?` = optional, `|` = alternative.

---

### Program

```
program     → statement* EOF
```

The parser loops in `parse()` until it reaches the end-of-file token.

### Statements

```
statement   → printStmt
            | exprStmt

printStmt   → "print" expression ";"
exprStmt    → expression ";"
```

| Rule | Parser method | AST node |
|------|---------------|----------|
| `statement` | `statement()` | dispatches |
| `printStmt` | `print_statement()` | `Stmt::Print` |
| `exprStmt` | `expression_statement()` | `Stmt::Expression` |

Statement dispatch checks the current token: `print` → print statement; anything else → expression statement (fallthrough).

The semicolon terminates the **statement**, not the expression. After parsing the expression, `consume(SEMICOLON)` requires and advances past `;`.

### Expressions

Operator precedence increases as you go down the chain (tighter binding lower in the tree).

```
expression  → assignment

equality    → comparison ( ("!=" | "==") comparison )*

comparison  → term ( (">" | ">=" | "<" | "<=") term )*

term        → factor ( ("-" | "+") factor )*

factor      → unary ( ("/" | "*") unary )*

unary       → ( "!" | "-" ) unary
            | primary

primary     → "true" | "false" | "nil"
            | NUMBER | STRING
            | "(" expression ")"
```

### Precedence (lowest to highest)

| Level | Operators |
|-------|-----------|
| Equality | `==` `!=` |
| Comparison | `>` `>=` `<` `<=` |
| Term | `+` `-` |
| Factor | `*` `/` |
| Unary | `!` `-` |
| Primary | literals, grouping |

---

### Declarations and variables

```
program     → declaration* EOF

declaration → varDecl
            | statement

varDecl     → "var" IDENTIFIER ( "=" expression )? ";"
```

### Assignment

```
expression  → assignment

assignment  → IDENTIFIER "=" assignment
            | equality
```

Right-associative: `a = b = c` parses as `a = (b = c)`.

### Variable access

```
primary     → "true" | "false" | "nil"
            | NUMBER | STRING
            | "(" expression ")"
            | IDENTIFIER
```

### Blocks and local scope (later)

```
statement   → exprStmt
            | printStmt
            | block

block       → "{" declaration* "}"
```

---

## Hierarchy diagram

```
program
└── statement*
      ├── printStmt        →  print expression ;
      └── exprStmt         →  expression ;

expression
└── equality
      └── comparison
            └── term
                  └── factor
                        └── unary
                              └── primary
                                    ├── true / false / nil
                                    ├── NUMBER / STRING
                                    └── ( expression )
```

---

## Expressions vs statements

Lox keeps **two separate hierarchies** (`Expr` and `Stmt`), because the grammars are disjoint:

- Operands of `+` are always **expressions**, never statements.
- The body of a `while` loop is always a **statement**.
