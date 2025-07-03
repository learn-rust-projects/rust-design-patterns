pub trait Expression {
    fn interpret(&self) -> i32;
}

pub struct Number {
    value: i32,
}

impl Number {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Expression for Number {
    fn interpret(&self) -> i32 {
        self.value
    }
}

pub struct Add {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Add {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { left, right }
    }
}

impl Expression for Add {
    fn interpret(&self) -> i32 {
        self.left.interpret() + self.right.interpret()
    }
}

pub struct Subtract {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Subtract {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { left, right }
    }
}

impl Expression for Subtract {
    fn interpret(&self) -> i32 {
        self.left.interpret() - self.right.interpret()
    }
}
fn main() {
    // Construct expression: 5 + (3 - 2)
    let expr = Add::new(
        Box::new(Number::new(5)),
        Box::new(Subtract::new(
            Box::new(Number::new(3)),
            Box::new(Number::new(2)),
        )),
    );

    let result = expr.interpret();
    // Output: The result is: 6
    println!("The result is: {}", result); 
}
