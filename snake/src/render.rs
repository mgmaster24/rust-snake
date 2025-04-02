use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToColumn, Show},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize},
    ExecutableCommand,
};

use crate::{direction::Direction, point::Point, snake::Snake};

pub struct Renderer {
    height: u16,
    width: u16,
    stdout: Stdout,
    initial_term_size: (u16, u16),
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        let initial_term_size = size().unwrap();
        Renderer {
            height,
            width,
            stdout: stdout(),
            initial_term_size,
        }
    }

    pub fn init(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Hide)
            .unwrap();
    }

    pub fn restore(&mut self) {
        let (cols, rows) = self.initial_term_size;
        self.stdout
            .execute(SetSize(cols, rows))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Show)
            .unwrap()
            .execute(ResetColor)
            .unwrap();
        disable_raw_mode().unwrap();
    }

    pub fn render(&mut self, food: &Option<Point>, snake: &Snake) {
        self.draw_borders();
        self.draw_bg();
        self.draw_food(food);
        self.draw_snake(snake);
    }

    fn draw_bg(&mut self) {
        self.stdout.execute(ResetColor).unwrap();

        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                self.stdout
                    .execute(MoveTo(x, y))
                    .unwrap()
                    .execute(Print(" "))
                    .unwrap();
            }
        }
    }

    fn draw_borders(&mut self) {
        self.stdout
            .execute(SetForegroundColor(Color::DarkGrey))
            .unwrap();
        for y in 0..self.height + 2 {
            self.stdout
                .execute(MoveTo(0, y))
                .unwrap()
                .execute(Print("#"))
                .unwrap()
                .execute(MoveTo(self.width + 1, y))
                .unwrap()
                .execute(Print("#"))
                .unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .execute(MoveTo(x, 0))
                .unwrap()
                .execute(Print("#"))
                .unwrap()
                .execute(MoveTo(x, self.height + 1))
                .unwrap()
                .execute(Print("#"))
                .unwrap();
        }

        self.stdout
            .execute(MoveTo(0, 0))
            .unwrap()
            .execute(Print("#"))
            .unwrap()
            .execute(MoveTo(self.width + 1, self.height + 1))
            .unwrap()
            .execute(Print("#"))
            .unwrap()
            .execute(MoveTo(self.width + 1, 0))
            .unwrap()
            .execute(Print("#"))
            .unwrap()
            .execute(MoveTo(0, self.height + 1))
            .unwrap()
            .execute(Print("#"))
            .unwrap();
    }

    fn draw_food(&mut self, food: &Option<Point>) {
        self.stdout
            .execute(SetForegroundColor(Color::White))
            .unwrap();

        for food in food.iter() {
            self.stdout
                .execute(MoveTo(food.x + 1, food.y + 1))
                .unwrap()
                .execute(Print("•"))
                .unwrap();
        }
    }

    fn draw_snake(&mut self, snake: &Snake) {
        let fg = SetForegroundColor(match snake.get_speed() % 3 {
            0 => Color::Green,
            1 => Color::Cyan,
            _ => Color::Yellow,
        });

        self.stdout.execute(fg).unwrap();
        let body_pts = snake.get_body();
        for (i, body) in body_pts.iter().enumerate() {
            let previous = if i == 0 { None } else { body_pts.get(i - 1) };
            let next = body_pts.get(i + 1);
            let symbol = if let Some(&next) = next {
                if let Some(&previous) = previous {
                    if previous.x == next.x {
                        '║'
                    } else if previous.y == next.y {
                        '═'
                    } else {
                        let d = body.transform(Direction::Down, 1);
                        let r = body.transform(Direction::Right, 1);
                        let u = if body.y == 0 {
                            *body
                        } else {
                            body.transform(Direction::Up, 1)
                        };
                        let l = if body.x == 0 {
                            *body
                        } else {
                            body.transform(Direction::Left, 1)
                        };
                        if (next == d && previous == r) || (previous == d && next == r) {
                            '╔'
                        } else if (next == d && previous == l) || (previous == d && next == l) {
                            '╗'
                        } else if (next == u && previous == r) || (previous == u && next == r) {
                            '╚'
                        } else {
                            '╝'
                        }
                    }
                } else {
                    'O'
                }
            } else if let Some(&previous) = previous {
                if body.y == previous.y {
                    '═'
                } else {
                    '║'
                }
            } else {
                self.restore();
                panic!("Invalid snake body point.");
            };

            self.stdout
                .execute(MoveTo(body.x + 1, body.y + 1))
                .unwrap()
                .execute(Print(symbol))
                .unwrap();
        }
    }

    pub fn draw_gameover(&mut self, score: u16) {
        let gameover = vec![
            "█▀▀ ▄▀█ █▀▄▀█ █▀▀",
            "█▄█ █▀█ █░▀░█ ██▄",
            "",
            "█▀█ █░█ █▀▀ █▀█",
            "█▄█ ▀▄▀ ██▄ █▀▄",
        ];

        self.stdout.execute(Clear(ClearType::All)).unwrap();
        for (i, line) in gameover.into_iter().enumerate() {
            self.stdout
                .execute(MoveTo(0, i as u16))
                .unwrap()
                .execute(Print(line))
                .unwrap()
                .execute(Print("\n"))
                .unwrap();
        }

        self.stdout
            .execute(MoveToColumn(0))
            .unwrap()
            .execute(Print("\n"))
            .unwrap()
            .execute(Print(format!("Your score: {}\n", score)))
            .unwrap()
            .execute(MoveToColumn(0))
            .unwrap()
            .execute(Print("Press any key to continue"))
            .unwrap();
    }
}
