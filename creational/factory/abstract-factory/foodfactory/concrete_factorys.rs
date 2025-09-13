// ------- Concrete Factories -------
use super::{
    abstract_factory::AbstractFactory,
    abstract_products::{Drink, Food},
    concrete_products::{ChineseRice, ChineseTea, WesternSteak, WesternWine},
};

pub struct ChineseFactory;
impl AbstractFactory for ChineseFactory {
    fn create_drink(&self) -> Box<dyn Drink> {
        Box::new(ChineseTea)
    }
    fn create_food(&self) -> Box<dyn Food> {
        Box::new(ChineseRice)
    }
}

pub struct WesternFactory;
impl AbstractFactory for WesternFactory {
    fn create_drink(&self) -> Box<dyn Drink> {
        Box::new(WesternWine)
    }
    fn create_food(&self) -> Box<dyn Food> {
        Box::new(WesternSteak)
    }
}
