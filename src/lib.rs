use std::io;
use std::sync::mpsc;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::Color;
use tui::text::{Span, Spans};
use tui::widgets::canvas::{Canvas, Rectangle};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::Terminal;

pub fn run<B>(mut terminal: Terminal<B>) -> Result<(), io::Error>
where
    B: Backend,
{
    terminal.hide_cursor()?;
    terminal.clear()?;
    let mut rect = Rectangle {
        x: 10.0,
        y: 10.0,
        width: 10.0,
        height: 10.0,
        color: Color::Red,
    };
    let (tx, rx) = mpsc::channel();
    let _input_handle = {
        let tx = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                match evt {
                    Ok(key) => {
                        if let Err(_) = tx.send(key) {
                            return;
                        }
                        if key == Key::Char('q') {
                            return;
                        }
                    }
                    Err(_) => {}
                }
            }
        })
    };
    loop {
        // Rendering the frame
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.size());
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                )
                .x_bounds([-180.0, 180.0])
                .y_bounds([-180.0, 180.0])
                .paint(|ctx| ctx.draw(&rect));
            f.render_widget(canvas, chunks[0]);
            let text = vec![
                Spans::from(""),
                Spans::from(Span::raw("Welcome to the game")),
                Spans::from(Span::raw("This is how to play the game")),
                Spans::from(""),
                Spans::from(Span::raw("Move up: up")),
                Spans::from(Span::raw("Move down: down")),
                Spans::from(Span::raw("Move left: left")),
                Spans::from(Span::raw("Move right: right")),
                Spans::from(""),
                Spans::from(Span::raw("Quit the game: q")),
            ];
            let block = Paragraph::new(text)
                .block(Block::default().title("How to play").borders(Borders::ALL))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(block, chunks[1]);
        })?;

        // Event handling
        let mut events = rx.iter();
        match events.next() {
            Some(event) => match event {
                Key::Char('q') => {
                    terminal.clear()?;
                    break Ok(());
                }
                Key::Up => {
                    let Rectangle { y, .. } = rect;
                    rect = Rectangle { y: y + 1.0, ..rect }
                }
                Key::Down => {
                    let Rectangle { y, .. } = rect;
                    rect = Rectangle { y: y - 1.0, ..rect }
                }
                Key::Right => {
                    let Rectangle { x, .. } = rect;
                    rect = Rectangle { x: x + 1.0, ..rect }
                }
                Key::Left => {
                    let Rectangle { x, .. } = rect;
                    rect = Rectangle { x: x - 1.0, ..rect }
                }
                _ => {}
            },
            None => {
                continue;
            }
        }
    }
}
