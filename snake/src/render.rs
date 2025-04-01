use std::io::Stdout;

use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

pub struct Renderer {
    height: u16,
    width: u16,
    stdout: Stdout,
}

impl Renderer {
    pub fn draw_bg(&mut self) {
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
    pub fn draw_borders(&mut self) {
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
            .execute(MoveTo(self.width + 2, 0))
            .unwrap()
            .execute(Print("#"))
            .unwrap()
            .execute(MoveTo(0, self.height + 1))
            .unwrap()
            .execute(Print("#"))
            .unwrap();
    }
}
