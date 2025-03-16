use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::{canvas::Canvas, canvas::Points, Block}};
use ratatui::symbols::Marker;

mod states;


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = states::App::new();
    let _ = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: states::App) -> io::Result<()> {
    // main loop - drawing and handling events
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            // handle keyboard events
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
        if let Event::Mouse(event) = event::read()? {
            // handle mouse events
            match event.kind {
                MouseEventKind::Down(_) => {}, //self.is_drawing = true,
                MouseEventKind::Up(_) => {}, //self.is_drawing = false,
                MouseEventKind::Drag(_) => {
                    //self.points.push(Position::new(event.column, event.row));
                }
                _ => {}
            }
        }
    }
}


// main drawing function
pub fn ui(f: &mut Frame, app: &mut states::App) {
    let area = f.area();

    let board = Canvas::default()
        .block(Block::bordered().title("Draw here"))
        .marker(Marker::Bar)
        .x_bounds([0.0, f64::from(area.width)])
        .y_bounds([0.0, f64::from(area.height)])
        .paint(move |ctx| {
            let points: Vec<(f64, f64)> = app.get_points().iter().map(|p| (f64::from(p.0), f64::from(p.1))).collect();

            ctx.draw(&Points {
                coords: &points,
                color: Color::White,
            });
        });

    f.render_widget(board, area);
}