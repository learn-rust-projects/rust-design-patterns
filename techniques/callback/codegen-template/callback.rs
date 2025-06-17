use super::ast::{Expr, Statement};

pub trait CodeGenCallback {
    fn emit_stmt(&mut self, stmt: &Statement);
    fn emit_expr(&mut self, expr: &Expr) -> String;
}
