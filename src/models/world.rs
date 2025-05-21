// use crate::traits::object::{Object};
use super::point::Point;
// use super::line::Line;
use super::polyline::PolyLine;
use crate::traits::object::Object;
use macroquad::prelude::*;

pub enum State {
    DrawPoint,
    DragPoint,
    Animate
}

pub struct World {
    points: Vec<Point>,
    state: State,
    point_drag: usize
    // polyline: PolyLine,
}

impl World {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            state: State::DrawPoint,
            point_drag: 0
            // polyline: PolyLine::new(),
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);        
    }

    pub fn draw(&self) {
        for p in &self.points {
            p.draw();
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::DrawPoint => self.update_draw_point(),
            State::DragPoint => self.update_drag_point(),
            State::Animate => self.update_animate()
        }
        
    }

    fn update_draw_point(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            for (i, p) in self.points.iter().enumerate() {
                if p.collide((x,y)) {
                    self.state = State::DragPoint;
                    self.point_drag = i;
                    return;
                }
            }
            let point = Point::new(x, y);
            self.add_point(point);
        } else if 
    }

    fn update_drag_point(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            self.state = State::DrawPoint;
            return;
        }
        let mut p = self.points[self.point_drag];
        let (x, y) = mouse_position();
        p.x = x;
        p.y = y;
        self.points[self.point_drag] = p;
    }

    fn update_animate(&mut self) {

    }
}
