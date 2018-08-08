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
    dir: [f64; 2],
    gl: GlGraphics
}

struct Paddle {
    position: [i32; 2],
    gl: GlGraphics
}

struct Game {
    score: [i32; 2],
    player: Paddle,
    enemy: Paddle,
    enemy_dir: bool,
    ball: Ball,
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
            position: [40, 220],
            gl: GlGraphics::new(opengl)
        },
        enemy: Paddle {
            position: [440, 0],
            gl: GlGraphics::new(opengl)
        },
        enemy_dir: true,
        ball: Ball {
            position: [250; 2],
            dir: [-1.0, 0.0],
            gl: GlGraphics::new(opengl)
        },
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
    fn render(&mut self, arg: &RenderArgs) {
        let ball_graphics = graphics::rectangle::square(
            self.position[0] as f64,
            self.position[1] as f64,
            20.0
        );

        self.gl.draw(arg.viewport(), |_c, gl| {
           graphics::rectangle([1.0; 4], ball_graphics, _c.transform, gl);
        });
    }

    fn update(&mut self) {
        self.position[0] += (self.dir[0] * 5.0) as i32;
        self.position[1] += (self.dir[1] * 5.0) as i32;
    } 
}

impl Paddle {
    fn render(&mut self, arg: &RenderArgs) {
        let paddle_graphics = graphics::rectangle::rectangle_by_corners(
            self.position[0] as f64,
            self.position[1] as f64,
            (self.position[0] + 20) as f64,
            (self.position[1] + 100) as f64
        );
        
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::rectangle([1.0; 4], paddle_graphics, _c.transform, gl);
        });
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
        
        if (self.ball.position[0] + 20 >= self.enemy.position[0])
            && (self.ball.position[0] + 20 <= self.enemy.position[0] + 20)
            && (self.ball.position[1] >= self.enemy.position[1])
            && (self.ball.position[1] + 20 <= self.enemy.position[1] + 100) {
                let y =  (((self.ball.position[1] + 10) - (self.player.position[1] + 50)) as f64 / 50.0);
                self.ball.dir[0] = -1.0;
                self.ball.dir[1] = y;
        }
        
        if (self.ball.position[0] <= self.player.position[0] + 20)
            && (self.ball.position[0] >= self.player.position[0])
            && (self.ball.position[1] >= self.player.position[1])
            && (self.ball.position[1] + 20 <= self.player.position[1] + 100) {
                let y =  (((self.ball.position[1] + 10) - (self.player.position[1] + 50)) as f64 / 50.0);
                self.ball.dir[0] = 1.0;
                self.ball.dir[1] = y; 
        }

        if (self.ball.position[1] <= 0) {
            self.ball.dir[1] *= -1.0;
        }

        if (self.ball.position[1] >= 400) {
            self.ball.dir[1] *= -1.0;
        }

        self.ball.update();
    }

    fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        let bg = [0.01; 4];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(bg, gl);
        });

        self.player.render(&arg);
        self.enemy.render(&arg);
        self.ball.render(&arg); 
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
