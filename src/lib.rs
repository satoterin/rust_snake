use rand::Rng;
use std::fmt;
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
    eaten: Vec<SnakeUnit>,
    score: i32,
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
    Stopped,
}

enum GameState {
    Done,
    Running,
}

#[derive(Debug, Clone)]
struct CollisionError;

impl fmt::Display for CollisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was a collision detected")
    }
}

const UNIT: f64 = 5.0;
const RANGE: f64 = 20.0;
const BOUND: f64 = UNIT * RANGE;

impl Snake {
    fn update(&mut self, food: &mut SnakeUnit) -> Result<(), CollisionError> {
        match self.direction {
            SnakeDirection::Up => {
                let first = self.shape.get(0).unwrap().clone();
                if first == *food {
                    self.eaten.insert(
                        0,
                        SnakeUnit {
                            x: food.x,
                            y: food.y,
                        },
                    );
                    self.score += 10;
                    let new_food = random_snake_unit(&self);
                    food.x = new_food.x;
                    food.y = new_food.y;
                }
                let SnakeUnit { y, .. } = first;
                let y = if y > (BOUND - UNIT) { -BOUND } else { y + UNIT };
                if let (Some(eaten), Some(last)) = (self.eaten.last(), self.shape.pop()) {
                    if last == *eaten {
                        self.shape.push(last);
                        self.eaten.pop();
                    }
                }
                let new_block = SnakeUnit { y, ..first };
                if self.occupied(&new_block) {
                    Err(CollisionError)
                } else {
                    self.shape.insert(0, new_block);
                    Ok(())
                }
            }
            SnakeDirection::Down => {
                let first = self.shape.first().unwrap().clone();
                if first == *food {
                    self.eaten.insert(
                        0,
                        SnakeUnit {
                            x: food.x,
                            y: food.y,
                        },
                    );
                    self.score += 10;
                    let new_food = random_snake_unit(&self);
                    food.x = new_food.x;
                    food.y = new_food.y;
                }
                let SnakeUnit { y, .. } = first;
                let y = if y < (-BOUND + UNIT) { BOUND } else { y - UNIT };
                if let (Some(eaten), Some(last)) = (self.eaten.last(), self.shape.pop()) {
                    if last == *eaten {
                        self.shape.push(last);
                        self.eaten.pop();
                    }
                }
                let new_block = SnakeUnit { y, ..first };
                if self.occupied(&new_block) {
                    Err(CollisionError)
                } else {
                    self.shape.insert(0, new_block);
                    Ok(())
                }
            }
            SnakeDirection::Right => {
                let first = self.shape.first().unwrap().clone();
                if first == *food {
                    self.eaten.insert(
                        0,
                        SnakeUnit {
                            x: food.x,
                            y: food.y,
                        },
                    );
                    self.score += 10;
                    let new_food = random_snake_unit(&self);
                    food.x = new_food.x;
                    food.y = new_food.y;
                }
                let SnakeUnit { x, .. } = first;
                let x = if x > (BOUND - UNIT) { -BOUND } else { x + UNIT };
                if let (Some(eaten), Some(last)) = (self.eaten.last(), self.shape.pop()) {
                    if last == *eaten {
                        self.shape.push(last);
                        self.eaten.pop();
                    }
                }
                let new_block = SnakeUnit { x, ..first };
                if self.occupied(&new_block) {
                    Err(CollisionError)
                } else {
                    self.shape.insert(0, new_block);
                    Ok(())
                }
            }
            SnakeDirection::Left => {
                let first = self.shape.first().unwrap().clone();
                if first == *food {
                    self.eaten.insert(
                        0,
                        SnakeUnit {
                            x: food.x,
                            y: food.y,
                        },
                    );
                    self.score += 10;
                    let new_food = random_snake_unit(&self);
                    food.x = new_food.x;
                    food.y = new_food.y;
                }
                let SnakeUnit { x, .. } = first;
                let x = if x < (-BOUND + UNIT) { BOUND } else { x - UNIT };
                if let (Some(eaten), Some(last)) = (self.eaten.last(), self.shape.pop()) {
                    if last == *eaten {
                        self.shape.push(last);
                        self.eaten.pop();
                    }
                }
                let new_block = SnakeUnit { x, ..first };
                if self.occupied(&new_block) {
                    Err(CollisionError)
                } else {
                    self.shape.insert(0, new_block);
                    Ok(())
                }
            }
            SnakeDirection::Stopped => Ok(()),
        }
    }
    fn occupied(&self, block: &SnakeUnit) -> bool {
        self.shape.iter().any(|x| x.eq(block))
    }
}

fn random_snake_unit(snake: &Snake) -> SnakeUnit {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(-RANGE as i32, RANGE as i32);
        let y = rng.gen_range(-RANGE as i32, RANGE as i32);
        let food = SnakeUnit {
            x: (x as f64) * UNIT,
            y: (y as f64) * UNIT,
        };
        if snake.occupied(&food) {
            continue;
        } else {
            break food;
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
        eaten: Vec::new(),
        score: 0,
    };
    let mut food = random_snake_unit(&snake);
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
        match snake.update(&mut food) {
            Ok(()) => {}
            Err(CollisionError) => {
                snake.direction = SnakeDirection::Stopped;
            }
        };
        thread::sleep(Duration::from_millis(100));
        render_screen(&mut terminal, &snake, &food)?;

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
                if snake.direction != SnakeDirection::Up
                    && snake.direction != SnakeDirection::Down
                    && snake.direction != SnakeDirection::Stopped
                {
                    snake.direction = SnakeDirection::Up;
                }
                Ok(GameState::Running)
            }
            Key::Down => {
                if snake.direction != SnakeDirection::Up
                    && snake.direction != SnakeDirection::Down
                    && snake.direction != SnakeDirection::Stopped
                {
                    snake.direction = SnakeDirection::Down;
                }
                Ok(GameState::Running)
            }
            Key::Right => {
                if snake.direction != SnakeDirection::Left
                    && snake.direction != SnakeDirection::Right
                    && snake.direction != SnakeDirection::Stopped
                {
                    snake.direction = SnakeDirection::Right;
                }
                Ok(GameState::Running)
            }
            Key::Left => {
                if snake.direction != SnakeDirection::Left
                    && snake.direction != SnakeDirection::Right
                    && snake.direction != SnakeDirection::Stopped
                {
                    snake.direction = SnakeDirection::Left;
                }
                Ok(GameState::Running)
            }
            _ => Ok(GameState::Running),
        },
        None => Ok(GameState::Running),
    }
}

fn render_screen<B>(
    terminal: &mut Terminal<B>,
    snake: &Snake,
    food: &SnakeUnit,
) -> Result<(), io::Error>
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
                });
                ctx.draw(&Rectangle {
                    x: food.x,
                    y: food.y,
                    width: UNIT,
                    height: UNIT,
                    color: Color::Green,
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
            Spans::from(""),
            Spans::from(Span::raw(format!("Score: {}", snake.score))),
        ];
        let block = Paragraph::new(text)
            .block(Block::default().title("How to play").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(block, chunks[1]);
    })
}
