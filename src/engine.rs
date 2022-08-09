#[derive(Copy, Clone)]
enum Color {
    Black,
    White,
}

// Used for scoring, captures, etc.
struct ColorCount {
    black: u16,
    white: u16,
}

struct ScoreAcceptance {
    black_accepts: bool,
    white_accepts: bool,
}

enum GamePhase {
    NotStarted,
    InProgress(Color),
    InReview {
        score: ColorCount,
        acceptance: ScoreAcceptance,
    },
    Complete(Color),
}

enum GameAction {
    StonePlacement(usize, usize),
    Pass,
    SubmitScore,
    AcceptScore,
}

type PlayerMove = (Color, GameAction);

type Board = Vec<Vec<Option<Color>>>;

pub struct Game {
    board: Board,
    phase: GamePhase,
    captures: ColorCount,
    history: Vec<Board>,
}

impl Game {
    pub fn new() -> Game {
        let board: Board = vec![vec![None; 9]; 9];

        Game {
            board,
            phase: GamePhase::NotStarted,
            captures: ColorCount { black: 0, white: 0 },
            history: Vec::new(),
        }
    }

    fn validate_turn_phase(&self, pm: &PlayerMove) -> bool {
        false
    }

    pub fn try_move(&mut self, pm: PlayerMove) -> Result<&Game, &str> {
        // IF Complete:
        // * error, game is over

        // IF StonePlacement:
        // * validate correct turn and correct game phase
        // * validate move isn't suicide
        // * validate move isn't a ko
        // * calculate captured pieces, if any affected
        // * save game history
        // * change turn

        // IF Pass:
        // * validate correct turn and game phase
        // * determine whether to transition game
        // * change turn

        // IF SubmitScore:
        // * validate correct game phase only (turn doesn't matter)

        // IF AcceptScore:
        // * validate correct game phase
        // * determine whether to transition game state to complete or not

        Ok(self)
    }
}

// TODO: Figure out how to use internal modifier to allow Game creation with values passed in (to facilitate testing)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_turn_phase() {
        let g: Game = Game::new();

        // player cannot play out of turn
        assert_eq!(
            g.validate_turn_phase(&(Color::White, GameAction::StonePlacement(0, 0))),
            false
        );

        // can only place stone during NotStarted and InProgress
        assert_eq!(
            g.validate_turn_phase(&(Color::Black, GameAction::SubmitScore)),
            false
        );
        assert_eq!(
            g.validate_turn_phase(&(Color::Black, GameAction::AcceptScore)),
            false
        );

        // can only submit or accept during the InReview phase
        assert_eq!(
            g.validate_turn_phase(&(Color::Black, GameAction::AcceptScore)),
            false
        );

        // cannot move during Complete phase
    }
}
