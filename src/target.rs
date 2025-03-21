use crate::{
    constants::{TARGET_COLOR, TARGET_WIDTH},
    graphic_utils::render_scaled_square,
};
use euclid::Point2D;
use macroquad::rand::gen_range;

#[derive(Debug, PartialEq)]
pub struct Target {
    pub position: Point2D<i32, i32>,
}

impl Target {
    pub fn new(obstacles: &[Point2D<i32, i32>], width: i32, height: i32) -> Self {
        loop {
            let position = Point2D::new(gen_range(0, width), gen_range(0, height));
            if !obstacles.contains(&position) {
                return Target { position };
            }
        }
    }

    pub fn render(&mut self, scaling: (f32, f32)) {
        render_scaled_square(TARGET_COLOR, self.position, TARGET_WIDTH, scaling);
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
