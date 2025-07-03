use crate::state::*;

pub struct MarioStateMachine {
    pub score: i32,
    pub current_state: Box<dyn MarioState>,
}
impl Default for MarioStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl MarioStateMachine {
    pub fn new() -> Self {
        Self {
            score: 0,
            current_state: Box::new(SmallMario),
        }
    }

    fn transition<F>(&mut self, f: F)
    where
        F: FnOnce(Box<dyn MarioState>, &mut MarioStateMachine) -> Option<Box<dyn MarioState>>,
    {
        let prev_state = std::mem::replace(&mut self.current_state, Box::new(DummyMario));
        if let Some(state) = f(prev_state, self) {
            self.current_state = state
        }
    }

    pub fn obtain_mushroom(&mut self) {
        self.transition(|s, sm| s.obtain_mushroom(sm));
    }

    pub fn obtain_cape(&mut self) {
        self.transition(|s, sm| s.obtain_cape(sm));
    }

    pub fn obtain_fire_flower(&mut self) {
        self.transition(|s, sm| s.obtain_fire_flower(sm));
    }

    pub fn meet_monster(&mut self) {
        self.transition(|s, sm| s.meet_monster(sm));
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_state(&self) -> State {
        self.current_state.name()
    }
}
