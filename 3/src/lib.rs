#[derive(Debug, PartialEq)]
pub enum Move {
    Right(u32),
    Left(u32),
    Up(u32),
    Down(u32),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn parse_path(input: &str) -> Vec<Move> {
    input
        .split(",")
        .map(|step| {
            let num: u32 = step[1..]
                .parse()
                .expect(&format!("Unable to parse: {}", step[1..].to_string()));
            let direction = step.chars().nth(0).unwrap();
            match direction {
                'R' => Move::Right(num),
                'L' => Move::Left(num),
                'U' => Move::Up(num),
                'D' => Move::Down(num),
                _ => panic!("Unknown step direction: {}", direction),
            }
        })
        .collect()
}

pub fn path_points(path: &Vec<Move>) -> Vec<Point> {
    use Move::*;

    let mut points = Vec::default();

    path.iter().fold(Point { x: 0, y: 0 }, |pos, mov| {
        let new_points: Vec<Point> = match mov {
            Right(steps) => (0..*steps)
                .map(|s| Point {
                    x: pos.x + (s as i32) + 1,
                    y: pos.y,
                })
                .collect(),
            Left(steps) => (0..*steps)
                .map(|s| Point {
                    x: pos.x - (s as i32) - 1,
                    y: pos.y,
                })
                .collect(),
            Up(steps) => (0..*steps)
                .map(|s| Point {
                    x: pos.x,
                    y: pos.y + (s as i32) + 1,
                })
                .collect(),
            Down(steps) => (0..*steps)
                .map(|s| Point {
                    x: pos.x,
                    y: pos.y - (s as i32) - 1,
                })
                .collect(),
        };

        new_points.iter().for_each(|p| {
            points.push(*p);
        });

        *new_points.last().unwrap()
    });

    points
}

#[cfg(test)]
mod tests {
    use super::Move::*;
    use super::*;

    #[test]
    fn parsing() {
        let path = parse_path("R10,D30,L5,U125");
        assert_eq!(vec![Right(10), Down(30), Left(5), Up(125)], path)
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
