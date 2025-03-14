use crate::snake::Direction;
use euclid::Point2D;
use std::fs::{read_dir, File};
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct Level {
    pub target_points: Option<i32>,
    pub start_position: Option<Point2D<i32, i32>>,
    pub start_direction: Option<Direction>,
    pub obstacles: Vec<Point2D<i32, i32>>,
    pub updates_per_second: i32,
    pub height: i32,
    pub width: i32,
}

struct Map {
    start_position: Point2D<i32, i32>,
    direction: Direction,
    obstacles: Vec<Point2D<i32, i32>>,
    height: i32,
    width: i32,
}

impl Level {
    pub fn default() -> Self {
        Level {
            target_points: None,
            start_position: None,
            start_direction: None,
            obstacles: vec![],
            updates_per_second: 8,
            height: 10,
            width: 10,
        }
    }

    pub fn load_level(dir: &str, name: &str) -> Result<Self, LoadLevelError> {
        let path = format!("{dir}/{name}");

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let json: serde_json::Value =
            serde_json::from_str(&contents).map_err(|_| LoadLevelError::InvalidFormat)?;

        let target_points = parse_property(&json["target_points"])?;
        let updates_per_second = parse_property(&json["updates_per_second"])?;
        let map = parse_map(&json["map"])?;

        Ok(Level {
            target_points: Some(target_points),
            start_position: Some(map.start_position),
            start_direction: Some(map.direction),
            obstacles: map.obstacles,
            updates_per_second,
            height: map.height,
            width: map.width,
        })
    }
}

fn parse_property(value: &serde_json::Value) -> Result<i32, LoadLevelError> {
    i32::try_from(value.as_i64().ok_or(LoadLevelError::InvalidFormat)?)
        .map_err(|_| LoadLevelError::TooLargeValue)
}

fn get_dimensions(values: &Vec<Vec<&str>>) -> Result<(i32, i32), LoadLevelError> {
    let height = values.len();
    let width = values[0].len();

    for row in values {
        if row.len() != width {
            return Err(LoadLevelError::InvalidFormat);
        }
    }
    Ok((
        i32::try_from(width).map_err(|_| LoadLevelError::TooLargeValue)?,
        i32::try_from(height).map_err(|_| LoadLevelError::TooLargeValue)?,
    ))
}

fn extract_values(value_array: &[serde_json::Value]) -> Result<Vec<Vec<&str>>, LoadLevelError> {
    value_array
        .iter()
        .map(|row| {
            row.as_array()
                .ok_or(LoadLevelError::InvalidFormat)?
                .iter()
                .map(|val| val.as_str().ok_or(LoadLevelError::InvalidFormat))
                .collect::<Result<Vec<&str>, LoadLevelError>>()
        })
        .collect()
}

fn extract_single_occurrence_element(
    element: &[Point2D<i32, i32>],
) -> Result<Point2D<i32, i32>, LoadLevelError> {
    if element.len() != 1 {
        return Err(LoadLevelError::InvalidFormat);
    }
    Ok(element[0])
}

fn convert_direction(
    start_position: Point2D<i32, i32>,
    direction_marker: Point2D<i32, i32>,
) -> Result<Direction, LoadLevelError> {
    Ok(
        match (
            direction_marker.x - start_position.x,
            direction_marker.y - start_position.y,
        ) {
            (0, 1) => Direction::Down,
            (0, -1) => Direction::Up,
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            _ => return Err(LoadLevelError::InvalidFormat),
        },
    )
}

fn parse_map(values_raw: &serde_json::Value) -> Result<Map, LoadLevelError> {
    let value_array = values_raw.as_array().ok_or(LoadLevelError::InvalidFormat)?;
    let values = extract_values(value_array)?;
    let (width, height) = get_dimensions(&values)?;

    let mut obstacles: Vec<Point2D<i32, i32>> = vec![];
    let mut start_positions: Vec<Point2D<i32, i32>> = vec![];
    let mut directions: Vec<Point2D<i32, i32>> = vec![];
    for y in 0..height {
        for x in 0..width {
            #[allow(clippy::match_on_vec_items)]
            #[allow(clippy::cast_sign_loss)]
            match values[y as usize][x as usize] {
                "o" => obstacles.push(Point2D::new(x, y)),
                "s" => start_positions.push(Point2D::new(x, y)),
                "d" => directions.push(Point2D::new(x, y)),
                "-" => {}
                _ => return Err(LoadLevelError::InvalidFormat),
            }
        }
    }

    let start_position = extract_single_occurrence_element(&start_positions)?;
    let direction_marker = extract_single_occurrence_element(&directions)?;
    let direction = convert_direction(start_position, direction_marker)?;

    Ok(Map {
        start_position,
        direction,
        obstacles,
        height,
        width,
    })
}

pub fn search_for_levels(search_path: &str) -> Result<Vec<String>, LoadLevelError> {
    let mut levels = Vec::new();
    let paths = read_dir(search_path)?;

    for path in paths.filter_map(Result::ok) {
        let path = path.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            if let Some(file_name) = path.file_name() {
                levels.push(file_name.to_string_lossy().into_owned());
            }
        }
    }
    levels.sort();
    Ok(levels)
}

#[derive(thiserror::Error, Debug)]
pub enum LoadLevelError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("One of the level parameters exceeds its maximum value")]
    TooLargeValue,

    #[error("The format of the file describing the level is not valid")]
    InvalidFormat,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    fn create_files(path: &Path, files: &[&str]) {
        for f in files {
            let file_path = path.join(f);
            File::create(&file_path).unwrap();
        }
    }

    #[test]
    fn test_create_json_files() {
        let expected_files = ["level_1.json", "level_2.json", "level_3.json"];

        let dir = tempdir().unwrap();
        let path = dir.path();
        create_files(path, &expected_files);

        let dir_string = path.to_string_lossy().into_owned();
        let result = search_for_levels(&dir_string).unwrap();
        assert_eq!(expected_files.to_vec(), result);
    }

    #[test]
    fn test_search_for_levels_invalid_path() {
        let search_path = "levelsX";
        assert!(search_for_levels(search_path).is_err());
    }

    #[test]
    fn test_load_level() {
        let file_content: &str = r#"{
            "target_points": 10,
            "updates_per_second": 8,
            "map": [
                ["o","d","-","o"],
                ["-","s","-","-"],
                ["o","-","-","o"]
            ]
        }"#;

        let expected = Level {
            target_points: Some(10),
            start_position: Some(Point2D::new(1, 1)),
            start_direction: Some(Direction::Up),
            obstacles: vec![
                Point2D::new(0, 0),
                Point2D::new(3, 0),
                Point2D::new(0, 2),
                Point2D::new(3, 2),
            ],
            updates_per_second: 8,
            height: 3,
            width: 4,
        };

        let dir = tempdir().unwrap();
        let path = dir.path();
        let file_name = "level.json";

        let file_path = path.join(file_name);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", file_content).unwrap();

        let dir_string = path.to_string_lossy().into_owned();
        let result = Level::load_level(&dir_string, file_name).unwrap();
        assert_eq!(expected, result);
    }

    #[test_case::test_case(
        r#"{
            "updates_per_second": 8,
            "map": [
                ["o","-","-","o"],
                ["-","s","d","-"],
                ["-","-","-","-"],
                ["o","-","-","o"]
            ]
        }"#
    )]
    #[test_case::test_case(
        r#"{
            "target_points": 10,
            "map": [
                ["o","-","-","o"],
                ["-","s","d","-"],
                ["-","-","-","-"],
                ["o","-","-","o"]
            ]
        }"#
    )]
    #[test_case::test_case(
        r#"{
            "target_points": 10,
            "updates_per_second": 8,
            "map": [
                ["o","x","-","o"],
                ["-","s","d","-"],
                ["-","-","-","-"],
                ["o","-","-","o"]
            ]
        }"#
    )]
    #[test_case::test_case(
        r#"{
            "target_points": 10,
            "updates_per_second": 8,
            "map": [
                ["o","-","-","o"],
                ["-","-","d","-"],
                ["-","-","-","-"],
                ["o","-","-","o"]
            ]
        }"#
    )]
    #[test_case::test_case(
        r#"{
            "target_points": 10,
            "updates_per_second": 8,
            "map": [
                ["o","s","-","o"],
                ["-","s","d","-"],
                ["-","-","-","-"],
                ["o","-","-","o"]
            ]
        }"#
    )]
    #[test_case::test_case(
        r#"{
            "target_points": 10,
            "updates_per_second": 8,
            "map": [
                ["o","-","-","o"],
                ["-","s","-","-"],
                ["-","-","-","-"],
                ["o","-","-","o"]
            ]
        }"#
    )]
    #[test_case::test_case(
        r#"{
            "target_points": 10,
            "updates_per_second": 8,
            "map": [
                ["o","-","d","o"],
                ["-","s","d","-"],
                ["-","-","-","-"],
                ["o","-","-","o"]
            ]
        }"#
    )]
    #[test_case::test_case(
        r#"{
            "target_points": 10,
            "updates_per_second": 8,
            "map": [
                ["o","-","-","o"],
                ["-","s","-","d"],
                ["-","-","-","-"],
                ["o","-","-","o"]
            ]
        }"#
    )]
    #[test_case::test_case(
        r#"{
            "target_points": 10,
            "updates_per_second": 8,
            "map": [
                ["o","-","-","o"],
                ["-","s","d","-"],
                ["-","-","-","-"],
                ["o","-","-"]
            ]
        }"#
    )]
    fn try_to_load_invalid_level(file_content: &str) {
        let dir = tempdir().unwrap();
        let path = dir.path();
        let file_name = "level.json";

        let file_path = path.join(file_name);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", file_content).unwrap();

        let dir_string = path.to_string_lossy().into_owned();
        let err = Level::load_level(&dir_string, file_name).unwrap_err();
        assert!(matches!(err, LoadLevelError::InvalidFormat));
    }
}
