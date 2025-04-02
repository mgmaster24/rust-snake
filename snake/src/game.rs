use crate::command::Command;
use crate::direction::Direction;
use crate::point::Point;
use crate::render::Renderer;
use crate::snake::Snake;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use rand::Rng;
use std::time::{Duration, Instant};

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

pub struct Game {
    renderer: Renderer,
    width: u16,
    height: u16,
    food: Option<Point>,
    snake: Snake,
    speed: u16,
    score: u16,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            renderer: Renderer::new(width, height),
            width,
            height,
            food: None,
            snake: Snake::new(
                Point::new(width / 2, height / 2),
                3,
                0,
                match rand::rng().random_range(0..4) {
                    0 => Direction::Up,
                    1 => Direction::Right,
                    2 => Direction::Down,
                    _ => Direction::Left,
                },
            ),
            speed: 0,
            score: 0,
        }
    }

    pub fn run(&mut self) {
        self.place_food();
        self.renderer.init();
        self.renderer.render(&self.food, &self.snake);

        let mut done = false;
        while !done {
            let interval = self.calculate_interval();
            let direction = self.snake.get_direction();
            let now = Instant::now();
            while now.elapsed() < interval {
                if let Some(cmd) = self.get_command(interval - now.elapsed()) {
                    match cmd {
                        Command::Quit => {
                            done = true;
                            break;
                        }
                        Command::Turn(toward) => {
                            if direction != toward && direction.opposite() != toward {
                                self.snake.set_direction(toward)
                            }
                        }
                    }
                }
            }

            if self.snake.hit_wall(self.width, self.height) || self.snake.bit_self() {
                done = true;
            } else {
                self.snake.slither();
                if let Some(food_pt) = self.food {
                    if self.snake.get_head() == food_pt {
                        self.snake.grow();
                        self.place_food();
                        self.score += 1;

                        if self.score % ((self.width * self.height) / MAX_SPEED) == 0 {
                            self.speed += 1;
                            self.snake.set_speed(self.speed)
                        }
                    }
                }

                self.renderer.render(&self.food, &self.snake);
            }
        }

        self.renderer.restore();
        println!("Game Over! Your score is {}", self.score);
    }

    fn place_food(&mut self) {
        loop {
            let rand_x = rand::rng().random_range(0..self.width);
            let rand_y = rand::rng().random_range(0..self.height);
            let point = Point::new(rand_x, rand_y);
            if !self.snake.contains(&point) {
                self.food = Some(point);
                break;
            }
        }
    }

    fn calculate_interval(&self) -> Duration {
        let speed = MAX_SPEED - self.speed;
        Duration::from_millis(
            (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64,
        )
    }

    fn wait_for_key(&self, duration: Duration) -> Option<KeyEvent> {
        if poll(duration).ok()? {
            let event = read().ok()?;
            if let Event::Key(key_event) = event {
                return Some(key_event);
            }
        }

        None
    }

    fn get_command(&self, duration: Duration) -> Option<Command> {
        let kevt = self.wait_for_key(duration)?;
        match kevt.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Command::Quit),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if kevt.modifiers == KeyModifiers::CONTROL {
                    Some(Command::Quit)
                } else {
                    None
                }
            }
            KeyCode::Up => Some(Command::Turn(Direction::Up)),
            KeyCode::Right => Some(Command::Turn(Direction::Right)),
            KeyCode::Down => Some(Command::Turn(Direction::Down)),
            KeyCode::Left => Some(Command::Turn(Direction::Left)),
            _ => None,
        }
    }
}
