use super::{ast::Statement, callback::CodeGenCallback};

pub struct CodeGen;

impl CodeGen {
    pub fn generate<C: CodeGenCallback>(&self, stmts: &[Statement], callback: &mut C) {
        for stmt in stmts {
            callback.emit_stmt(stmt);
        }
    }
}
