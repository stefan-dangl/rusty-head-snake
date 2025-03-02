use crate::constants::{FONT_PATH, POINTS_TEXT_SIZE, SNAKE_HEAD_COLOR};
use euclid::Point2D;
use graphics::character::CharacterCache;
use graphics::{text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::RenderArgs;

pub struct TextHandler<'a> {
    font: GlyphCache<'a>,
}

impl TextHandler<'_> {
    pub fn new() -> Self {
        let font = GlyphCache::new(FONT_PATH, (), TextureSettings::new())
            .expect("Not able to load text font");
        TextHandler { font }
    }

    pub fn render(
        &mut self,
        args: &RenderArgs,
        gl: &mut GlGraphics,
        text: &str,
        position: Point2D<f64, f64>,
        scaling: f64,
        color: [f32; 4],
    ) {
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let text_width = self.font.width(scaling as u32, text).unwrap_or(0.0) * (scaling * 42.0);
        let text_height = scaling * 28.0;

        let centered_position = Point2D::<f64, euclid::UnknownUnit>::new(
            position.x - text_width / 2.0,
            position.y + text_height / 2.0,
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(centered_position.x, centered_position.y)
                .scale(scaling, scaling);
            text::Text::new_color(color, 32)
                .draw(text, &mut self.font, &c.draw_state, transform, gl)
                .expect("Failed to draw text");
        });
    }

    fn format_points(
        draw_size: &[u32],
        point_counter: i32,
        point_target: Option<i32>,
    ) -> (String, Point2D<f64, f64>) {
        const X_RATIO_WITH_TARGET_POINTS: f64 = 0.8;
        const X_RATIO_WITHOUT_TARGET_POINTS: f64 = 0.95;
        const Y_RATIO: f64 = 0.1;

        if let Some(point_target) = point_target {
            (
                format!("{point_counter} / {point_target}"),
                Point2D::new(
                    f64::from(draw_size[0]) * X_RATIO_WITH_TARGET_POINTS,
                    f64::from(draw_size[1]) * Y_RATIO,
                ),
            )
        } else {
            (
                format!("{point_counter}"),
                Point2D::new(
                    f64::from(draw_size[0]) * X_RATIO_WITHOUT_TARGET_POINTS,
                    f64::from(draw_size[1]) * Y_RATIO,
                ),
            )
        }
    }

    pub fn render_points(
        &mut self,
        args: &RenderArgs,
        gl: &mut GlGraphics,
        point_counter: i32,
        point_target: Option<i32>,
    ) {
        let (formatted_points, position) =
            TextHandler::format_points(&args.draw_size, point_counter, point_target);
        self.render(
            args,
            gl,
            &formatted_points,
            position,
            POINTS_TEXT_SIZE,
            SNAKE_HEAD_COLOR,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_points() {
        let draw_size = [10, 10];
        let point_counter = 5;

        let point_target = None;
        let expected_text = format!("{}", point_counter);
        let expected_position = Point2D::new(
            f64::from(draw_size[0]) * 0.95,
            f64::from(draw_size[1]) * 0.1,
        );
        let res = TextHandler::format_points(&draw_size, point_counter, point_target);
        assert_eq!((expected_text, expected_position), res);

        let point_target = Some(10);
        let expected_text = format!("{} / {}", point_counter, point_target.unwrap());
        let expected_position =
            Point2D::new(f64::from(draw_size[0]) * 0.8, f64::from(draw_size[1]) * 0.1);
        let res = TextHandler::format_points(&draw_size, point_counter, point_target);
        assert_eq!((expected_text, expected_position), res);
    }
}
