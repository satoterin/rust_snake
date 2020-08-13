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
    shape: Vec<SnakeUnit>,
    direction: SnakeDirection,
}

struct SnakeUnit {
    x: f64,
    y: f64,
}

impl Clone for SnakeUnit {
    fn clone(&self) -> Self {
        SnakeUnit {
            x: self.x,
            y: self.y,
        }
    }
}

enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

enum GameState {
    Done,
    Running,
}

const UNIT: f64 = 5.0;
const BOUND: f64 = 100.0;

impl Snake {
    fn update(&mut self) {
        match self.direction {
            SnakeDirection::Up => {
                let first = self.shape.get(0).unwrap().clone();
                let SnakeUnit { y, .. } = first;
                let y = if y > (BOUND - UNIT) { -BOUND } else { y + UNIT };
                self.shape.insert(0, SnakeUnit { y, ..first });
                self.shape.pop();
            }
            SnakeDirection::Down => {
                let first = self.shape.first().unwrap().clone();
                let SnakeUnit { y, .. } = first;
                let y = if y < (-BOUND + UNIT) { BOUND } else { y - UNIT };
                self.shape.pop();
                self.shape.insert(0, SnakeUnit { y, ..first });
            }
            SnakeDirection::Right => {
                let first = self.shape.first().unwrap().clone();
                let SnakeUnit { x, .. } = first;
                let x = if x > (BOUND - UNIT) { -BOUND } else { x + UNIT };
                self.shape.pop();
                self.shape.insert(0, SnakeUnit { x, ..first });
            }
            SnakeDirection::Left => {
                let first = self.shape.first().unwrap().clone();
                let SnakeUnit { x, .. } = first;
                let x = if x < (-BOUND + UNIT) { BOUND } else { x - UNIT };
                self.shape.pop();
                self.shape.insert(0, SnakeUnit { x, ..first });
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
        shape: vec![
            SnakeUnit {
                x: 0.0,
                y: 0.0 + 4.0 * UNIT,
            },
            SnakeUnit {
                x: 0.0,
                y: 0.0 + 3.0 * UNIT,
            },
            SnakeUnit {
                x: 0.0,
                y: 0.0 + 2.0 * UNIT,
            },
            SnakeUnit {
                x: 0.0,
                y: 0.0 + UNIT,
            },
            SnakeUnit { x: 0.0, y: 0.0 },
        ],
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
        render_screen(&mut terminal, &mut snake)?;

        // Event handling
        match event_handler(&mut terminal, &mut snake, &rx) {
            Ok(GameState::Done) => break Ok(()),
            Ok(_) => continue,
            Err(_) => break Ok(()),
        }
    }
}

fn event_handler<B>(
    terminal: &mut Terminal<B>,
    snake: &mut Snake,
    rx: &mpsc::Receiver<termion::event::Key>,
) -> Result<GameState, io::Error>
where
    B: Backend,
{
    let mut events = rx.try_iter();
    match events.next() {
        Some(event) => match event {
            Key::Char('q') => {
                terminal.clear()?;
                Ok(GameState::Done)
            }
            Key::Up => {
                snake.direction = SnakeDirection::Up;
                Ok(GameState::Running)
            }
            Key::Down => {
                snake.direction = SnakeDirection::Down;
                Ok(GameState::Running)
            }
            Key::Right => {
                snake.direction = SnakeDirection::Right;
                Ok(GameState::Running)
            }
            Key::Left => {
                snake.direction = SnakeDirection::Left;
                Ok(GameState::Running)
            }
            _ => Ok(GameState::Running),
        },
        None => Ok(GameState::Running),
    }
}

fn render_screen<B>(terminal: &mut Terminal<B>, snake: &mut Snake) -> Result<(), io::Error>
where
    B: Backend,
{
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
            .x_bounds([-BOUND, BOUND])
            .y_bounds([-BOUND, BOUND])
            .paint(|ctx| {
                snake.shape.iter().for_each(|x| {
                    let rect = Rectangle {
                        x: x.x,
                        y: x.y,
                        width: UNIT,
                        height: UNIT,
                        color: Color::Red,
                    };
                    ctx.draw(&rect);
                })
            });
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
    })
}
