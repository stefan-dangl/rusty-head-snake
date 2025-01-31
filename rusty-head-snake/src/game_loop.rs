use crate::constants::{
    BACKGROUND_COLOR, FRAMES_PER_SECOND, GAME_FIELD_HEIGHT, GAME_FIELD_WIDTH, SCALING_FACTOR,
};
use crate::snake::{Direction, Snake};
use crate::target::Target;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{ButtonArgs, ButtonEvent, EventLoop};

pub struct App {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub target: Target,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND_COLOR, gl);
        });
        self.target.render(args, &mut self.gl);
        self.snake.render(args, &mut self.gl);
    }

    fn update(&mut self, args: &UpdateArgs) {
        let tail = self.snake.position[self.snake.position.len() - 1];
        let mut met_target: bool = false;

        // check if we hit the target
        if self.snake.position.contains(&self.target.position) {
            met_target = true;
            self.target = Target::new();
        }

        if self.snake.position[1..].contains(&self.snake.position[0]) {
            println!("Game over!");
            std::process::exit(0);
        }

        // Propagate position
        for i in (1..self.snake.position.len()).rev() {
            self.snake.position[i] = self.snake.position[i - 1];
        }

        // Adjust head:
        match self.snake.direction {
            Direction::Up => {
                self.snake.position[0].y -= 1.0 * SCALING_FACTOR;
                if self.snake.position[0].y < 0.0 {
                    self.snake.position[0].y = (GAME_FIELD_HEIGHT + 1.0) * SCALING_FACTOR;
                }
            }
            Direction::Down => {
                self.snake.position[0].y = (self.snake.position[0].y + 1.0 * SCALING_FACTOR)
                    % ((GAME_FIELD_HEIGHT + 1.0) * SCALING_FACTOR);
            }
            Direction::Left => {
                self.snake.position[0].x -= 1.0 * SCALING_FACTOR;
                if self.snake.position[0].x < 0.0 {
                    self.snake.position[0].x = (GAME_FIELD_WIDTH + 1.0) * SCALING_FACTOR;
                }
            }
            Direction::Right => {
                self.snake.position[0].x = (self.snake.position[0].x + 1.0 * SCALING_FACTOR)
                    % ((GAME_FIELD_WIDTH + 1.0) * SCALING_FACTOR);
            }
        }

        if met_target {
            self.snake.position.push(tail);
        }
    }

    fn button_press(&mut self, args: &ButtonArgs) {
        if let piston::Button::Keyboard(key) = args.button {
            match key {
                piston::Key::Up | piston::Key::W => {
                    if self.snake.direction != Direction::Down {
                        self.snake.direction = Direction::Up;
                    }
                }
                piston::Key::Down | piston::Key::S => {
                    if self.snake.direction != Direction::Up {
                        self.snake.direction = Direction::Down;
                    }
                }
                piston::Key::Left | piston::Key::A => {
                    if self.snake.direction != Direction::Right {
                        self.snake.direction = Direction::Left;
                    }
                }
                piston::Key::Right | piston::Key::D => {
                    if self.snake.direction != Direction::Left {
                        self.snake.direction = Direction::Right;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn game_loop(app: &mut App, window: &mut Window) {
    let mut events = Events::new(EventSettings::new()).ups(FRAMES_PER_SECOND);
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            app.button_press(&args);
        }
    }
}
