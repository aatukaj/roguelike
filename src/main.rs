use std::{error::Error, io, iter};
mod game;
mod map;
mod render;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use game::*;
use glam::ivec2;
use map::*;
use ratatui::{prelude::*, widgets::*};
use render::RenderWidget;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = Game::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: Game) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('h') => MovePlayer(ivec2(-1, 0)).execute(&mut app),
                    KeyCode::Char('j') => MovePlayer(ivec2(0, 1)).execute(&mut app),
                    KeyCode::Char('k') => MovePlayer(ivec2(0, -1)).execute(&mut app),
                    KeyCode::Char('l') => MovePlayer(ivec2(1, 0)).execute(&mut app),
                    KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut Game) {
    let size = f.size();
    f.buffer_mut()
        .set_style(size, Style::default().bg(Color::Rgb(0, 0, 0)));
    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Fill(1), Constraint::Length(7)],
    )
    .split(f.size());
    f.render_stateful_widget(RenderWidget, layout[0], app);
    let block = Block::bordered().title("UI");
    f.render_widget(block, layout[1]);
}
