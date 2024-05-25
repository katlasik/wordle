use crate::guess::{CompletedGuess, EmptyGuess, Guess, PendingGuess};
use crate::letters::Letters;
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameStatus {
    Pending,
    Won,
    Lost
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameUpdateResult {
    Ok,
    GameFinished,
    InvalidInput
}

#[derive(Clone)]
pub struct GameState {
    pub(crate) target_word: String,
    guesses: Vec<CompletedGuess>,
    pending_guess: Option<PendingGuess>,
    pub(crate) letters: Letters,
    pub(crate) max_tries: u8,
    pub(crate) status: GameStatus
}

impl GameState {

    pub(crate) fn get(&self, i: usize) -> Box<dyn Guess> {

        match self.guesses.get(i) {
            Some(guess) => Box::new(guess.clone()),
            None =>
                match self.current_guess() {
                    Some(guess) if i == self.guesses.len() => Box::new(guess.clone()),
                    _ => Box::new(EmptyGuess)
                }
        }
    }

    pub(crate) fn new(target_word: &str, max_tries: u8) -> Self {
        Self{
            target_word: target_word.to_string(),
            guesses: Vec::new(),
            pending_guess: Some(PendingGuess::new(target_word.to_string())),
            letters: Letters::new(),
            max_tries,
            status: GameStatus::Pending
        }
    }

    fn current_guess(&self) -> Option<&PendingGuess> {
        self.pending_guess.as_ref()
    }

    fn current_guess_mut(&mut self) -> Option<&mut PendingGuess> {
        self.pending_guess.as_mut()
    }

    pub(crate) fn push_letter(&mut self, letter: char) -> GameUpdateResult {
        let is_alpha= letter.is_alphabetic();

        match self.current_guess_mut() {
            Some(guess) if is_alpha => guess.push_letter(letter),
            _ => return GameUpdateResult::InvalidInput
        }
    }

    pub(crate) fn pop_letter(&mut self) -> GameUpdateResult {
        match self.current_guess_mut() {
            Some(guess) => guess.pop_letter(),
            _ => GameUpdateResult::InvalidInput
        }
    }

    pub(crate) fn commit_guess(&mut self) -> GameUpdateResult {
        match self.current_guess().cloned() {
            Some(guess) if guess.full() => {
                if self.guesses.len() >= self.max_tries as usize {
                    self.pending_guess = None;
                }
                let committed_guess = guess.complete();

                let all_correct = committed_guess.is_all_correct();
                self.letters.update_many(&committed_guess.letters());
                self.guesses.push(committed_guess);

                if all_correct {
                    self.pending_guess = None;
                    self.status = GameStatus::Won;
                } else if self.guesses.len() == self.max_tries as usize {
                    self.pending_guess = None;
                    self.status = GameStatus::Lost;
                } else {
                    self.pending_guess = Some(PendingGuess::new(self.target_word.clone()));
                }

                GameUpdateResult::Ok
            }
            Some(_) => GameUpdateResult::InvalidInput,
            None => GameUpdateResult::GameFinished
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::game_state::{GameState, GameUpdateResult};
    use crate::guess::Guess;
    use crate::letters::{GuessedLetter, LetterState};

    fn setup() -> GameState {
        GameState::new("test", 3)
    }

    #[test]
    fn should_not_allow_to_commit_if_not_all_letters_are_provided() {
        let mut state = setup();
        state.push_letter('t');
        state.push_letter('e');
        state.push_letter('s');
        let result = state.commit_guess();
        assert_eq!(result, GameUpdateResult::InvalidInput);
    }

    #[test]
    fn should_not_allow_to_delete_letters_if_fields_are_empty() {
        let mut state = setup();
        let result = state.pop_letter();
        assert_eq!(result, GameUpdateResult::InvalidInput);
    }

    #[test]
    fn should_not_allow_to_add_letters_if_fields_are_full() {
        let mut state = setup();
        state.push_letter('x');
        state.push_letter('x');
        state.push_letter('x');
        let pre= state.push_letter('x');
        let post = state.push_letter('x');
        assert_eq!(pre, GameUpdateResult::Ok);
        assert_eq!(post, GameUpdateResult::InvalidInput);
    }

    #[test]
    fn correctly_mark_letters() {
        let mut state = setup();
        state.push_letter('t');
        state.push_letter('e');
        state.push_letter('x');
        state.push_letter('s');
        state.commit_guess();
        assert_eq!(
            state.guesses.last().unwrap().letters(),
            &vec![
                GuessedLetter::new('t', LetterState::RightPosition),
                GuessedLetter::new('e', LetterState::RightPosition),
                GuessedLetter::new('x', LetterState::NotOccurring),
                GuessedLetter::new('s', LetterState::WrongPosition)
            ]
        );

        assert_eq!(state.letters.state('t'), LetterState::RightPosition);
        assert_eq!(state.letters.state('e'), LetterState::RightPosition);
        assert_eq!(state.letters.state('x'), LetterState::NotOccurring);
        assert_eq!(state.letters.state('s'), LetterState::WrongPosition);
        assert_eq!(state.letters.state('u'), LetterState::NotChecked);
    }

    #[test]
    fn should_allow_to_win_game() {
        let mut state = setup();
        state.push_letter('t');
        state.push_letter('e');
        state.push_letter('s');
        state.commit_guess();
        let mut state = setup();
        state.push_letter('t');
        state.push_letter('e');
        state.push_letter('s');
        state.push_letter('t');
        let result = state.commit_guess();
        assert_eq!(result, GameUpdateResult::Ok);
        assert_eq!(state.status, crate::game_state::GameStatus::Won);
    }

    #[test]
    fn should_allow_to_lose_game() {
        let mut state = setup();
        state.push_letter('t');
        state.push_letter('e');
        state.push_letter('s');
        state.push_letter('x');
        state.commit_guess();
        let mut state = setup();
        state.push_letter('t');
        state.push_letter('e');
        state.push_letter('x');
        state.push_letter('t');
        let mut state = setup();
        state.push_letter('x');
        state.push_letter('e');
        state.push_letter('x');
        state.push_letter('t');
        let result = state.commit_guess();
        assert_eq!(result, GameUpdateResult::Ok);
        assert_eq!(state.status, crate::game_state::GameStatus::Lost);
    }

}