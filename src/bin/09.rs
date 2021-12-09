use std::io;

use aoc_2021::input::*;

pub fn main() -> io::Result<()> {
    let lines = lines()?;
    let lines_ = lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let height_map = HeightMap::parse(&lines_).expect("missing height_map");
    let mut basins = height_map.basin_sizes();
    basins.sort();

    println!(
        "{}",
        height_map
            .coords()
            .iter()
            .filter(|(x, y)| height_map.is_low_point(*x, *y))
            .filter_map(|(x, y)| height_map.at(*x, *y).map(|h| h + 1))
            .sum::<usize>()
    );
    println!(
        "{}",
        basins
            .iter()
            .rev()
            .take(3)
            .fold(1, |acc, basin_size| acc * basin_size)
    );

    Ok(())
}

struct HeightMap {
    floor: Vec<usize>, // row-major
    width: usize,
    height: usize,
}

impl HeightMap {
    fn parse(lines: &[&str]) -> Option<Self> {
        let height = lines.len();

        if height > 0 {
            let width = lines[0].len();
            let mut floor = vec![];

            for line in lines {
                for height in line.chars() {
                    floor.push(height.to_digit(10)? as usize);
                }
            }

            Some(Self {
                floor,
                width,
                height,
            })
        } else {
            None
        }
    }

    fn coords(&self) -> Vec<(isize, isize)> {
        let mut coords = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                coords.push((x as isize, y as isize))
            }
        }

        coords
    }

    fn basin_size(&self, x: isize, y: isize) -> usize {
        let mut visited = vec![];
        let mut to_visit = vec![(x, y)];

        while let Some((x, y)) = to_visit.pop() {
            if visited.contains(&(x, y)) {
                // pass
            } else if self.at(x, y).map(|h| h < 9).unwrap_or(false) {
                visited.push((x, y));

                if !visited.contains(&(x - 1, y)) {
                    to_visit.push((x - 1, y));
                }
                if !visited.contains(&(x + 1, y)) {
                    to_visit.push((x + 1, y));
                }
                if !visited.contains(&(x, y - 1)) {
                    to_visit.push((x, y - 1));
                }
                if !visited.contains(&(x, y + 1)) {
                    to_visit.push((x, y + 1));
                }
            }
        }

        visited.len()
    }

    fn basin_sizes(&self) -> Vec<usize> {
        self.coords()
            .iter()
            .filter(|(x, y)| self.is_low_point(*x, *y))
            .map(|(x, y)| self.basin_size(*x, *y))
            .collect::<Vec<_>>()
    }

    fn is_low_point(&self, x: isize, y: isize) -> bool {
        self.at(x, y)
            .map(|h| {
                self.at(x - 1, y).map(|v| v > h).unwrap_or(true)
                    && self.at(x + 1, y).map(|v| v > h).unwrap_or(true)
                    && self.at(x, y - 1).map(|v| v > h).unwrap_or(true)
                    && self.at(x, y + 1).map(|v| v > h).unwrap_or(true)
            })
            .unwrap_or(false)
    }

    fn at(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            None
        } else {
            let index = (y as usize) * self.width + (x as usize);

            Some(self.floor[index])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 5] = [
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];

    #[test]
    fn _01_parse() {
        let height_map = HeightMap::parse(&EXAMPLE).expect("missing height_map");

        assert_eq!(height_map.width, 10);
        assert_eq!(height_map.height, 5);
        assert_eq!(
            height_map.floor,
            vec![
                2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8,
                9, 2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
            ]
        );
    }

    #[test]
    fn _01_example() {
        let height_map = HeightMap::parse(&EXAMPLE).expect("missing height_map");

        assert_eq!(
            height_map
                .coords()
                .iter()
                .filter(|(x, y)| height_map.is_low_point(*x, *y))
                .filter_map(|(x, y)| height_map.at(*x, *y).map(|h| h + 1))
                .sum::<usize>(),
            15
        );
    }

    #[test]
    fn _02_basin_size() {
        let height_map = HeightMap::parse(&EXAMPLE).expect("missing height_map");

        assert_eq!(height_map.basin_size(1, 0), 3);
        assert_eq!(height_map.basin_size(9, 0), 9);
        assert_eq!(height_map.basin_size(2, 2), 14);
        assert_eq!(height_map.basin_size(7, 4), 9);
    }

    #[test]
    fn _02_example() {
        let height_map = HeightMap::parse(&EXAMPLE).expect("missing height_map");
        let mut basins = height_map.basin_sizes();
        basins.sort();

        assert_eq!(
            basins
                .iter()
                .rev()
                .take(3)
                .fold(1, |acc, basin_size| acc * basin_size),
            1134
        );
    }
}
