use super::abstract_products::Drink;
use super::abstract_products::Food;
// ------- Concrete Products -------
pub struct ChineseTea;
impl Drink for ChineseTea {
    fn name(&self) -> &'static str {
        "Chinese Tea"
    }
}
pub struct ChineseRice;
impl Food for ChineseRice {
    fn name(&self) -> &'static str {
        "Rice"
    }
}

pub struct WesternWine;
impl Drink for WesternWine {
    fn name(&self) -> &'static str {
        "Red Wine"
    }
}
pub struct WesternSteak;
impl Food for WesternSteak {
    fn name(&self) -> &'static str {
        "Steak"
    }
}
