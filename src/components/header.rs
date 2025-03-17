use ratatui::{prelude::*, widgets::*};
use ratatui::style::{Modifier, Style};
use crate::states::{App, Mode};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Title centered
    let title = Paragraph::new("Game of Life")
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));

    // Status line: either "Drawing" in yellow or simulation stats
    let mut status_line = vec!["Mode: ".into()];

    status_line.push(match app.mode {
        Mode::Drawing => "Drawing".yellow().bold(),
        Mode::Simulating => "Simulating".green().bold(),
        Mode::None => "Idle".green().bold(),
    });
    
    if app.mode != Mode::Drawing {
        status_line.extend(vec![
            " | ".into(),
            "Generation: ".into(), app.generation.to_string().green().bold(), " | ".into(),
            "Live Cells: ".into(), app.live_cells.to_string().green().bold(),
        ]);
    }
    
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
