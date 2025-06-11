// ------- Concrete Factories -------
use super::abstract_factory::AbstractFactory;
use super::abstract_products::Drink;
use super::abstract_products::Food;
use super::concrete_products::ChineseRice;
use super::concrete_products::ChineseTea;
use super::concrete_products::WesternSteak;
use super::concrete_products::WesternWine;

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
