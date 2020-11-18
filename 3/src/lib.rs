#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    direction: Direction,
    distance: u32,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn add(&self, movement: &Move) -> Point {
        use Direction::*;

        match movement.direction {
            Right => Point {
                x: self.x + movement.distance as i32,
                y: self.y,
            },
            Left => Point {
                x: self.x - movement.distance as i32,
                y: self.y,
            },
            Up => Point {
                x: self.x,
                y: self.y + movement.distance as i32,
            },
            Down => Point {
                x: self.x,
                y: self.y - movement.distance as i32,
            },
        }
    }
}

pub fn parse_path(input: &str) -> Vec<Move> {
    use Direction::*;

    input
        .split(",")
        .map(|step| {
            let distance: u32 = step[1..]
                .parse()
                .expect(&format!("Unable to parse: {}", step[1..].to_string()));

            let direction = match step.chars().nth(0).unwrap() {
                'R' => Right,
                'L' => Left,
                'U' => Up,
                'D' => Down,
                _ => panic!("Unknown step direction: {}", step),
            };

            Move {
                direction,
                distance,
            }
        })
        .collect()
}

pub fn path_points(path: &[Move]) -> Vec<Point> {
    let mut points = Vec::default();

    let mut starting_point = Point { x: 0, y: 0 };

    for movement in path {
        points.extend((0..movement.distance).map(|s| {
            starting_point.add(&Move {
                direction: movement.direction,
                distance: s + 1,
            })
        }));

        starting_point = *points.last().unwrap();
    }

    points
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::*;

    #[test]
    fn parsing() {
        let path = parse_path("R10,D30,L5,U125");
        assert_eq!(
            vec![
                Move {
                    direction: Right,
                    distance: 10
                },
                Move {
                    direction: Down,
                    distance: 30
                },
                Move {
                    direction: Left,
                    distance: 5
                },
                Move {
                    direction: Up,
                    distance: 125
                }
            ],
            path
        )
    }

    #[test]
    fn walking() {
        let path = parse_path("R1,U1,L2,D2");
        let points = path_points(&path);
        let expected = vec![
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: -1 },
        ];
        assert_eq!(expected, points);
    }
}
