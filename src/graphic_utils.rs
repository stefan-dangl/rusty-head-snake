use crate::constants::{ERROR_TEXT_SIZE, POINTS_TEXT_SIZE, SNAKE_HEAD_COLOR};
use euclid::Point2D;
use macroquad::{
    color::{Color, BLACK, RED},
    shapes::draw_rectangle,
    text::{draw_text_ex, get_text_center, Font, TextParams},
    window::{clear_background, next_frame, screen_height, screen_width},
};

pub async fn render_error_message(text: &str) {
    clear_background(BLACK);
    loop {
        render_text(
            text,
            Point2D::new(screen_width() / 2.0, screen_height() / 2.0),
            None,
            ERROR_TEXT_SIZE,
            RED,
        );
        next_frame().await;
    }
}

pub fn render_scaled_square(
    color: Color,
    position: Point2D<i32, i32>,
    width: f32,
    scale: (f32, f32),
) {
    draw_rectangle(
        position.x as f32 * scale.0,
        position.y as f32 * scale.1,
        width * scale.0,
        width * scale.1,
        color,
    );
}

pub fn render_x_centered_rect(y_position: f32, height: f32, color: Color) {
    draw_rectangle(
        screen_width() / 4.0,
        y_position - height / 2.0,
        screen_width() / 2.0,
        height,
        color,
    );
}

pub fn render_text(
    text: &str,
    position: Point2D<f32, f32>,
    font: Option<&Font>,
    font_size: u16,
    color: Color,
) {
    let text_center = get_text_center(text, font, font_size, 1.0, 0.0);
    let params = TextParams {
        font,
        font_size,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        rotation: 0.0,
        color,
    };
    draw_text_ex(
        text,
        position.x - text_center.x,
        position.y - text_center.y,
        params,
    );
}

pub fn render_points(point_counter: i32, point_target: Option<i32>, font: Option<&Font>) {
    let (text, position) = format_points(
        (screen_width(), screen_height()),
        point_counter,
        point_target,
    );
    render_text(&text, position, font, POINTS_TEXT_SIZE, SNAKE_HEAD_COLOR);
}

fn format_points(
    window_size: (f32, f32),
    point_counter: i32,
    point_target: Option<i32>,
) -> (String, Point2D<f32, f32>) {
    const X_RATIO_WITH_TARGET_POINTS: f32 = 0.8;
    const X_RATIO_WITHOUT_TARGET_POINTS: f32 = 0.95;
    const Y_RATIO: f32 = 0.1;

    if let Some(point_target) = point_target {
        (
            format!("{point_counter} / {point_target}"),
            Point2D::new(
                window_size.0 * X_RATIO_WITH_TARGET_POINTS,
                window_size.1 * Y_RATIO,
            ),
        )
    } else {
        (
            format!("{point_counter}"),
            Point2D::new(
                window_size.0 * X_RATIO_WITHOUT_TARGET_POINTS,
                window_size.1 * Y_RATIO,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_no_target_points() {
        let window_size = (10.0, 10.0);
        let point_counter = 5;

        let point_target = None;
        let expected_text = format!("{}", point_counter);
        let expected_position = Point2D::new(window_size.0 * 0.95, window_size.1 * 0.1);
        let res = format_points(window_size, point_counter, point_target);
        assert_eq!((expected_text, expected_position), res);

        let point_target = 10;
        let expected_text = format!("{} / {}", point_counter, point_target);
        let expected_position = Point2D::new(window_size.0 * 0.8, window_size.1 * 0.1);
        let res = format_points(window_size, point_counter, Some(point_target));
        assert_eq!((expected_text, expected_position), res);
    }
}
