use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Move {
    Right(u16),
    Left(u16),
    Up(u16),
    Down(u16),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point(pub i16, pub i16);

pub fn parse_path(input: &str) -> Vec<Move> {
    input
        .split(",")
        .map(|step| {
            let num: u16 = step[1..]
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

pub fn path_points(path: &Vec<Move>) -> HashSet<Point> {
    use Move::*;

    let mut points = HashSet::new();

    path.iter().fold(Point(0, 0), |pos, mov| {
        let new_points: Vec<Point> = match mov {
            Right(steps) => (0..*steps)
                .map(|x| Point(pos.0 + (x as i16) + 1, pos.1))
                .collect(),
            Left(steps) => (0..*steps)
                .map(|x| Point(pos.0 - (x as i16) - 1, pos.1))
                .collect(),
            Up(steps) => (0..*steps)
                .map(|y| Point(pos.0, pos.1 + (y as i16) + 1))
                .collect(),
            Down(steps) => (0..*steps)
                .map(|y| Point(pos.0, pos.1 - (y as i16) - 1))
                .collect(),
        };

        new_points.iter().for_each(|p| {
            points.insert(*p);
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
        let expected: HashSet<Point> = vec![
            Point(1, 0),
            Point(1, 1),
            Point(0, 1),
            Point(-1, 1),
            Point(-1, 0),
            Point(-1, -1),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            expected,
            points,
            "difference: {:?}",
            expected.difference(&points)
        );
    }
}
