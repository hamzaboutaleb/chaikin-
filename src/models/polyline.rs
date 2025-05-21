use super::point::Point;
use crate::models::line::Line;
use crate::traits::object::{Object};

#[derive(Debug,Clone)]
pub struct PolyLine {
    points: Vec<Point>,
}

impl PolyLine {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
}

impl Object for PolyLine {
    fn draw(&self) {
        if self.points.len() <= 1 {
            return;
        }

        for i in 0..(self.points.len() - 1) {
            let p0 = self.points[i];
            let p1 = self.points[i + 1];
            Line::draw(&p0, &p1);
        }
    }
}