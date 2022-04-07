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

    pub fn try_move(&mut self, pm: PlayerMove) -> Result<&Game, &str> {
        // IF StonePlacement:
        // * validate correct turn and correct game phase
        // * validate move isn't suicide
        // * validate move isn't a ko
        // * calculate captured pieces, if any affected
        // * save game history

        // IF Pass:
        // * validate correct turn and game phase
        // * determine whether to transition game

        // IF SubmitScore:
        // * validate correct game phase only (turn doesn't matter)

        // IF AcceptScore:
        // * validate correct game phase
        // * determine whether to transition game state to complete or not

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_try_move() {
        assert_eq!(1, 1);
    }
}
