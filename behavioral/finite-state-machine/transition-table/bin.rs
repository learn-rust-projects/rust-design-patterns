#[derive(Debug, Clone, Copy)]
pub enum State {
    Small = 0,
    Super = 1,
    Cape = 2,
    Fire = 3,
}

pub enum Event {
    Mushroom = 0,
    Cape = 1,
    FireFlower = 2,
    Monster = 3,
}

const TRANSITIONS: [[State; 4]; 4] = [
    [State::Super, State::Cape, State::Fire, State::Small], // Small
    [State::Super, State::Cape, State::Fire, State::Small], // Super
    [State::Cape, State::Cape, State::Cape, State::Small],  // Cape
    [State::Fire, State::Fire, State::Fire, State::Small],  // Fire
];

const SCORES: [[i32; 4]; 4] = [
    [100, 200, 300, 0],
    [0, 200, 300, -100],
    [0, 0, 0, -200],
    [0, 0, 0, -300],
];

pub struct MarioStateMachine {
    state: State,
    score: i32,
}
impl Default for MarioStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl MarioStateMachine {
    pub fn new() -> Self {
        Self {
            state: State::Small,
            score: 0,
        }
    }

    pub fn execute_event(&mut self, event: Event) {
        let s = self.state as usize;
        let e = event as usize;
        self.state = TRANSITIONS[s][e];
        self.score += SCORES[s][e];
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

    mario.execute_event(Event::Mushroom);
    println!(
        "after obtain mushroom: {:?}, score: {}",
        mario.get_state(),
        mario.get_score()
    );

    mario.execute_event(Event::Cape);
    println!(
        "after obtain cape: {:?}, score: {}",
        mario.get_state(),
        mario.get_score()
    );

    mario.execute_event(Event::Monster);
    println!(
        "after meet monster: {:?}, score: {}",
        mario.get_state(),
        mario.get_score()
    );
}
