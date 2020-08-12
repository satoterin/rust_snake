use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::Color;
use tui::text::{Span, Spans};
use tui::widgets::canvas::{Canvas, Rectangle};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::Terminal;

struct Snake {
    shape: Rectangle,
    direction: SnakeDirection,
}

enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

const UNIT: f64 = 5.0;

impl Snake {
    fn update(&mut self) {
        match self.direction {
            SnakeDirection::Up => {
                let Snake {
                    shape: Rectangle { y, .. },
                    ..
                } = self;
                let y = if *y > (100.0 - UNIT) {
                    -100.0
                } else {
                    *y + UNIT
                };
                self.shape = Rectangle { y, ..self.shape };
            }
            SnakeDirection::Down => {
                let Snake {
                    shape: Rectangle { y, .. },
                    ..
                } = self;
                let y = if *y < (-100.0 + UNIT) {
                    100.0
                } else {
                    *y - UNIT
                };
                self.shape = Rectangle { y, ..self.shape };
            }
            SnakeDirection::Right => {
                let Snake {
                    shape: Rectangle { x, .. },
                    ..
                } = self;
                let x = if *x > (100.0 - UNIT) {
                    -100.0
                } else {
                    *x + UNIT
                };
                self.shape = Rectangle { x, ..self.shape };
            }
            SnakeDirection::Left => {
                let Snake {
                    shape: Rectangle { x, .. },
                    ..
                } = self;
                let x = if *x < (-100.0 + UNIT) {
                    100.0
                } else {
                    *x - UNIT
                };
                self.shape = Rectangle { x, ..self.shape };
            }
        }
    }
}

pub fn run<B>(mut terminal: Terminal<B>) -> Result<(), io::Error>
where
    B: Backend,
{
    terminal.hide_cursor()?;
    terminal.clear()?;
    let mut snake = Snake {
        shape: Rectangle {
            x: 0.0,
            y: 0.0,
            width: UNIT,
            height: UNIT,
            color: Color::Red,
        },
        direction: SnakeDirection::Up,
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
        snake.update();
        thread::sleep(Duration::from_millis(100));
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
                .x_bounds([-100.0, 100.0])
                .y_bounds([-100.0, 100.0])
                .paint(|ctx| ctx.draw(&snake.shape));
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
        let mut events = rx.try_iter();
        match events.next() {
            Some(event) => match event {
                Key::Char('q') => {
                    terminal.clear()?;
                    break Ok(());
                }
                Key::Up => {
                    snake.direction = SnakeDirection::Up;
                }
                Key::Down => {
                    snake.direction = SnakeDirection::Down;
                }
                Key::Right => {
                    snake.direction = SnakeDirection::Right;
                }
                Key::Left => {
                    snake.direction = SnakeDirection::Left;
                }
                _ => {}
            },
            None => {
                continue;
            }
        }
    }
}
