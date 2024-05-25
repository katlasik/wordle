use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LetterState {
    RightPosition,
    WrongPosition,
    NotChecked,
    NotOccurring
}

impl Default for LetterState {
    fn default() -> Self {
        LetterState::NotChecked
    }
}

#[derive(Clone, Debug)]
pub struct GuessedLetter {
    pub value: char,
    pub state: LetterState
}


impl GuessedLetter {
    pub fn new(value: char, state: LetterState) -> Self {
        Self{ value, state }
    }
}

impl Default for GuessedLetter {
    fn default() -> Self {
        Self { value: ' ', state: LetterState::NotChecked }
    }
}

#[derive(Clone)]
pub struct Letters {
    values: HashMap<char, LetterState>
}

impl Letters {
    pub fn new() -> Self {
        let values = ('a'..'z').into_iter().map(|c| (c, LetterState::NotChecked)).collect();
        Self{ values }
    }

    pub fn state(&self, letter: char) -> LetterState {
        self.values.get(&letter).unwrap_or(&LetterState::NotChecked).clone()
    }

    pub fn update(&mut self, letter: char, letter_state: LetterState) {
        match self.values.get(&letter) {
            Some(LetterState::RightPosition) => {},
            _ => {
                self.values.insert(letter, letter_state);
            }
        }
    }

    pub fn update_many(&mut self, letters: &Vec<GuessedLetter>) {
        for letter in letters {
            self.update(letter.value, letter.state.clone());
        }
    }

}