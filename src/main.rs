mod letters_widget;
mod letters;
mod event_handler;
mod guess;
mod game_state;
mod guess_widget;
mod game_manager;
mod letter_box_widget;

use letters_widget::LettersWidget;
use std::io::{self, stdout};
use event_handler::handle_events;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*};
use ratatui::widgets::{Block, Borders, BorderType, Padding, Paragraph};
use crate::game_manager::{GameManager, GameManagerState};
use crate::game_state::GameStatus;
use crate::guess_widget::GuessWidget;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;

    let mut manager = GameManager::new();
    manager.restart();

    while !should_quit {
        terminal.draw(|frame| ui(&mut manager, frame))?;
        should_quit = handle_events(&mut manager)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(manager: &mut GameManager, frame: &mut Frame) {
    match manager.state {
        GameManagerState::Playing(ref mut state) => {
            let status = state.status.clone();
            let target_word = state.target_word.clone();
            let letters_widget = LettersWidget::new(state.letters.clone());
            let guess_widget = GuessWidget::new(state);

            let (title, title_style) = match status {
                GameStatus::Won =>
                    ("You won! Press enter to start over.".to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                GameStatus::Lost =>
                    (format!("You lost! The correct word was '{}'. Press enter to start over.", target_word), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                _ =>
                    ("Type and press enter to guess. Backspace to clear. Esc to quit.".to_string(), Style::default().fg(Color::White))
            };

            let bottom_block = Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .title_style(title_style)
                .borders(Borders::TOP)
                .padding(Padding::new(1, 1, 1, 1))
                .border_type(BorderType::Double);

            let top_block = Block::default()
                .padding(Padding::new(1, 1, 1, 0));

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(65),
                    Constraint::Percentage(35),
                ])
                .spacing(2)
                .split(frame.size());

            guess_widget.render(top_block.inner(layout[0]), frame.buffer_mut());

            frame.render_widget(
                top_block,
                layout[0]
            );

            letters_widget.render(bottom_block.inner(layout[1]), frame.buffer_mut());

            frame.render_widget(
                bottom_block,
                layout[1]
            );
        },
        GameManagerState::Failure(ref message) =>
            frame.render_widget(
                Paragraph::new(format!("Error: {}. Press enter to retry. Esc to quit.", message))
                    .centered()
                    .style(Style::default().fg(Color::Red))
                    .bold(),
                frame.size()
            ),
        GameManagerState::Loading => {
            frame.render_widget(
                Paragraph::new("Loading...")
                    .centered()
                    .style(Style::default().fg(Color::LightBlue))
                    .bold(),
                frame.size()
            )
        }
    }



}