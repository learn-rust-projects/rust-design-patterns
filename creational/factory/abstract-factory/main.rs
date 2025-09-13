mod foodfactory;
pub use foodfactory::{AbstractFactory, ChineseFactory, WesternFactory};
// ------- Client code -------
fn main() {
    // Can be ChineseFactory or WesternFactory
    let chinese = ChineseFactory;
    let western = WesternFactory;

    let c_drink = chinese.create_drink();
    let c_food = chinese.create_food();
    println!("Chinese meal: {} + {}", c_food.name(), c_drink.name());

    let w_drink = western.create_drink();
    let w_food = western.create_food();
    println!("Western meal: {} + {}", w_food.name(), w_drink.name());

    // Can be ChineseFactory or WesternFactory
    let chinese = ChineseFactory;
    let western = WesternFactory;

    let factory: &dyn AbstractFactory = if true { &chinese } else { &western };
    let c_drink = factory.create_drink();
    let c_food = factory.create_food();
    println!("dyn meal: {} + {}", c_food.name(), c_drink.name());
}
