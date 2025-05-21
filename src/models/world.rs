// use crate::traits::object::{Object};
use super::point::Point;
use super::line::Line;
use super::polyline::PolyLine;
use crate::traits::object::Object;

pub struct World {
    points: Vec<Point>,
    polyline: PolyLine
}

impl World {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            polyline: PolyLine::new(),
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);        
    }

    fn draw_points_line(&self) {
        for i in 0..(self.points.len()-1) {
            let p0 = &self.points[i];
            let p1 = &self.points[i + 1];
            Line::draw(p0, p1);
        }
    }

    pub fn draw(&self) {
        for p in &self.points {
            p.draw()
        }
        self.draw_points_line();
    }

}
