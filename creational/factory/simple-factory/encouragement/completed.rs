use super::trait_def::Encouragement;

pub struct Completed;

impl Encouragement for Completed {
    fn encourage(&self) -> &'static str {
        "Congratulations, you made it!"
    }
}
