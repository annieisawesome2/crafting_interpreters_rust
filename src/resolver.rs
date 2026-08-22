use crate::expr::Expr;
use crate::lox::Lox;
use crate::stmt::Stmt;
use crate::token::Token;

use std::collections::HashMap;

#[allow(dead_code)]
struct Resolver<'a> {
    lox: &'a mut Lox,
    scopes: Vec<HashMap<String, bool>>,
}

#[allow(dead_code)]
impl<'a> Resolver<'a> {
    fn new(lox: &'a mut Lox) -> Self {
        Self {
            lox,
            scopes: Vec::new(),
        }
    }

    // resolver walks AST before execution
    fn resolve(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements);
                self.end_scope();
            }

            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(init) = initializer {
                    self.resolve_expr(init);
                }
                self.define(name);
            }
            _ => {}
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        let Some(scope) = self.scopes.last_mut() else {
            return; // global
        };

        scope.insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Variable { name } => {
                if let Some(scope) = self.scopes.last() {
                    if scope.get(&name.lexeme) == Some(&false) {
                        self.lox
                            .error(name, "Can't read local variable in its own initializer.");
                    }
                }
                self.resolve_local(expr, name);
            }

            _ => {}
        }
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                let depth = self.scopes.len() - 1 - i;
                self.lox.interpreter.resolve(expr, depth);
                return;
            }
        }
    }
}
