use crate::game_state::GameUpdateResult;
use crate::letters::{GuessedLetter, LetterState};
pub trait Guess {

    fn is_pending(&self) -> bool;
    fn letters(&self) -> &Vec<GuessedLetter>;
}

pub struct EmptyGuess;

static EMPTY: Vec<GuessedLetter> = Vec::new();

impl Guess for EmptyGuess {


    fn is_pending(&self) -> bool {
        false
    }

    fn letters(&self) -> &Vec<GuessedLetter> {
        &EMPTY
    }
}

#[derive(Clone)]
pub struct CompletedGuess {
    letters: Vec<GuessedLetter>
}

impl Guess for CompletedGuess {
    fn is_pending(&self) -> bool {
        false
    }

    fn letters(&self) -> &Vec<GuessedLetter> {
        &self.letters
    }
}

impl CompletedGuess {
    pub(crate) fn is_all_correct(&self) -> bool {
        self.letters.iter().all(|l| l.state == LetterState::RightPosition)
    }
}

#[derive(Clone)]
pub struct PendingGuess {
    letters: Vec<GuessedLetter>,
    target_word: String
}

impl PendingGuess {

    pub(crate) fn new(target_word: String) -> Self {
        Self{
            letters: Vec::new(),
            target_word
        }
    }

    pub(crate) fn complete(&self) -> CompletedGuess {

        let letters = self.letters.iter().zip(self.target_word.chars()).map(|(l, c)| {
            if l.value == c {
                GuessedLetter::new(l.value, LetterState::RightPosition)
            } else if self.target_word.contains(l.value) {
                GuessedLetter::new(l.value, LetterState::WrongPosition)
            } else {
                GuessedLetter::new(l.value, LetterState::NotOccurring)
            }
        }).collect();

        CompletedGuess { letters }
    }

    pub(crate) fn full(&self) -> bool {
        self.letters.len() == self.target_word.len()
    }

    pub(crate) fn push_letter(&mut self, letter: char) -> GameUpdateResult {
        if self.full() {
            GameUpdateResult::InvalidInput
        } else {
            self.letters.push(GuessedLetter::new(letter, LetterState::default()));
            GameUpdateResult::Ok
        }
    }

    pub(crate) fn pop_letter(&mut self) -> GameUpdateResult {
        if self.letters.len() > 0 {
            self.letters.pop();
            GameUpdateResult::Ok
        } else {
            GameUpdateResult::InvalidInput
        }
    }
}

impl Guess for PendingGuess {
    fn is_pending(&self) -> bool {
        true
    }

    fn letters(&self) -> &Vec<GuessedLetter> {
        &self.letters
    }
}