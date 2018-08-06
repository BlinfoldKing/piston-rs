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
            position: [0, 0]
        },
        gl: GlGraphics::new(opengl)
    };


    let mut events = Events::new(EventSettings::new()).ups(60);
    while let Some(event) = events.next(&mut window) {
        if let Some(r) = event.render_args() {
            game.render(&r); 
        }

        if let Some(u) = event.update_args() {
        }

        if let Some(b) = event.button_args() {
            println!("button pressed");
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
}

impl Game {
    fn reset(&mut self) {
    }

    fn update(&mut self) {
    }

    fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        let bg = [0.1; 4];
       
        let player_graphics = graphics::rectangle::square(
            self.player.position[0] as f64,
            self.player.position[1] as f64,
            100.0
        );
        
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(bg, gl);
            graphics::rectangle([1.0; 4], player_graphics, _c.transform, gl);
        });

        
    }
}
