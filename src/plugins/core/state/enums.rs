use bevy::prelude::{ComputedStates, States};

#[derive(States, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading, // assets loading
    InGame {
        paused: bool,
    },
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct InGame;

impl ComputedStates for InGame {
    // Computed states can be calculated from one or many source states.
    type SourceStates = GameState;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: GameState) -> Option<InGame> {
        match sources {
            GameState::InGame { .. } => Some(InGame),
            _ => None,
        }
    }
}

// #[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
// // This macro means that `GamePhase` will only exist when we're in the `InGame` computed state.
// // The intermediate computed state is helpful for clarity here, but isn't required:
// // you can manually `impl SubStates` for more control, multiple parent states and non-default initial value!
// #[source(InGame = InGame)]
// enum GamePhase {
//     #[default]
//     Setup,
//     Battle,
//     Conclusion
// }