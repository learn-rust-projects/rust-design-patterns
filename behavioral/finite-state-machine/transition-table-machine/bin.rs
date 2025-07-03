// Mario State Machine with Lookup Table + Behavioral Side Effects

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Small = 0,
    Super = 1,
    Cape = 2,
    Fire = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Mushroom = 0,
    Cape = 1,
    FireFlower = 2,
    Monster = 3,
}

const TRANSITIONS: [[State; 4]; 4] = [
    [State::Super, State::Cape, State::Fire, State::Small],
    [State::Super, State::Cape, State::Fire, State::Small],
    [State::Cape, State::Cape, State::Cape, State::Small],
    [State::Fire, State::Fire, State::Fire, State::Small],
];

const SCORES: [[i32; 4]; 4] = [
    [100, 200, 300, 0],
    [0, 200, 300, -100],
    [0, 0, 0, -200],
    [0, 0, 0, -300],
];

pub trait MarioState {
    fn name(&self) -> State;
    fn obtain_mushroom(self: Box<Self>, machine: &mut MarioStateMachine);
    fn obtain_cape(self: Box<Self>, machine: &mut MarioStateMachine);
    fn obtain_fire_flower(self: Box<Self>, machine: &mut MarioStateMachine);
    fn meet_monster(self: Box<Self>, machine: &mut MarioStateMachine);
}

pub struct SmallMario;
impl MarioState for SmallMario {
    fn name(&self) -> State {
        State::Small
    }

    fn obtain_mushroom(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Small] Eat mushroom -> become Super");
    }
    fn obtain_cape(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Small] Obtain cape -> fly");
    }
    fn obtain_fire_flower(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Small] Obtain fire flower -> fire mode");
    }
    fn meet_monster(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Small] Hit by monster -> die");
    }
}

pub struct SuperMario;
impl MarioState for SuperMario {
    fn name(&self) -> State {
        State::Super
    }
    fn obtain_mushroom(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Super] Eat mushroom -> no effect");
    }
    fn obtain_cape(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Super] Obtain cape -> flying mode");
    }
    fn obtain_fire_flower(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Super] Obtain fire flower -> fire mode");
    }
    fn meet_monster(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Super] Hit by monster -> become Small");
    }
}

pub struct CapeMario;
impl MarioState for CapeMario {
    fn name(&self) -> State {
        State::Cape
    }
    fn obtain_mushroom(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Cape] Eat mushroom -> no effect");
    }
    fn obtain_cape(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Cape] Already has cape");
    }
    fn obtain_fire_flower(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Cape] Reject fire flower (strategy limit)");
    }
    fn meet_monster(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Cape] Hit by monster -> become Small");
    }
}

pub struct FireMario;
impl MarioState for FireMario {
    fn name(&self) -> State {
        State::Fire
    }
    fn obtain_mushroom(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Fire] Eat mushroom -> no effect");
    }
    fn obtain_cape(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Fire] Reject cape");
    }
    fn obtain_fire_flower(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Fire] Already in fire mode");
    }
    fn meet_monster(self: Box<Self>, _: &mut MarioStateMachine) {
        println!("[Fire] Hit by monster -> become Small");
    }
}

pub struct MarioStateMachine {
    state: State,
    score: i32,
    state_object: Box<dyn MarioState>,
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
            state_object: Box::new(SmallMario),
        }
    }

    pub fn execute_event(&mut self, event: Event) {
        let current = std::mem::replace(&mut self.state_object, Box::new(DummyMario));
        match event {
            Event::Mushroom => current.obtain_mushroom(self),
            Event::Cape => current.obtain_cape(self),
            Event::FireFlower => current.obtain_fire_flower(self),
            Event::Monster => current.meet_monster(self),
        }

        let s = self.state as usize;
        let e = event as usize;
        self.score += SCORES[s][e];
        self.state = TRANSITIONS[s][e];

        self.state_object = match self.state {
            State::Small => Box::new(SmallMario),
            State::Super => Box::new(SuperMario),
            State::Cape => Box::new(CapeMario),
            State::Fire => Box::new(FireMario),
        };

        println!("=> State: {:?}, Score: {}\n", self.state, self.score);
    }

    pub fn get_state(&self) -> State {
        self.state
    }
    pub fn get_score(&self) -> i32 {
        self.score
    }
}

pub struct DummyMario;
impl MarioState for DummyMario {
    fn name(&self) -> State {
        State::Small
    }
    fn obtain_mushroom(self: Box<Self>, _: &mut MarioStateMachine) {}
    fn obtain_cape(self: Box<Self>, _: &mut MarioStateMachine) {}
    fn obtain_fire_flower(self: Box<Self>, _: &mut MarioStateMachine) {}
    fn meet_monster(self: Box<Self>, _: &mut MarioStateMachine) {}
}

fn main() {
    let mut mario = MarioStateMachine::new();
    println!(
        "Initial State: {:?}, Score: {}\n",
        mario.get_state(),
        mario.get_score()
    );

    mario.execute_event(Event::Mushroom);
    mario.execute_event(Event::Cape);
    mario.execute_event(Event::FireFlower);
    mario.execute_event(Event::Monster);
}
