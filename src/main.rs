use std::{io, thread, time::Duration};
use tui::{
    backend::Backend, backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal, Frame,
}; 
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};



fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }

    thread::sleep(Duration::from_millis(5000));

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

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            ].as_ref()
        ).split(size);
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
   f.render_widget(block, chunks[0]);
   let block = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);
   f.render_widget(block, chunks[1]);
}