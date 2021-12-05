use std::io;
use std::str::FromStr;

use aoc_2021::input::*;

pub fn main() -> io::Result<()> {
    let lines = lines()?;
    let mut straight_diagram = VentDiagram::with_capacity(1000);
    let mut full_diagram = VentDiagram::with_capacity(1000);

    for line in &lines {
        let vent_line = line.parse::<VentLine>().expect("bad vent line");

        full_diagram.push(&vent_line);
        if vent_line.is_straight() {
            straight_diagram.push(&vent_line);
        }
    }

    println!("{}", straight_diagram.num_overlapping());
    println!("{}", full_diagram.num_overlapping());

    Ok(())
}

#[derive(PartialEq, Debug)]
enum VentLineParseErr {
    MissingCoordinate,
    InvalidCoordinate,
    MissingPoint,
}

#[derive(PartialEq, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = VentLineParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.trim().split(',');
        let x = coords
            .next()
            .ok_or(VentLineParseErr::MissingCoordinate)
            .and_then(|x| {
                x.parse::<usize>()
                    .map_err(|_| VentLineParseErr::InvalidCoordinate)
            })?;
        let y = coords
            .next()
            .ok_or(VentLineParseErr::MissingCoordinate)
            .and_then(|x| {
                x.parse::<usize>()
                    .map_err(|_| VentLineParseErr::InvalidCoordinate)
            })?;

        Ok(Self { x, y })
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn max(&self, other: &Point) -> Self {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

#[derive(PartialEq, Debug)]
struct VentLine {
    source: Point,
    dest: Point,
}

impl FromStr for VentLine {
    type Err = VentLineParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split("->");
        let source = points
            .next()
            .ok_or(VentLineParseErr::MissingPoint)
            .and_then(|x| x.parse::<Point>())?;
        let dest = points
            .next()
            .ok_or(VentLineParseErr::MissingPoint)
            .and_then(|x| x.parse::<Point>())?;

        Ok(Self { source, dest })
    }
}

impl VentLine {
    #[cfg(test)]
    fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        let source = Point::new(x0, y0);
        let dest = Point::new(x1, y1);

        Self { source, dest }
    }

    fn max(&self) -> Point {
        Point::new(
            self.source.x.max(self.dest.x),
            self.source.y.max(self.dest.y),
        )
    }

    fn is_straight(&self) -> bool {
        self.dest.x == self.source.x || self.dest.y == self.source.y
    }
}

struct VentDiagram {
    size: Point,
    points: Vec<usize>,
}

impl VentDiagram {
    fn with_capacity(size: usize) -> Self {
        Self {
            size: Point::new(size, size),
            points: vec![0; (size + 1) * (size + 1)],
        }
    }

    fn ensure_size(&mut self, point: &Point) {
        if self.size.x < point.x || self.size.y < point.y {
            self.resize(&point.max(&self.size));
        }
    }

    fn resize(&mut self, new_size: &Point) {
        let mut new_points = vec![0; (new_size.x + 1) * (new_size.y + 1)];

        for x in 0..=self.size.x {
            for y in 0..=self.size.y {
                let prev_idx = Self::p2idx_by(x, y, &self.size);
                let new_idx = Self::p2idx_by(x, y, &new_size);

                new_points[new_idx] = self.points[prev_idx];
            }
        }

        self.size = new_size.clone();
        self.points = new_points;
    }

    fn p2idx_by(x: usize, y: usize, size: &Point) -> usize {
        y * (size.x + 1) + x
    }

    fn push(&mut self, line: &VentLine) {
        self.ensure_size(&line.max());

        let mut curr_point = line.source.clone();
        loop {
            let idx = Self::p2idx_by(curr_point.x, curr_point.y, &self.size);
            self.points[idx] += 1;

            if curr_point == line.dest {
                break;
            }

            if curr_point.x < line.dest.x {
                curr_point.x += 1;
            } else if curr_point.x > line.dest.x {
                curr_point.x -= 1;
            }

            if curr_point.y < line.dest.y {
                curr_point.y += 1;
            } else if curr_point.y > line.dest.y {
                curr_point.y -= 1;
            }
        }
    }

    fn num_overlapping(&self) -> usize {
        self.points
            .iter()
            .map(|&x| if x >= 2 { 1 } else { 0 })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn _01_parse() {
        assert_eq!(
            EXAMPLE[0].parse::<VentLine>(),
            Ok(VentLine::new(0, 9, 5, 9))
        );
        assert_eq!(
            EXAMPLE[1].parse::<VentLine>(),
            Ok(VentLine::new(8, 0, 0, 8))
        );
        assert_eq!(
            EXAMPLE[2].parse::<VentLine>(),
            Ok(VentLine::new(9, 4, 3, 4))
        );
        assert_eq!(
            EXAMPLE[3].parse::<VentLine>(),
            Ok(VentLine::new(2, 2, 2, 1))
        );
        assert_eq!(
            EXAMPLE[4].parse::<VentLine>(),
            Ok(VentLine::new(7, 0, 7, 4))
        );
        assert_eq!(
            EXAMPLE[5].parse::<VentLine>(),
            Ok(VentLine::new(6, 4, 2, 0))
        );
        assert_eq!(
            EXAMPLE[6].parse::<VentLine>(),
            Ok(VentLine::new(0, 9, 2, 9))
        );
        assert_eq!(
            EXAMPLE[7].parse::<VentLine>(),
            Ok(VentLine::new(3, 4, 1, 4))
        );
        assert_eq!(
            EXAMPLE[8].parse::<VentLine>(),
            Ok(VentLine::new(0, 0, 8, 8))
        );
        assert_eq!(
            EXAMPLE[9].parse::<VentLine>(),
            Ok(VentLine::new(5, 5, 8, 2))
        );
    }

    #[test]
    fn _01_overlap() {
        let mut diagram = VentDiagram::new();

        for line in &EXAMPLE {
            let vent_line = line.parse::<VentLine>().expect("bad vent line");

            if vent_line.is_straight() {
                diagram.push(&vent_line);
            }
        }

        assert_eq!(diagram.num_overlapping(), 5);
    }

    #[test]
    fn _02_overlap() {
        let mut diagram = VentDiagram::new();

        for line in &EXAMPLE {
            let vent_line = line.parse::<VentLine>().expect("bad vent line");
            diagram.push(&vent_line);
        }

        assert_eq!(diagram.num_overlapping(), 12);
    }
}
