use super::trait_def::Encouragement;

pub struct InProgress;

impl Encouragement for InProgress {
    fn encourage(&self) -> &'static str {
        "Keep going!"
    }
}
