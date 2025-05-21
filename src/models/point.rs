use crate::traits::object::{Object};
use macroquad::prelude::*;
use crate::constants::draw::POINT_COLOR;

#[derive(Debug,Clone,Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub radius: f32
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x,
            y: y,
            radius: 5.0
        }
    }

    pub fn collide(&self, p: (f32, f32)) -> bool {
        let distance = ((p.0 - self.x).powi(2) + (p.1 - self.y).powi(2)).sqrt();
        return distance <= self.radius
    }
}

impl Object for Point {
    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, POINT_COLOR);
    }
}