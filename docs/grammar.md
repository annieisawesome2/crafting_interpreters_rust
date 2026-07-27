# Lox Grammar

## Program and declarations

```
program        → declaration* EOF ;

declaration    → funDecl
               | varDecl
               | statement ;

funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
```

The parser loops in `parse()` / `declaration()` until it reaches the end-of-file token. Declarations are the top-level unit so `var` and `fun` can appear inside blocks as well as at global scope.

| Rule | Parser method | AST node |
|------|---------------|----------|
| `declaration` | `declaration()` / `try_declaration()` | dispatches |
| `funDecl` / `function` | `function()` | `Stmt::Function` |
| `varDecl` | `var_declaration()` | `Stmt::Var` |

---

## Statements

One canonical `statement` rule — every statement form lives here (not redeclared per chapter).

```
statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | returnStmt
               | whileStmt
               | block ;

exprStmt       → expression ";" ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;

printStmt      → "print" expression ";" ;

returnStmt     → "return" expression? ";" ;   // not yet implemented

whileStmt      → "while" "(" expression ")" statement ;

block          → "{" declaration* "}" ;
```

`for` is syntactic sugar: the parser desugars it into `while` (plus optional initializer / increment), so there is no `Stmt::For` node.

The semicolon terminates the **statement**, not the expression. After parsing the expression, `consume(SEMICOLON)` requires and advances past `;`.

| Rule | Parser method | AST node |
|------|---------------|----------|
| `statement` | `statement()` | dispatches |
| `exprStmt` | `expression_statement()` | `Stmt::Expression` |
| `forStmt` | `for_statement()` | desugared (no dedicated node) |
| `ifStmt` | `if_statement()` | `Stmt::If` |
| `printStmt` | `print_statement()` | `Stmt::Print` |
| `returnStmt` | — | — (planned) |
| `whileStmt` | `while_statement()` | `Stmt::While` |
| `block` | `block()` | `Stmt::Block` |

---

## Expressions

Operator precedence increases as you go down the chain (tighter binding lower in the tree).

```
expression     → assignment ;

assignment     → IDENTIFIER "=" assignment
               | logic_or ;

logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;

equality       → comparison ( ( "!=" | "==" ) comparison )* ;

comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

term           → factor ( ( "-" | "+" ) factor )* ;

factor         → unary ( ( "/" | "*" ) unary )* ;

unary          → ( "!" | "-" ) unary
               | call ;

call           → primary ( "(" arguments? ")" )* ;

arguments      → expression ( "," expression )* ;

primary        → "true" | "false" | "nil"
               | NUMBER | STRING
               | IDENTIFIER
               | "(" expression ")" ;
```

Assignment is right-associative: `a = b = c` parses as `a = (b = c)`.

`call` matches a primary followed by zero or more function calls. With no parentheses, it is just a bare primary. The `*` allows a series like `fn(1)(2)(3)`. `arguments` requires at least one expression; zero-argument calls make the whole `arguments` production optional in `call`.

AST node: `Call { callee, paren, arguments }` — callee expression, closing `)` token (for runtime error location), and argument list.

In the parser, `unary()` calls `call()` instead of jumping straight to `primary()`. `call()` parses a primary, then loops while it sees `(`, finishing each call with `finish_call()`.

### Precedence (lowest to highest)

| Level | Operators |
|-------|-----------|
| Assignment | `=` |
| Or | `or` |
| And | `and` |
| Equality | `==` `!=` |
| Comparison | `>` `>=` `<` `<=` |
| Term | `+` `-` |
| Factor | `*` `/` |
| Unary | `!` `-` |
| Call | `()` |
| Primary | literals, grouping, identifiers |

---

## Hierarchy diagram

```
program
└── declaration*
      ├── funDecl          →  fun function
      ├── varDecl          →  var IDENTIFIER ( = expression )? ;
      └── statement
            ├── exprStmt   →  expression ;
            ├── forStmt    →  for ( ... ) statement   (desugared)
            ├── ifStmt     →  if ( expression ) statement ( else statement )?
            ├── printStmt  →  print expression ;
            ├── returnStmt →  return expression? ;    (planned)
            ├── whileStmt  →  while ( expression ) statement
            └── block      →  { declaration* }

expression
└── assignment
      └── logic_or
            └── logic_and
                  └── equality
                        └── comparison
                              └── term
                                    └── factor
                                          └── unary
                                                └── call
                                                      └── primary ( "(" arguments? ")" )*
                                                            ├── true / false / nil
                                                            ├── NUMBER / STRING
                                                            ├── IDENTIFIER
                                                            └── ( expression )
```

---

## Expressions vs statements

Lox keeps **two separate hierarchies** (`Expr` and `Stmt`), because the grammars are disjoint:

- Operands of `+` are always **expressions**, never statements.
- The body of a `while` loop is always a **statement**.
