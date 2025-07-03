pub use crate::state_machine::MarioStateMachine;
pub trait MarioState {
    fn name(&self) -> State;
    fn obtain_mushroom(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>>;
    fn obtain_cape(self: Box<Self>, machine: &mut MarioStateMachine)
    -> Option<Box<dyn MarioState>>;
    fn obtain_fire_flower(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>>;
    fn meet_monster(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>>;
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Small,
    Super,
    Cape,
    Fire,
}

pub struct DummyMario;

impl MarioState for DummyMario {
    fn name(&self) -> State {
        State::Small
    }

    fn obtain_mushroom(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        None
    }

    fn obtain_cape(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        None
    }

    fn obtain_fire_flower(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        None
    }

    fn meet_monster(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        None
    }
}

pub struct SmallMario;

impl MarioState for SmallMario {
    fn name(&self) -> State {
        State::Small
    }

    fn obtain_mushroom(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        machine.score += 100;
        Some(Box::new(SuperMario))
    }

    fn obtain_cape(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        machine.score += 200;
        Some(Box::new(CapeMario))
    }

    fn obtain_fire_flower(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        machine.score += 300;
        Some(Box::new(FireMario))
    }

    fn meet_monster(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        None
    }
}

pub struct SuperMario;

impl MarioState for SuperMario {
    fn name(&self) -> State {
        State::Super
    }

    fn obtain_mushroom(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        Some(self)
    }

    fn obtain_cape(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        machine.score += 200;
        Some(Box::new(CapeMario))
    }

    fn obtain_fire_flower(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        machine.score += 300;
        Some(Box::new(FireMario))
    }

    fn meet_monster(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        machine.score -= 100;
        Some(Box::new(SmallMario))
    }
}

pub struct CapeMario;

impl MarioState for CapeMario {
    fn name(&self) -> State {
        State::Cape
    }

    fn obtain_mushroom(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        Some(self)
    }

    fn obtain_cape(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        Some(self)
    }

    fn obtain_fire_flower(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        Some(self)
    }

    fn meet_monster(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        machine.score -= 200;
        Some(Box::new(SmallMario))
    }
}

pub struct FireMario;

impl MarioState for FireMario {
    fn name(&self) -> State {
        State::Fire
    }

    fn obtain_mushroom(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        Some(self)
    }

    fn obtain_cape(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        Some(self)
    }

    fn obtain_fire_flower(
        self: Box<Self>,
        _machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        Some(self)
    }

    fn meet_monster(
        self: Box<Self>,
        machine: &mut MarioStateMachine,
    ) -> Option<Box<dyn MarioState>> {
        machine.score -= 300;
        Some(Box::new(SmallMario))
    }
}
