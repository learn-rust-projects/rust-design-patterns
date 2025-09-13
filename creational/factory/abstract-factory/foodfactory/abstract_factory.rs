// ------- Abstract Factory -------
use super::abstract_products::{Drink, Food};
pub trait AbstractFactory {
    fn create_drink(&self) -> Box<dyn Drink>;
    fn create_food(&self) -> Box<dyn Food>;
}
