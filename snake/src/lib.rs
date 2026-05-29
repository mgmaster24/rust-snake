#![cfg(target_arch = "wasm32")]

mod direction;
mod point;
mod snake;

use direction::Direction;
use point::Point;
use snake::Snake;
use wasm_bindgen::prelude::*;

const WIDTH: u16  = 40;
const HEIGHT: u16 = 20;

// Simple xorshift32 seeded from JS Math.random — avoids pulling in getrandom
struct Rng(u32);

impl Rng {
    fn new() -> Self {
        let seed = (js_sys::Math::random() * u32::MAX as f64) as u32;
        Rng(if seed == 0 { 0xdeadbeef } else { seed })
    }

    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }

    fn range(&mut self, max: u16) -> u16 {
        (self.next() % max as u32) as u16
    }
}

#[wasm_bindgen]
pub struct WasmGame {
    snake: Snake,
    food:  Point,
    score: u16,
    speed: u16,
    over:  bool,
    rng:   Rng,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = Rng::new();
        let dir = match rng.range(4) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            _ => Direction::Left,
        };
        let start = Point::new(WIDTH / 2, HEIGHT / 2);
        let snake = Snake::new(start, 3, 0, dir);
        let mut g = WasmGame { snake, food: Point::new(0, 0), score: 0, speed: 0, over: false, rng };
        g.food = g.place_food();
        g
    }

    pub fn input(&mut self, key: &str) {
        let dir = match key {
            "ArrowUp"    | "w" | "W" => Direction::Up,
            "ArrowDown"  | "s" | "S" => Direction::Down,
            "ArrowLeft"  | "a" | "A" => Direction::Left,
            "ArrowRight" | "d" | "D" => Direction::Right,
            _ => return,
        };
        let current = self.snake.get_direction();
        if dir != current && dir != current.opposite() {
            self.snake.set_direction(dir);
        }
    }

    pub fn tick(&mut self) {
        if self.over { return; }

        if self.snake.hit_wall(WIDTH, HEIGHT) || self.snake.bit_self() {
            self.over = true;
            return;
        }

        self.snake.slither();

        if self.snake.eat(self.food) {
            self.score += 1;
            let threshold = (WIDTH * HEIGHT) / 20;
            if self.score % threshold == 0 {
                self.speed += 1;
                self.snake.set_speed(self.speed);
            }
            self.food = self.place_food();
        }
    }

    pub fn is_over(&self) -> bool { self.over }
    pub fn score(&self) -> u16   { self.score }

    /// Returns the current frame as ANSI escape codes for xterm.js to render.
    pub fn render(&self) -> String {
        let mut buf = String::with_capacity(2048);

        // Hide cursor, clear screen
        buf.push_str("\x1b[?25l\x1b[2J");

        // Borders — dark grey
        buf.push_str("\x1b[90m");
        for y in 0..=HEIGHT + 1 {
            write_move(&mut buf, 0,         y);  buf.push('#');
            write_move(&mut buf, WIDTH + 1, y);  buf.push('#');
        }
        for x in 0..=WIDTH + 1 {
            write_move(&mut buf, x, 0);          buf.push('#');
            write_move(&mut buf, x, HEIGHT + 1); buf.push('#');
        }

        // Food — white
        buf.push_str("\x1b[37m");
        write_move(&mut buf, self.food.x + 1, self.food.y + 1);
        buf.push('•');

        // Snake — colour cycles with speed
        let colour = match self.speed % 3 {
            0 => "\x1b[32m",
            1 => "\x1b[36m",
            _ => "\x1b[33m",
        };
        buf.push_str(colour);

        let body = self.snake.get_body();
        for (i, pt) in body.iter().enumerate() {
            let prev = if i == 0 { None } else { body.get(i - 1) };
            let next = body.get(i + 1);
            let ch = segment_char(pt, prev, next);
            write_move(&mut buf, pt.x + 1, pt.y + 1);
            buf.push(ch);
        }

        // Score line below board
        buf.push_str("\x1b[0m");
        write_move(&mut buf, 0, HEIGHT + 2);
        buf.push_str(&format!(" Score: {}  [q] quit", self.score));

        buf
    }

    pub fn render_gameover(&self) -> String {
        let mut buf = String::new();
        buf.push_str("\x1b[2J\x1b[H");
        buf.push_str("\x1b[35m");
        buf.push_str("█▀▀ ▄▀█ █▀▄▀█ █▀▀\r\n");
        buf.push_str("█▄█ █▀█ █░▀░█ ██▄\r\n\r\n");
        buf.push_str("█▀█ █░█ █▀▀ █▀█\r\n");
        buf.push_str("█▄█ ▀▄▀ ██▄ █▀▄\r\n\r\n");
        buf.push_str("\x1b[0m");
        buf.push_str(&format!("Score: {}\r\n\r\n", self.score));
        buf.push_str("\x1b[90mPress any key to restart\x1b[0m\r\n");
        buf
    }

    fn place_food(&mut self) -> Point {
        loop {
            let p = Point::new(self.rng.range(WIDTH), self.rng.range(HEIGHT));
            if !self.snake.contains(&p) { return p; }
        }
    }
}

fn write_move(buf: &mut String, x: u16, y: u16) {
    buf.push_str(&format!("\x1b[{};{}H", y + 1, x + 1));
}

fn segment_char(pt: &Point, prev: Option<&Point>, next: Option<&Point>) -> char {
    match (prev, next) {
        (None, Some(_)) => 'O',
        (Some(p), None) => {
            if pt.y == p.y { '═' } else { '║' }
        }
        (Some(p), Some(n)) => {
            if p.x == n.x { '║' }
            else if p.y == n.y { '═' }
            else {
                let d = safe_transform(pt, Direction::Down);
                let r = safe_transform(pt, Direction::Right);
                let u = safe_transform_up(pt);
                let l = safe_transform_left(pt);
                if (*n == d && *p == r) || (*p == d && *n == r) { '╔' }
                else if (*n == d && *p == l) || (*p == d && *n == l) { '╗' }
                else if (*n == u && *p == r) || (*p == u && *n == r) { '╚' }
                else { '╝' }
            }
        }
        (None, None) => 'O',
    }
}

fn safe_transform(pt: &Point, dir: Direction) -> Point {
    pt.transform(dir, 1)
}

fn safe_transform_up(pt: &Point) -> Point {
    if pt.y == 0 { *pt } else { pt.transform(Direction::Up, 1) }
}

fn safe_transform_left(pt: &Point) -> Point {
    if pt.x == 0 { *pt } else { pt.transform(Direction::Left, 1) }
}
