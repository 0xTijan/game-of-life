use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

mod states;
mod components;


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
                MouseEventKind::Drag(_) => app.draw(event.column, event.row-2),
                _ => {}
            }
        }
    }
}


// main drawing function
pub fn ui(f: &mut Frame, app: &mut states::App) {
    let area = f.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(12),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    components::header::render(f, app, layout[0]);
    components::canvas::render(f, app, layout[1]);
    components::footer::render(f, app, layout[3]);
}