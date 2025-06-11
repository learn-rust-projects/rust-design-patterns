// ------- Abstract Products -------
pub trait Drink {
    fn name(&self) -> &'static str;
}
pub trait Food {
    fn name(&self) -> &'static str;
}
