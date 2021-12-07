use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

use aoc_2021::input::*;

pub fn main() -> io::Result<()> {
    let lines = lines()?;
    let crabs = lines.iter().map(|line| parse_crabs(&line.trim())).fold(
        vec![],
        |mut orig, mut new_crabs| {
            orig.append(&mut new_crabs);
            orig
        },
    );

    println!(
        "{}",
        CrabSet::new(&crabs)
            .min_cost(&SimpleCrabEvaluator::new())
            .expect("no best target position")
    );
    println!(
        "{}",
        CrabSet::new(&crabs)
            .min_cost(&ComplexCrabEvaluator::new())
            .expect("no best target position")
    );
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct Crab {
    horizontal_position: usize,
}

impl FromStr for Crab {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let horizontal_position = s.parse::<usize>()?;

        Ok(Self {
            horizontal_position,
        })
    }
}

trait CrabEvaluator {
    fn cost(&self, crab: &Crab, target_position: usize) -> usize;
}

struct SimpleCrabEvaluator;

impl SimpleCrabEvaluator {
    fn new() -> Self {
        Self {}
    }
}

impl CrabEvaluator for SimpleCrabEvaluator {
    fn cost(&self, crab: &Crab, target_position: usize) -> usize {
        if target_position > crab.horizontal_position {
            target_position - crab.horizontal_position
        } else {
            crab.horizontal_position - target_position
        }
    }
}

struct ComplexCrabEvaluator;

impl ComplexCrabEvaluator {
    fn new() -> Self {
        Self {}
    }

    fn distance(&self, crab: &Crab, target_position: usize) -> usize {
        if target_position > crab.horizontal_position {
            target_position - crab.horizontal_position
        } else {
            crab.horizontal_position - target_position
        }
    }
}

impl CrabEvaluator for ComplexCrabEvaluator {
    fn cost(&self, crab: &Crab, target_position: usize) -> usize {
        let distance = self.distance(crab, target_position);

        ((distance + 1) * distance) / 2
    }
}

struct CrabSet {
    crabs: Vec<Crab>,
}

impl CrabSet {
    fn new(crabs: &[Crab]) -> Self {
        let crabs = crabs.to_vec();

        Self { crabs }
    }

    fn min_cost<Eval: CrabEvaluator>(&self, evaluator: &Eval) -> Option<usize> {
        let min_position = self.crabs.iter().map(|c| c.horizontal_position).min()?;
        let max_position = self.crabs.iter().map(|c| c.horizontal_position).max()?;

        (min_position..=max_position)
            .map(move |target_position| self.cost(evaluator, target_position))
            .min()
    }

    fn cost<Eval: CrabEvaluator>(&self, evaluator: &Eval, target_position: usize) -> usize {
        self.crabs
            .iter()
            .map(|c| evaluator.cost(c, target_position))
            .sum::<usize>()
    }
}

fn parse_crabs(s: &str) -> Vec<Crab> {
    s.split(',')
        .map(|s| s.parse::<Crab>().expect("not a crab"))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn _01_parse() {
        let crabs = parse_crabs(EXAMPLE);

        assert_eq!(crabs.len(), 10);
        assert_eq!(
            crabs[0],
            Crab {
                horizontal_position: 16
            }
        );
        assert_eq!(
            crabs[1],
            Crab {
                horizontal_position: 1
            }
        );
        assert_eq!(
            crabs[2],
            Crab {
                horizontal_position: 2
            }
        );
        assert_eq!(
            crabs[3],
            Crab {
                horizontal_position: 0
            }
        );
        assert_eq!(
            crabs[4],
            Crab {
                horizontal_position: 4
            }
        );
        assert_eq!(
            crabs[5],
            Crab {
                horizontal_position: 2
            }
        );
        assert_eq!(
            crabs[6],
            Crab {
                horizontal_position: 7
            }
        );
        assert_eq!(
            crabs[7],
            Crab {
                horizontal_position: 1
            }
        );
        assert_eq!(
            crabs[8],
            Crab {
                horizontal_position: 2
            }
        );
        assert_eq!(
            crabs[9],
            Crab {
                horizontal_position: 14
            }
        );
    }

    #[test]
    fn _01_cost() {
        let crabs = CrabSet::new(&parse_crabs(EXAMPLE));

        assert_eq!(crabs.cost(&SimpleCrabEvaluator::new(), 1), 41);
        assert_eq!(crabs.cost(&SimpleCrabEvaluator::new(), 2), 37);
        assert_eq!(crabs.cost(&SimpleCrabEvaluator::new(), 10), 71);
    }

    #[test]
    fn _01_example() {
        let crabs = CrabSet::new(&parse_crabs(EXAMPLE));

        assert_eq!(crabs.min_cost(&SimpleCrabEvaluator::new()), Some(37));
    }

    #[test]
    fn _02_example() {
        let crabs = CrabSet::new(&parse_crabs(EXAMPLE));

        assert_eq!(crabs.cost(&ComplexCrabEvaluator::new(), 2), 206);
        assert_eq!(crabs.min_cost(&ComplexCrabEvaluator::new()), Some(168));
    }
}
