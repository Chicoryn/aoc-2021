extern crate ndarray;

use std::fmt::{self, Display, Formatter};
use std::io;

use aoc_2021::input::*;
use ndarray::Array2;

fn main() -> io::Result<()> {
    let lines = lines()?;
    let lines = lines.iter().map(|line| line.as_str()).collect::<Vec<_>>();
    let mut octopuses_1 = Octopuses::parse(&lines).ok_or(io::ErrorKind::InvalidData)?;
    let mut octopuses_2 = Octopuses::parse(&lines).ok_or(io::ErrorKind::InvalidData)?;

    println!(
        "{}",
        (0..100)
            .map(|_| octopuses_1.age().flash().0.reset().1)
            .sum::<usize>()
    );

    println!(
        "{}",
        (1..usize::MAX)
            .skip_while(|_| {
                let (_, count) = octopuses_2.age().flash();
                octopuses_2.reset();

                count != octopuses_2.len()
            })
            .next()
            .expect("no all flash?")
    );

    Ok(())
}

#[derive(Clone)]
struct Octopus {
    energy_level: usize,
    has_flashed: bool,
}

impl Default for Octopus {
    fn default() -> Self {
        Self {
            energy_level: 0,
            has_flashed: false,
        }
    }
}

impl Octopus {
    fn new(n: usize) -> Self {
        Self {
            energy_level: n,
            has_flashed: false,
        }
    }

    fn age(&mut self) {
        self.energy_level += 1;
    }

    fn flashed(&mut self) {
        self.energy_level += 1;
    }

    fn try_flash(&mut self) -> bool {
        if self.energy_level > 9 && !self.has_flashed {
            self.has_flashed = true;

            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        if self.has_flashed {
            self.energy_level = 0;
            self.has_flashed = false;
        }
    }
}

struct Octopuses {
    octopuses: Array2<Octopus>,
}

impl Octopuses {
    fn new(octopuses: Array2<Octopus>) -> Self {
        Self { octopuses }
    }

    fn parse(lines: &[&str]) -> Option<Octopuses> {
        let width = lines[0].len();
        let height = lines.len();
        let mut out = Array2::default((height, width));

        for (i, line) in lines.iter().enumerate() {
            for (j, n) in line
                .chars()
                .map(|ch| ch.to_digit(10).expect("not a number"))
                .enumerate()
            {
                out[(i, j)] = Octopus::new(n as usize);
            }
        }

        Some(Octopuses::new(out))
    }

    fn len(&self) -> usize {
        self.octopuses.len()
    }

    fn age(&mut self) -> &mut Self {
        for octopus in self.octopuses.iter_mut() {
            octopus.age();
        }

        self
    }

    fn try_flash(&mut self) -> usize {
        let mut to_flash = vec![];

        for ((i, j), octopus) in self.octopuses.indexed_iter_mut() {
            if octopus.try_flash() {
                to_flash.push((i, j));
            }
        }

        for &(i, j) in &to_flash {
            for di in &[-1, 0, 1] {
                for dj in &[-1, 0, 1] {
                    let ii = (i as isize) + di;
                    let jj = (j as isize) + dj;

                    if *di == 0 && *dj == 0 {
                        // pass
                    } else if ii < 0 || jj < 0 {
                        // pass
                    } else if ii >= self.octopuses.shape()[0] as isize {
                        // pass
                    } else if jj >= self.octopuses.shape()[1] as isize {
                        // pass
                    } else {
                        self.octopuses[(ii as usize, jj as usize)].flashed();
                    }
                }
            }
        }

        to_flash.len()
    }

    fn flash(&mut self) -> (&mut Self, usize) {
        let mut total_count = 0;

        loop {
            let count = self.try_flash();

            if count == 0 {
                break;
            }

            total_count += count;
        }

        (self, total_count)
    }

    fn reset(&mut self) -> (&mut Self, usize) {
        let mut count = 0;

        for octopus in self.octopuses.iter_mut() {
            if octopus.has_flashed {
                count += 1;
            }

            octopus.reset();
        }

        (self, count)
    }
}

impl Display for Octopuses {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.octopuses.rows() {
            for octopus in row.iter() {
                write!(f, "{}", octopus.energy_level)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 10] = [
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ];

    #[test]
    fn _01_parse() {
        let octopuses = Octopuses::parse(&EXAMPLE).expect("no octopuses");

        assert_eq!(
            format!("{}", octopuses),
            "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n"
        );
    }

    #[test]
    fn _01_flash() {
        let mut octopuses =
            Octopuses::parse(&["11111", "19991", "19191", "19991", "11111"]).expect("no octopuses");

        assert_eq!(
            format!("{}", octopuses.age().flash().0.reset().0),
            "34543\n40004\n50005\n40004\n34543\n"
        );
        assert_eq!(
            format!("{}", octopuses.age().flash().0.reset().0),
            "45654\n51115\n61116\n51115\n45654\n"
        );
    }

    #[test]
    fn _01_example_10() {
        let mut octopuses = Octopuses::parse(&EXAMPLE).expect("no octopuses");

        assert_eq!(
            (0..10)
                .map(|_| octopuses.age().flash().0.reset().1)
                .sum::<usize>(),
            204
        );
    }

    #[test]
    fn _01_example_100() {
        let mut octopuses = Octopuses::parse(&EXAMPLE).expect("no octopuses");

        assert_eq!(
            (0..100)
                .map(|_| octopuses.age().flash().0.reset().1)
                .sum::<usize>(),
            1656
        );
    }

    #[test]
    fn _02_example() {
        let mut octopuses = Octopuses::parse(&EXAMPLE).expect("no octopuses");

        assert_eq!(
            (1..usize::MAX)
                .skip_while(|_| {
                    let (_, count) = octopuses.age().flash();
                    octopuses.reset();

                    count != octopuses.len()
                })
                .next(),
            Some(195)
        );
    }
}
