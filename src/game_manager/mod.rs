use minreq::Error;
use crate::game_manager::GameManagerState::Loading;
use crate::game_state::GameState;


pub enum GameManagerState {
    Loading,
    Failure(String),
    Playing(GameState),
}
pub struct GameManager {
    pub state: GameManagerState
}

const WORD_LEN: u8 = 5;
const MAX_TRIES: u8 = 6;

impl GameManager {
    pub fn new() -> Self {
        Self {
            state: Loading
        }
    }

    fn fetch_word() -> Result<String, Error> {
        let response = minreq::get(format!("https://random-word-api.herokuapp.com/word?length={}", WORD_LEN)).send()?;
        let json = response.json::<Vec<String>>()?;
        json.first().map(|word| word.to_string()).ok_or(Error::Other("Didn't receive word from the API"))
    }

    pub fn restart(&mut self) {

        match GameManager::fetch_word() {
            Ok(word) => self.state = GameManagerState::Playing(GameState::new(&word, MAX_TRIES)),
            Err(_) => self.state = GameManagerState::Failure("Failed to fetch your word".to_string())
        }

    }

}