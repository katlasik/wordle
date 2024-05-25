use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style, Widget};
use ratatui::widgets::{Block, Paragraph};
use crate::letters::LetterState;
use ratatui::prelude::*;

pub struct LetterBoxWidget {
    letter: char,
    state: LetterState
}

impl LetterBoxWidget {
    pub fn new(letter: char, state: LetterState) -> Self {
        Self { letter, state }
    }
}

impl Widget for LetterBoxWidget {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {

        let mut letter_block = Block::default();

        letter_block = match self.state {
            LetterState::RightPosition =>
                letter_block.style(Style::default().bg(Color::Green)),
            LetterState::WrongPosition =>
                letter_block.style(Style::default().bg(Color::LightYellow).fg(Color::Black)),
            LetterState::NotChecked =>
                letter_block.style(Style::default().bg(Color::Gray).fg(Color::Black)),
            LetterState::NotOccurring =>
                letter_block.style(Style::default().bg(Color::DarkGray)),
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(3), Constraint::Fill(1)])
            .spacing(1)
            .split(letter_block.inner(area));

        let paragraph = Paragraph::new(self.letter.to_string().to_uppercase())
            .centered()
            .bold();

        paragraph.render(layout[1], buf);

        letter_block.render(area, buf);

    }
}