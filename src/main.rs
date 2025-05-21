mod models;
mod traits;
mod constants;
// use crate::traits::object::Object;
use models::prelude::*;
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut world = World::new();
    let p = Point::new(100.0, 100.0);
    let p1 = Point::new(150.0, 150.0);
    world.add_point(p);
    world.add_point(p1);
    println!("{:?}", p);
    
    loop {
        clear_background(BLACK);
        world.update();
        world.draw();
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        // draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}