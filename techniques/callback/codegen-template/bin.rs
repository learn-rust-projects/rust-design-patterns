mod ast;
mod callback;
mod gen_c;
mod gen_js;
mod gen_py;
mod template;

use ast::*;
use gen_c::CGen;
use gen_js::JsGen;
use gen_py::PyGen;
use template::CodeGen;

fn main() {
    let stmts = vec![
        Statement {
            name: "x".into(),
            value: Expr::Add(Box::new(Expr::Number(1)), Box::new(Expr::Number(2))),
        },
        Statement {
            name: "y".into(),
            value: Expr::Add(Box::new(Expr::Var("x".into())), Box::new(Expr::Number(3))),
        },
    ];

    // JavaScript
    let mut js = JsGen::new();
    CodeGen.generate(&stmts, &mut js);
    println!("--- JavaScript ---\n{}", js.result.join("\n"));

    // C
    let mut c = CGen::new();
    CodeGen.generate(&stmts, &mut c);
    println!("\n--- C ---\n{}", c.result.join("\n"));

    // Python
    let mut py = PyGen::new();
    CodeGen.generate(&stmts, &mut py);
    println!("\n--- Python ---\n{}", py.result.join("\n"));
}
