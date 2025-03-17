use ratatui::{prelude::*, widgets::*};
use crate::states::App;

pub fn render(f: &mut Frame, _app: &mut App, area: Rect) {
    let controls = vec![
        " Quit ".into(), "<Q>".green().bold(), " | ".into(),
        " Start ".into(), "<S>".green().bold(), " | ".into(),
        " End ".into(), "<E>".green().bold(), " | ".into(),
        " Clear ".into(), "<C>".green().bold(), " | ".into(),
        " Toggle Drawing Mode ".into(), "<T>".green().bold(),
    ];

    let paragraph = Paragraph::new(Line::from(controls))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
