mod almost_done;
mod completed;
mod in_progress;
mod trait_def;

pub struct EncouragementFactory;

impl EncouragementFactory {
    pub fn get_encourager(progress: u32) -> Box<dyn trait_def::Encouragement> {
        match progress {
            100 => Box::new(completed::Completed),
            90..=99 => Box::new(almost_done::AlmostDone),
            _ => Box::new(in_progress::InProgress),
        }
    }
}
