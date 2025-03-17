use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, canvas::Points, Block},
    symbols::Marker,
};
use crate::states::App;

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let board = Canvas::default()
        .block(Block::bordered().title("Simulation Board"))
        .marker(Marker::Dot)
        .x_bounds([0.0, f64::from(area.width)])   // X remains the same
        .y_bounds([0.0, f64::from(area.height)])  // Y remains the same
        .paint(move |ctx| {
            let height = f64::from(area.height); // Get max height

            let points: Vec<(f64, f64)> = app
                .get_points()
                .iter()
                .map(|p| (f64::from(p.0), height - f64::from(p.1))) // Invert Y-axis
                .collect();

            ctx.draw(&Points {
                coords: &points,
                color: Color::White,
            });
        });

    f.render_widget(board, area);
}
