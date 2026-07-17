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
            | call

call        → primary ( "(" arguments? ")" )*

arguments   → expression ( "," expression )*

primary     → "true" | "false" | "nil"
            | NUMBER | STRING
            | "(" expression ")"
```

`call` matches a primary followed by zero or more function calls. With no parentheses, it is just a bare primary. Each call is a pair of parentheses with an optional argument list inside.

The `*` on call allows a series like `fn(1)(2)(3)` — currying-style nested calls. `arguments` requires at least one expression; zero-argument calls are handled by making the whole `arguments` production optional in `call`.

AST node: `Call { callee, paren, arguments }` — stores the callee expression, the closing `)` token (for runtime error location), and the argument list.

In the parser, `unary()` calls `call()` instead of jumping straight to `primary()`. `call()` parses a primary, then loops while it sees `(`, finishing each call with `finish_call()`.

### Precedence (lowest to highest)

| Level | Operators |
|-------|-----------|
| Equality | `==` `!=` |
| Comparison | `>` `>=` `<` `<=` |
| Term | `+` `-` |
| Factor | `*` `/` |
| Unary | `!` `-` |
| Call | `()` |
| Primary | literals, grouping |

---

### Declarations and variables

```
program     → declaration* EOF

declaration → varDecl
            | statement

varDecl     → "var" IDENTIFIER ( "=" expression )? ";"
```

### Assignment and Logical Operators

```
expression  → assignment

assignment  → IDENTIFIER "=" assignment
            |logic_or

logic_or    → logic_and ("or" logic_and) *
logic_and   → equality ("and" equality )*

```

Right-associative: `a = b = c` parses as `a = (b = c)`.

### Variable access

```
primary     → "true" | "false" | "nil"
            | NUMBER | STRING
            | "(" expression ")"
            | IDENTIFIER
```

### Blocks and local scope

```
statement   → exprStmt
            | ifStmt
            | printStmt
            | block

ifStmt      → "if" "(" expression ")" statement
            ( "else" statement )?

block       → "{" declaration* "}"
```

---

## Hierarchy diagram

```
program
└── statement*
      ├── printStmt        →  print expression ;
      ├── exprStmt         →  expression ;
      ├── ifStmt           →  if ( expression ) statement ( else statement )?
      └── block            →  { declaration* }

expression
└── equality
      └── comparison
            └── term
                  └── factor
                        └── unary
                              └── call
                                    └── primary ( "(" arguments? ")" )*
                                          ├── true / false / nil
                                          ├── NUMBER / STRING
                                          └── ( expression )
```

---

## Expressions vs statements

Lox keeps **two separate hierarchies** (`Expr` and `Stmt`), because the grammars are disjoint:

- Operands of `+` are always **expressions**, never statements.
- The body of a `while` loop is always a **statement**.
