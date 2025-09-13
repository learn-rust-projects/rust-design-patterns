use super::{ast::*, callback::CodeGenCallback};

pub struct JsGen {
    pub result: Vec<String>,
}

impl JsGen {
    pub fn new() -> Self {
        Self { result: vec![] }
    }
}

impl CodeGenCallback for JsGen {
    fn emit_stmt(&mut self, stmt: &Statement) {
        let code = format!("let {} = {};", stmt.name, self.emit_expr(&stmt.value));
        self.result.push(code);
    }

    fn emit_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::Var(name) => name.clone(),
            Expr::Add(lhs, rhs) => format!("({} + {})", self.emit_expr(lhs), self.emit_expr(rhs)),
        }
    }
}
