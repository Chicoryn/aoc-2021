use ndarray::{s, Array2};

use aoc_2021::input::*;
use std::fmt::{self, Display, Formatter};
use std::io;

fn main() -> io::Result<()> {
    let lines = lines()?;
    let lines = lines.iter().map(|line| line.as_str()).collect::<Vec<_>>();
    let (paper, instr) = parse_manual(&lines).expect("no manual");

    println!("{}", instr[0].fold(&paper).count());
    println!("{}", instr.iter().fold(paper, |acc, i| i.fold(&acc)));

    Ok(())
}

struct Paper {
    dots: Array2<bool>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for j in 0..self.dots.shape()[1] {
            for i in 0..self.dots.shape()[0] {
                if self.dots[(i, j)] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Paper {
    fn new() -> Self {
        let dots = Array2::default((0, 0));

        Self { dots }
    }

    fn count(&self) -> usize {
        self.dots.iter().filter(|b| **b).count()
    }

    fn set(&mut self, i: usize, j: usize) {
        if i >= self.dots.shape()[0] || j >= self.dots.shape()[1] {
            let max_i = self.dots.shape()[0].max(i + 1);
            let max_j = self.dots.shape()[1].max(j + 1);
            let mut new_dots = Array2::default((max_i, max_j));

            new_dots
                .slice_mut(s![..self.dots.shape()[0], ..self.dots.shape()[1]])
                .assign(&self.dots);
            self.dots = new_dots;
        }

        self.dots[(i, j)] = true;
    }

    fn fold_left(&self, offset: usize) -> Self {
        let mut main = Array2::default([offset, self.dots.shape()[1]]);
        let sub = self.dots.slice(s![(offset + 1).., ..]);
        main.assign(&self.dots.slice(s![..offset, ..]));

        for i in 0..sub.shape()[0] {
            for j in 0..sub.shape()[1] {
                let new_i = offset - 1 - i;

                main[(new_i, j)] = main[(new_i, j)] || sub[(i, j)];
            }
        }

        Self { dots: main }
    }

    fn fold_up(&self, offset: usize) -> Self {
        let mut main = Array2::default([self.dots.shape()[0], offset]);
        let sub = self.dots.slice(s![.., (offset + 1)..]);
        main.assign(&self.dots.slice(s![.., ..offset]));

        for i in 0..sub.shape()[0] {
            for j in 0..sub.shape()[1] {
                let new_j = offset - 1 - j;

                main[(i, new_j)] = main[(i, new_j)] || sub[(i, j)];
            }
        }

        Self { dots: main }
    }

    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Option<Self> {
        let mut paper = Self::new();

        for line in lines {
            let mut parts = line.split(',');
            let x = parts.next()?.parse::<usize>().ok()?;
            let y = parts.next()?.parse::<usize>().ok()?;

            paper.set(x, y);
        }

        Some(paper)
    }
}

enum Axis {
    X,
    Y,
}

struct Instruction {
    along_axis: Axis,
    offset: usize,
}

impl Instruction {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Option<Vec<Self>> {
        let mut instr = vec![];

        for line in lines {
            if line.starts_with("fold along y=") {
                let offset = line.split('=').nth(1)?.parse::<usize>().ok()?;

                instr.push(Instruction {
                    along_axis: Axis::Y,
                    offset,
                });
            } else if line.starts_with("fold along x=") {
                let offset = line.split('=').nth(1)?.parse::<usize>().ok()?;

                instr.push(Instruction {
                    along_axis: Axis::X,
                    offset,
                });
            }
        }

        Some(instr)
    }

    fn fold(&self, paper: &Paper) -> Paper {
        match self.along_axis {
            Axis::X => paper.fold_left(self.offset),
            Axis::Y => paper.fold_up(self.offset),
        }
    }
}

fn parse_manual(lines: &[&str]) -> Option<(Paper, Vec<Instruction>)> {
    let paper = Paper::parse(lines.iter().take_while(|s| !s.is_empty()).map(|s| *s))?;
    let instr = Instruction::parse(lines.iter().skip_while(|s| !s.is_empty()).map(|s| *s))?;

    Some((paper, instr))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 21] = [
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ];

    #[test]
    fn _01_parse() {
        let (paper, instr) = parse_manual(&EXAMPLE).expect("no manual");

        assert_eq!(paper.dots.shape(), &[11, 15]);
        assert_eq!(instr.len(), 2);
    }

    #[test]
    fn _01_example() {
        let (paper, instr) = parse_manual(&EXAMPLE).expect("no manual");

        assert_eq!(instr[0].fold(&paper).count(), 17);
        assert_eq!(instr.iter().fold(paper, |acc, i| i.fold(&acc)).count(), 16);
    }

    #[test]
    fn _02_example() {
        let (paper, instr) = parse_manual(&EXAMPLE).expect("no manual");

        assert_eq!(
            format!("{}", instr.iter().fold(paper, |acc, i| i.fold(&acc))),
            "#####\n#...#\n#...#\n#...#\n#####\n.....\n.....\n"
        );
    }
}
