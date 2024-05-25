use std::io;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::game_manager::{GameManager, GameManagerState};
use crate::game_state::{GameStatus, GameUpdateResult};

fn beep() {
    println!("\x07")
}

pub fn handle_events(manager: &mut GameManager) -> io::Result<bool> {

    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match &mut manager.state {
                    GameManagerState::Playing(state) => {
                        match &state.status {
                            GameStatus::Won | GameStatus::Lost => {
                                match key.code {
                                    KeyCode::Enter => {
                                        manager.restart();
                                    },
                                    KeyCode::Esc => return Ok(true),
                                    _ => {}
                                }
                            },
                            _ => {
                                let correct_action = match key.code {
                                    KeyCode::Esc => return Ok(true),
                                    KeyCode::Backspace => state.pop_letter(),
                                    KeyCode::Enter => state.commit_guess(),
                                    KeyCode::Char(c) => state.push_letter(c.to_ascii_lowercase()),
                                    _ => GameUpdateResult::Ok
                                };

                                if correct_action == GameUpdateResult::InvalidInput {
                                    beep();
                                }
                            }

                        }

                    },
                    GameManagerState::Loading => {
                        match key.code {
                            KeyCode::Esc => return Ok(true),
                            _ => {}
                        }
                    }
                    GameManagerState::Failure(_)  => {
                        match key.code {
                            KeyCode::Esc => return Ok(true),
                            KeyCode::Enter => manager.restart(),
                            _ => {}
                        }
                    }
                }
            }
        }
    }


    Ok(false)
}