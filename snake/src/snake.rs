use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        let body: Vec<Point> = (0..length).map(|i| start.transform(opposite, i)).collect();
        Self {
            body,
            direction,
            digesting: false,
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
}
