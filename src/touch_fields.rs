use crate::constants::{
    TOUCH_BOUNDARY_ACTIVE_COLOR, TOUCH_BOUNDARY_ACTIVE_THICKNESS, TOUCH_BOUNDARY_INACTIVE_COLOR,
    TOUCH_BOUNDARY_INACTIVE_THICKNESS,
};

use macroquad::{
    color::Color,
    math::Vec2,
    shapes::draw_rectangle_lines,
    window::{screen_height, screen_width},
};

#[derive(Debug)]
pub struct TouchField {
    pub p1: Vec2,
    pub p2: Vec2,
}

impl TouchField {
    pub fn in_touch_field(&self, position: Vec2) -> bool {
        let (x_min, x_max) = (self.p1.x.min(self.p2.x), self.p1.x.max(self.p2.x));
        let (y_min, y_max) = (self.p1.y.min(self.p2.y), self.p1.y.max(self.p2.y));

        position.x < x_max && position.x > x_min && position.y < y_max && position.y > y_min
    }

    pub fn render_inactive_boundaries(&self) {
        self.render_boundaries(
            TOUCH_BOUNDARY_INACTIVE_THICKNESS,
            TOUCH_BOUNDARY_INACTIVE_COLOR,
        );
    }

    pub fn render_active_boundaries(&self) {
        self.render_boundaries(TOUCH_BOUNDARY_ACTIVE_THICKNESS, TOUCH_BOUNDARY_ACTIVE_COLOR);
    }

    fn scaled_point(point: Vec2) -> Vec2 {
        Vec2 {
            x: (point.x + 1.0) / 2.0,
            y: (point.y + 1.0) / 2.0,
        }
    }

    fn render_boundaries(&self, thickness: f32, color: Color) {
        let scaled_p1 = Self::scaled_point(self.p1);
        let scaled_p2 = Self::scaled_point(self.p2);
        let width = (scaled_p2.x - scaled_p1.x).abs();
        let height = (scaled_p2.y - scaled_p1.y).abs();

        draw_rectangle_lines(
            scaled_p1.x * screen_width(),
            scaled_p1.y * screen_height(),
            width * screen_width(),
            height * screen_height(),
            thickness,
            color,
        );
    }
}
