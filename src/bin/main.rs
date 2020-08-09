use snake_game;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    snake_game::run(Terminal::new(backend).unwrap())
}
