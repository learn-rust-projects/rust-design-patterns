use super::trait_def::Encouragement;

pub struct AlmostDone;

impl Encouragement for AlmostDone {
    fn encourage(&self) -> &'static str {
        "Hang in there!"
    }
}
