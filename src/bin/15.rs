use aoc_2021::input::*;
use ndarray::{s, Array2};
use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let lines = lines()?;
    let lines = lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let maze = Maze::parse(&lines).expect("no maze");

    println!("{}", maze.shortest_path_tb());
    println!("{}", maze.tile(5).shortest_path_tb());

    Ok(())
}

struct Maze {
    risk_level: Array2<usize>,
}

impl Maze {
    fn parse(lines: &[&str]) -> Option<Self> {
        let height = lines.len();

        if height > 0 {
            let width = lines[0].len();
            let mut risk_level = Array2::zeros((width, height));

            for (j, line) in lines.iter().enumerate() {
                for (i, ch) in line.chars().enumerate() {
                    risk_level[(i, j)] = ch.to_digit(10)? as usize;
                }
            }

            Some(Self { risk_level })
        } else {
            None
        }
    }

    fn tile(&self, n: usize) -> Maze {
        let w = self.risk_level.shape()[0];
        let h = self.risk_level.shape()[1];
        let mut new_risk_levels = Array2::zeros((n * w, n * h));

        for i in 0..n {
            for j in 0..n {
                let offset = i + j;

                new_risk_levels
                    .slice_mut(s![(i * w)..((i + 1) * w), (j * h)..((j + 1) * h),])
                    .assign(&self.risk_level.map(|x| {
                        let nx = x + offset;

                        (nx - 1) % 9 + 1
                    }));
            }
        }

        Self {
            risk_level: new_risk_levels,
        }
    }

    fn top_left(&self) -> (usize, usize) {
        (0, 0)
    }

    fn bottom_right(&self) -> (usize, usize) {
        (
            self.risk_level.shape()[0] - 1,
            self.risk_level.shape()[1] - 1,
        )
    }

    fn shortest_path_tb(&self) -> usize {
        self.shortest_path(self.top_left(), self.bottom_right())
    }

    fn shortest_path(&self, starting_point: (usize, usize), end_point: (usize, usize)) -> usize {
        let mut so_far = Array2::from_elem(self.risk_level.raw_dim(), usize::MAX);
        let mut to_visit = VecDeque::new();
        to_visit.push_back((starting_point, starting_point));

        while let Some((from, to)) = to_visit.pop_front() {
            if so_far[to] == usize::MAX
                || self.risk_level[to].saturating_add(so_far[from]) < so_far[to]
            {
                so_far[to] = self.risk_level[to].checked_add(so_far[from]).unwrap_or(0);

                let (i, j) = to;

                for di in [-1, 0, 1] {
                    for dj in [-1, 0, 1] {
                        let ii = i as isize + di;
                        let jj = j as isize + dj;

                        if ii < 0 || jj < 0 {
                            // pass
                        } else if ii >= so_far.shape()[0] as isize
                            || jj >= so_far.shape()[1] as isize
                        {
                            // pass
                        } else if di == 0 && dj == 0 {
                            // pass
                        } else if di != 0 && dj != 0 {
                            // pass
                        } else {
                            to_visit.push_back((to, (ii as usize, jj as usize)));
                        }
                    }
                }
            }
        }

        so_far[end_point]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    const EXAMPLE: [&str; 10] = [
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    #[test]
    fn _01_parse() {
        let maze = Maze::parse(&EXAMPLE);
        assert!(maze.is_some());
    }

    #[test]
    fn _01_example() {
        let maze = Maze::parse(&EXAMPLE).expect("no maze");

        assert_eq!(maze.shortest_path_tb(), 40);
    }

    #[test]
    fn _02_tile() {
        let maze = Maze::parse(&["1"]).expect("no maze");

        assert_eq!(
            maze.tile(3).risk_level,
            arr2(&[[1, 2, 3], [2, 3, 4], [3, 4, 5]])
        );
    }

    #[test]
    fn _02_tile_wrap() {
        let maze = Maze::parse(&["9"]).expect("no maze");

        assert_eq!(
            maze.tile(3).risk_level,
            arr2(&[[9, 1, 2], [1, 2, 3], [2, 3, 4]])
        );
    }

    #[test]
    fn _02_example() {
        let maze = Maze::parse(&EXAMPLE).expect("no maze").tile(5);

        assert_eq!(maze.shortest_path_tb(), 315);
    }
}
