use super::point::Point;

use crate::traits::object::{Object};
use crate::constants::draw::LINE_COLOR;
use macroquad::prelude::*;

#[derive(Debug,Clone,Copy)]
pub struct Line(pub Point, pub Point);

impl Line {
    pub fn new(start: Point, end:Point) -> Self {
        Self(start, end)
    }
    pub fn draw(p1: &Point, p2: &Point) {
        draw_line(p1.x, p1.y, p2.x, p2.y, 2.2, LINE_COLOR);
    }
}

impl Object for Line {
    fn draw(&self) {
        draw_line(self.0.x, self.0.y, self.1.x, self.1.y, 2.2, LINE_COLOR);
    }
}