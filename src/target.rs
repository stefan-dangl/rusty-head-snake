use crate::{
    constants::{TARGET_COLOR, TARGET_WIDTH},
    graphic_utils::{rectangle_corners, transform},
};
use euclid::Point2D;
use graphics::rectangle;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

#[derive(Debug, PartialEq)]
pub struct Target {
    pub position: Point2D<i32, i32>,
}

impl Target {
    pub fn new(obstacles: &[Point2D<i32, i32>], width: i32, height: i32) -> Self {
        loop {
            #[allow(clippy::cast_possible_truncation)]
            let position = Point2D::new(
                (rand::random::<f64>() * f64::from(width)).floor() as i32,
                (rand::random::<f64>() * f64::from(height)).floor() as i32,
            );
            if !obstacles.contains(&position) {
                return Target { position };
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, scaling: (f64, f64)) {
        let shape = rectangle_corners(TARGET_WIDTH, scaling);

        gl.draw(args.viewport(), |c, gl| {
            let transform = transform(c, self.position, scaling);
            rectangle(TARGET_COLOR, shape, transform, gl);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_new_target(
        obstacles: &[Point2D<i32, i32>],
        width: i32,
        height: i32,
        expected_position: Point2D<i32, i32>,
    ) {
        assert_eq!(
            Target {
                position: expected_position
            },
            Target::new(obstacles, width, height)
        );
    }

    #[test]
    fn new_target() {
        let width = 3;
        let height = 3;

        let field: Vec<_> = (0..height)
            .flat_map(|j| (0..width).map(move |i| Point2D::<i32, i32>::new(i, j)))
            .collect();

        for i in 0..field.len() {
            let target_position = field[i];
            let obstacles = [&field[..i], &field[i + 1..]].concat();
            assert_new_target(&obstacles, width, height, target_position);
        }
    }
}
