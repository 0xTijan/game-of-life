use ratatui::{prelude::*, widgets::*};
use crate::states::App;

pub fn render(f: &mut Frame, _app: &mut App, area: Rect) {
    let controls = vec![
        " Quit ".into(), "<Q>".green().bold(), " | ".into(),
        " Simulate ".into(), "<Hold S>".green().bold(), " | ".into(),
        " End ".into(), "<E>".green().bold(), " | ".into(),
        " Clear ".into(), "<C>".green().bold(), " | ".into(),
        " Enable Drawing ".into(), "<D>".green().bold(), " | ".into(),
        " Draw ".into(), "<Left Mouse>".green().bold(),
    ];

    let paragraph = Paragraph::new(Line::from(controls))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
