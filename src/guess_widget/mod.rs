use ratatui::prelude::*;
use ratatui::layout::Flex;
use crate::game_state::GameState;
use crate::letter_box_widget::LetterBoxWidget;
use crate::letters::GuessedLetter;

pub struct GuessWidget<'a>{
    game_state: &'a GameState,
}

impl <'a> GuessWidget<'a> {
    pub fn new(game_state: &'a mut GameState) -> Self {
        Self{ game_state }
    }

}

impl <'a> Widget for GuessWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {

        let max_tries = self.game_state.max_tries;
        let word_len = self.game_state.target_word.len();

        let max_height = (area.height / max_tries as u16).max(2) - 1;

        let lines = Layout::default()
            .direction(Direction::Vertical)
            .constraints((0..max_tries).into_iter().map(|_| Constraint::Length(max_height)).collect::<Vec<_>>())
            .spacing(1)
            .split(area);

        for line_idx in 0..max_tries as usize {

            let guess = self.game_state.get(line_idx);
            let letters = guess.letters();

            let line = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(35),
                    Constraint::Percentage(30),
                    Constraint::Percentage(35)
                ])
                .spacing(1)
                .split(lines[line_idx]);

            let letter_boxes = Layout::default()
                .direction(Direction::Horizontal)
                .constraints((0..word_len).into_iter().map(|_| Constraint::Ratio(1, word_len as u32)).collect::<Vec<_>>())
                .flex(Flex::SpaceBetween)
                .spacing(8)
                .split(line[1]);

            for letter_idx in 0..word_len {

                let letter = letters.get(letter_idx).cloned().unwrap_or(GuessedLetter::default());

                LetterBoxWidget::new(letter.value, letter.state)
                    .render(letter_boxes[letter_idx], buf);

            }
        }
    }
}