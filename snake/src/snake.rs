use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
    speed: u16,
}

impl Snake {
    pub fn new(start: Point, length: u16, speed: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        let body: Vec<Point> = (0..length).map(|i| start.transform(opposite, i)).collect();
        Self {
            body,
            direction,
            digesting: false,
            speed,
        }
    }

    pub fn get_head(&self) -> Point {
        *self.body.first().unwrap()
    }

    pub fn get_body(&self) -> Vec<Point> {
        self.body.clone()
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_speed(&self) -> u16 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: u16) {
        self.speed = speed;
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    pub fn slither(&mut self) {
        self.body
            .insert(0, self.body.first().unwrap().transform(self.direction, 1));
        if !self.digesting {
            self.body.remove(self.body.len() - 1);
        } else {
            self.digesting = false;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn grow(&mut self) {
        self.digesting = true;
    }

    pub fn bit_self(&self) -> bool {
        let next_head_point = self.get_head().transform(self.get_direction(), 1);
        let mut next_body_points = self.get_body().clone();
        next_body_points.remove(next_body_points.len() - 1);
        next_body_points.remove(0);
        next_body_points.contains(&next_head_point)
    }

    pub fn hit_wall(&self, width: u16, height: u16) -> bool {
        let head_point = self.get_head();
        match self.get_direction() {
            Direction::Up => head_point.y == 0,
            Direction::Right => head_point.x == width - 1,
            Direction::Down => head_point.y == height - 1,
            Direction::Left => head_point.x == 0,
        }
    }
}
