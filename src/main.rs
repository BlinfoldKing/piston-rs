extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };

struct Ball {
    position: [i32; 2],
    dir: f32
}

struct Paddle {
    position: [i32; 2]
}

struct Game {
    score: [i32; 2],
    player: Paddle,
    enemy: Paddle,
    enemy_dir: bool,
    gl: GlGraphics
}

fn main() {
    let mut opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "Pong Game",
            [500, 500])
                .opengl(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap();
    
    
    let mut game = Game {
        score: [0, 0],
        player: Paddle {
            position: [40, 0]
        },
        enemy: Paddle {
            position: [440, 0]
        },
        enemy_dir: true,
        gl: GlGraphics::new(opengl)
    };


    let mut events = Events::new(EventSettings::new()).ups(60);
    while let Some(event) = events.next(&mut window) {
        if let Some(r) = event.render_args() {
            game.render(&r); 
        }

        if let Some(_u) = event.update_args() {
            game.update();
        }

        if let Some(b) = event.button_args() {
            game.pressed(&b.button); 
        }
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

    fn moveDown(&mut self, speed: f32) {
        if self.position[1] < 400 { 
            self.position[1] += (20.0 * speed) as i32
        };
    }

    fn moveUp(&mut self, speed: f32) {
        if self.position[1] >  0 { 
            self.position[1] -= (20.0 * speed) as i32
        };
    }
}

impl Game {
    fn reset(&mut self) {
    }

    fn update(&mut self) {
        if self.enemy_dir {
            self.enemy.moveDown(0.2);
            if self.enemy.position[1] == 400 {
                self.enemy_dir = !self.enemy_dir;
            }
        } else {
            self.enemy.moveUp(0.2);
            if self.enemy.position[1] == 0 {
                self.enemy_dir = !self.enemy_dir;
            }
        }
    }

    fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        let bg = [0.01; 4];
       
        let player_graphics = graphics::rectangle::rectangle_by_corners(
            self.player.position[0] as f64,
            self.player.position[1] as f64,
            (self.player.position[0] + 20) as f64,
            (self.player.position[1] + 100) as f64
        );

        let enemy_graphics = graphics::rectangle::rectangle_by_corners(
            self.enemy.position[0] as f64,
            self.enemy.position[1] as f64,
            (self.enemy.position[0] + 20) as f64,
            (self.enemy.position[1] + 100) as f64
        );

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(bg, gl);
            graphics::rectangle([1.0; 4], player_graphics, _c.transform, gl);
            graphics::rectangle([1.0; 4], enemy_graphics, _c.transform, gl);
        });
        
    }

    fn pressed(&mut self, button: &Button) {
        if button == &Button::Keyboard(Key::W) {
            self.player.moveUp(1.0); 
        }
        if button == &Button::Keyboard(Key::S) {
            self.player.moveDown(1.0); 
        }

        println!("{:?}", self.player.position);
    }
}
