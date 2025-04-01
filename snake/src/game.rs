use crate::command::Command;
use crate::direction::Direction;
use crate::point::Point;
use crate::snake::Snake;
use crossterm::cursor::Hide;
use crossterm::terminal::{enable_raw_mode, size, Clear, ClearType, SetSize};
use crossterm::ExecutableCommand;
use rand::Rng;
use std::io::Stdout;
use std::time::{Duration, Instant};

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

pub struct Game {
    stdout: Stdout,
    init_term_size: (u16, u16),
    width: u16,
    height: u16,
    food: Option<Point>,
    snake: Snake,
    speed: u16,
    score: u16,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let init_term_size: (u16, u16) = size().unwrap();
        Self {
            stdout,
            init_term_size,
            width,
            height,
            food: None,
            snake: Snake::new(
                Point::new(width / 2, height / 2),
                3,
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
        self.prepare_ui();
        self.render();

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

            if self.collided_with_wall() || self.bitten() {
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
                        }
                    }
                }
                self.render();
            }
        }

        self.restore_ui();
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

    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Hide)
            .unwrap();
    }
}
