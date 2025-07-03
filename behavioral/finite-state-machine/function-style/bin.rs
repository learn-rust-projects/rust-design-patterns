#[derive(Debug, Clone, Copy)]
pub enum State {
    Small,
    Super,
    Cape,
    Fire,
}

pub struct MarioStateMachine {
    score: i32,
    state: State,
}

impl Default for MarioStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl MarioStateMachine {
    pub fn new() -> Self {
        MarioStateMachine {
            score: 0,
            state: State::Small,
        }
    }

    pub fn obtain_mushroom(&mut self) {
        if let State::Small = self.state {
            self.state = State::Super;
            self.score += 100;
        }
    }

    pub fn obtain_cape(&mut self) {
        match self.state {
            State::Small | State::Super => {
                self.state = State::Cape;
                self.score += 200;
            }
            _ => {}
        }
    }

    pub fn obtain_fire_flower(&mut self) {
        match self.state {
            State::Small | State::Super => {
                self.state = State::Fire;
                self.score += 300;
            }
            _ => {}
        }
    }

    pub fn meet_monster(&mut self) {
        match self.state {
            State::Super => {
                self.state = State::Small;
                self.score -= 100;
            }
            State::Cape => {
                self.state = State::Small;
                self.score -= 200;
            }
            State::Fire => {
                self.state = State::Small;
                self.score -= 300;
            }
            _ => {}
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_state(&self) -> State {
        self.state
    }
}

fn main() {
    let mut mario = MarioStateMachine::new();
    println!(
        "initial state: {:?}, score: {}",
        mario.get_state(),
        mario.get_score()
    );

    mario.obtain_fire_flower();
    println!(
        "after obtain fire flower: {:?}, score: {}",
        mario.get_state(),
        mario.get_score()
    );

    mario.meet_monster();
    println!(
        "after meet monster: {:?}, score: {}",
        mario.get_state(),
        mario.get_score()
    );
}
