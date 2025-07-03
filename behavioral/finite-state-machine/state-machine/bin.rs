use state_machine::MarioStateMachine;

fn main() {
    let mut mario = MarioStateMachine::new();
    println!(
        "initial state: {:?}, score: {}",
        mario.get_state(),
        mario.get_score()
    );

    mario.obtain_mushroom();
    println!(
        "after obtain mushroom: {:?}, score: {}",
        mario.get_state(),
        mario.get_score()
    );

    mario.obtain_cape();
    println!(
        "after obtain cape: {:?}, score: {}",
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
