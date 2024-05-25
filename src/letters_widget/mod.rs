use ratatui::buffer::Buffer;
use ratatui::layout::*;
use ratatui::prelude::*;
use crate::letter_box_widget::LetterBoxWidget;
use crate::letters::Letters;

#[derive(Clone)]
pub struct LettersWidget {
    letters: Letters,
    layout: Vec<Vec<char>>
}

impl LettersWidget {
    pub fn new(letters: Letters) -> Self {

        let layout = vec![
            vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
            vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
        ];

        Self{
            letters,
            layout
        }
    }
}

impl  Widget for LettersWidget {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {

        let letters_layout = &self.layout;

        let max_len = letters_layout.iter().map(|l| l.len()).max().unwrap_or(0) as u32;
        let max_height = (area.height / self.layout.len() as u16).max(2) - 1;

        let lines = Layout::default()
            .direction(Direction::Vertical)
            .constraints(letters_layout.iter().map(|_| Constraint::Length(max_height)).collect::<Vec<_>>())
            .spacing(1)
            .split(area);

        for (line_idx, letters) in letters_layout.iter().enumerate() {

            let line = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25)
                ])
                .split(lines[line_idx]);

            let letter_boxes = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints(letters.iter().map(|_| Constraint::Ratio(1 ,max_len)).collect::<Vec<_>>())
                .spacing(1)
                .split(line[1]);

            for (letter_idx, letter) in letters.iter().enumerate() {

                let state = self.letters.state(*letter);

                LetterBoxWidget::new(*letter, state)
                    .render(letter_boxes[letter_idx], buf);

            }
        }

    }
}