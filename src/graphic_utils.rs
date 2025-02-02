use euclid::Point2D;
use graphics::{rectangle, Context, Transformed};

pub fn rectangle_corners(width: f64, scaling: (f64, f64)) -> [f64; 4] {
    rectangle::rectangle_by_corners(0.0, 0.0, width * scaling.0, width * scaling.1)
}

pub fn transform(
    context: Context,
    position: Point2D<i32, i32>,
    scaling: (f64, f64),
) -> [[f64; 3]; 2] {
    context.transform.trans(
        f64::from(position.x) * scaling.0,
        f64::from(position.y) * scaling.1,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rectangle_corners() {
        let width = 10.0;
        let scaling = (2.0, 3.0);
        let result = rectangle_corners(width, scaling);
        assert_eq!(result, [0.0, 0.0, 20.0, 30.0]);
    }

    #[test]
    fn test_transform() {
        let context = Context::new();
        let position = Point2D::new(10, 20);
        let scaling = (2.0, 3.0);
        let result = transform(context, position, scaling);
        assert_eq!(result, [[1.0, 0.0, 20.0], [0.0, 1.0, 60.0]]);
    }
}
