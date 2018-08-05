extern crate piston_window;

use piston_window::*;

struct Ball {
    position: [i32; 2],
    dir: f32
}

struct Paddle {
    position: [i32; 2]
}

struct Game {
    score: [i32; 2],
    player: Paddle
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "Pong Game",
            [500, 500])
                .exit_on_esc(true)
                .build()
                .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, gl| {
            clear([1.0; 4], gl);
            rectangle([1.0, 0.0, 0.0, 1.0],
                      [0.0, 0.0, 100.0, 100.0],
                      ctx.transform,
                      gl);
        });
    }

    println!("Hello, world!");
}

impl Ball {
    fn render(&mut self) {
    }
}

impl Paddle {
    fn render(&mut self) {
    }
}

impl Game {
    fn reset(&mut self) {
    }

    fn update(&mut self) {
    }

    fn render(&mut self) {
    }
}
