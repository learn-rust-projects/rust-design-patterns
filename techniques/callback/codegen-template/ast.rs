#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Var(String),
}

#[derive(Debug)]
pub struct Statement {
    pub name: String,
    pub value: Expr,
}
