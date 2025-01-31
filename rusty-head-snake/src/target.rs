use crate::constants::{self, GAME_FIELD_HEIGHT, GAME_FIELD_WIDTH};

use constants::{SCALING_FACTOR, TARGET_COLOR, TARGET_WIDTH};
use euclid::Point2D;
use graphics::{rectangle, Transformed};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub struct Target {
    pub position: Point2D<f64, f64>,
}

impl Target {
    pub fn new() -> Self {
        Target {
            position: Point2D::new(
                (rand::random::<f64>() * GAME_FIELD_WIDTH).floor() * SCALING_FACTOR,
                (rand::random::<f64>() * GAME_FIELD_HEIGHT).floor() * SCALING_FACTOR,
            ),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, TARGET_WIDTH * SCALING_FACTOR);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(self.position.x, self.position.y);
            rectangle(TARGET_COLOR, square, transform, gl);
        });
    }
}
