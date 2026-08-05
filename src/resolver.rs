use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;

use std::collections::HashMap; 

struct Resolver<'a> {
    interpreter: &'a mut Interpreter, 
    scopes: Vec<HashMap<String, bool>>
}

impl<'a> Resolver<'a> {
    fn new(interpreter: &'a mut Interpreter) -> Self {
        Self { 
            interpreter, 
            scopes: Vec::new()
        }
    }
    // resolver owns/borrows interpreter and walks AST before execution
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
            _ => {}
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new()); 
    }

    end_scope(&mut self) {
        self.scopes.pop(); 
    }
}