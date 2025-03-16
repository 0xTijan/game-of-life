use ratatui::{prelude::*, widgets::*};
use ratatui::style::{Modifier, Style};
use crate::states::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Title centered
    let title = Paragraph::new("Game of Life")
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));

    // Status line: either "Drawing" in yellow or simulation stats
    let status_line = if app.is_drawing {
        vec!["Mode: ".into(), "Drawing".yellow().bold()]
    } else {
        vec![
            "Mode: ".into(), "Simulating: ".to_string().green().bold(), " | ".into(),
            "Generation: ".into(), app.generation.to_string().green().bold(), " | ".into(),
            "Live Cells: ".into(), app.live_cells.to_string().green().bold(),
        ]
    };

    let status = Paragraph::new(Line::from(status_line)).alignment(Alignment::Center);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(1), // Status line
        ])
        .split(area);

    f.render_widget(title, chunks[0]);
    f.render_widget(status, chunks[1]);
}
