use std::fmt;
use std::io;
use std::iter;
use std::ops;
use std::str::FromStr;

use aoc_2021::input::*;

fn main() -> io::Result<()> {
    let pairs = lines()?
        .iter()
        .map(|line| line.parse::<Pair>().expect("not a pair"))
        .collect::<Vec<_>>();

    println!("{}", pairs.iter().cloned().sum::<Pair>().magnitude());
    println!(
        "{}",
        combinations(&pairs)
            .iter()
            .map(|(a, b)| (a + b).magnitude())
            .max()
            .expect("no pairs")
    );

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum Pair {
    Leaf { value: usize },
    Interior { left: Box<Pair>, right: Box<Pair> },
}

impl Pair {
    fn try_rparse(s: &str, delimiter: char) -> Option<(Self, usize)> {
        let mut end = s.len();

        while let Some(next_end) = s[..end].rfind(delimiter) {
            if let Ok(left) = s[..next_end].parse::<Pair>() {
                return Some((left, next_end + 1));
            } else {
                end = next_end;
            }
        }

        None
    }

    fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf { value: _ } => true,
            _ => false,
        }
    }

    fn value(&self) -> usize {
        assert!(self.is_leaf());

        match self {
            Self::Leaf { value } => *value,
            _ => unreachable!(),
        }
    }

    fn add_to_left_most(&self, remaining: usize) -> (Self, usize) {
        match self {
            Self::Leaf { value } => (
                Self::Leaf {
                    value: value + remaining,
                },
                0,
            ),
            Self::Interior { left, right } => {
                let (left, remaining) = left.add_to_left_most(remaining);
                let right = right.clone();

                (
                    Self::Interior {
                        left: Box::new(left),
                        right,
                    },
                    remaining,
                )
            }
        }
    }

    fn add_to_right_most(&self, remaining: usize) -> (Self, usize) {
        match self {
            Self::Leaf { value } => (
                Self::Leaf {
                    value: value + remaining,
                },
                0,
            ),
            Self::Interior { left, right } => {
                let (right, remaining) = right.add_to_right_most(remaining);
                let left = left.clone();

                (
                    Self::Interior {
                        left,
                        right: Box::new(right),
                    },
                    remaining,
                )
            }
        }
    }

    fn try_explode(&self, depth: usize) -> (Option<Self>, usize, usize) {
        match self {
            Self::Interior { left, right } if depth >= 4 && left.is_leaf() && right.is_leaf() => {
                (Some(Self::Leaf { value: 0 }), left.value(), right.value())
            }
            Self::Interior { left, right } => {
                if let (Some(left), left_remaining, right_remaining) = left.try_explode(depth + 1) {
                    let (right, right_remaining) = right.add_to_left_most(right_remaining);

                    (
                        Some(Self::Interior {
                            left: Box::new(left),
                            right: Box::new(right),
                        }),
                        left_remaining,
                        right_remaining,
                    )
                } else if let (Some(right), left_remaining, right_remaining) =
                    right.try_explode(depth + 1)
                {
                    let (left, left_remaining) = left.add_to_right_most(left_remaining);
                    (
                        Some(Self::Interior {
                            left: Box::new(left),
                            right: Box::new(right),
                        }),
                        left_remaining,
                        right_remaining,
                    )
                } else {
                    (None, 0, 0)
                }
            }
            _ => (None, 0, 0),
        }
    }

    fn explode(&self) -> Option<Self> {
        self.try_explode(0).0
    }

    fn split(&self) -> Option<Self> {
        match self {
            Self::Leaf { value } if *value >= 10 => Some(Self::Interior {
                left: Box::new(Self::Leaf {
                    value: (*value + 0) / 2,
                }),
                right: Box::new(Self::Leaf {
                    value: (*value + 1) / 2,
                }),
            }),
            Self::Leaf { value: _ } => None,
            Self::Interior { left, right } => {
                let new_left = left.split();
                let new_right = if new_left.is_none() {
                    right.split()
                } else {
                    None
                };

                if new_left.is_none() && new_right.is_none() {
                    None
                } else {
                    Some(Self::Interior {
                        left: Box::new(new_left.unwrap_or(*left.clone())),
                        right: Box::new(new_right.unwrap_or(*right.clone())),
                    })
                }
            }
        }
    }

    fn reduce(&self) -> Option<Self> {
        if let Some(exploded) = self.explode() {
            Some(exploded)
        } else if let Some(split) = self.split() {
            Some(split)
        } else {
            None
        }
    }

    fn reduced(&self) -> Self {
        let mut sum = self.clone();

        while let Some(reduced) = sum.reduce() {
            sum = reduced;
        }

        sum
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Leaf { value } => *value,
            Self::Interior { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Leaf { value } => write!(f, "{}", value),
            Self::Interior { left, right } => write!(f, "[{},{}]", left, right),
        }
    }
}

impl ops::Add<Pair> for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = Self::Interior {
            left: Box::new(self),
            right: Box::new(rhs),
        };

        sum.reduced()
    }
}

impl ops::Add<&Pair> for &Pair {
    type Output = Pair;

    fn add(self, rhs: &Pair) -> Self::Output {
        let sum = Pair::Interior {
            left: Box::new(self.clone()),
            right: Box::new(rhs.clone()),
        };

        sum.reduced()
    }
}

impl iter::Sum for Pair {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let initial_value = iter.next().expect("no elements to sum");

        iter.fold(initial_value, |acc, value| acc + value)
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<usize>() {
            Ok(Self::Leaf { value })
        } else if s.starts_with("[") {
            let (left, e) = Self::try_rparse(&s[1..], ',').ok_or(())?;
            let (right, f) = Self::try_rparse(&s[1 + e..], ']').ok_or(())?;

            if &s[e + f..] == "]" {
                Ok(Self::Interior {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

fn combinations(pairs: &[Pair]) -> Vec<(Pair, Pair)> {
    let mut all = vec![];

    for p in pairs {
        for q in pairs {
            if p != q {
                all.push((p.clone(), q.clone()));
            }
        }
    }

    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01_leaf() {
        assert_eq!(
            "[1,2]".parse::<Pair>().expect("not a pair"),
            Pair::Interior {
                left: Box::new(Pair::Leaf { value: 1 }),
                right: Box::new(Pair::Leaf { value: 2 }),
            }
        )
    }

    #[test]
    fn _01_interior() {
        assert_eq!(
            "[[1,9],[8,5]]".parse::<Pair>().expect("not a pair"),
            Pair::Interior {
                left: Box::new(Pair::Interior {
                    left: Box::new(Pair::Leaf { value: 1 }),
                    right: Box::new(Pair::Leaf { value: 9 }),
                }),
                right: Box::new(Pair::Interior {
                    left: Box::new(Pair::Leaf { value: 8 }),
                    right: Box::new(Pair::Leaf { value: 5 }),
                })
            }
        );
        assert_eq!(
            "[[1,2],3]".parse::<Pair>().expect("not a pair"),
            Pair::Interior {
                left: Box::new(Pair::Interior {
                    left: Box::new(Pair::Leaf { value: 1 }),
                    right: Box::new(Pair::Leaf { value: 2 }),
                }),
                right: Box::new(Pair::Leaf { value: 3 })
            }
        );
        assert_eq!(
            "[9,[8,7]]".parse::<Pair>().expect("not a pair"),
            Pair::Interior {
                left: Box::new(Pair::Leaf { value: 9 }),
                right: Box::new(Pair::Interior {
                    left: Box::new(Pair::Leaf { value: 8 }),
                    right: Box::new(Pair::Leaf { value: 7 }),
                })
            }
        );
    }

    #[test]
    fn _01_magnitude() {
        assert_eq!(
            "[[1,2],[[3,4],5]]"
                .parse::<Pair>()
                .expect("not a pair")
                .magnitude(),
            143
        );
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<Pair>()
                .expect("not a pair")
                .magnitude(),
            1384
        );
        assert_eq!(
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .parse::<Pair>()
                .expect("not a pair")
                .magnitude(),
            445
        );
        assert_eq!(
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
                .parse::<Pair>()
                .expect("not a pair")
                .magnitude(),
            791
        );
        assert_eq!(
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
                .parse::<Pair>()
                .expect("not a pair")
                .magnitude(),
            1137
        );
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<Pair>()
                .expect("not a pair")
                .magnitude(),
            3488
        );
    }

    #[test]
    fn _01_split() {
        assert!("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
            .parse::<Pair>()
            .expect("not a pair")
            .split()
            .is_none());
        assert!("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"
            .parse::<Pair>()
            .expect("not a pair")
            .split()
            .is_none());
        assert_eq!(
            &format!(
                "{}",
                "[[[[0,7],4],[15,[0,13]]],[1,1]]"
                    .parse::<Pair>()
                    .expect("not a pair")
                    .split()
                    .expect("nothing to split")
            ),
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
        );
        assert_eq!(
            &format!(
                "{}",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
                    .parse::<Pair>()
                    .expect("not a pair")
                    .split()
                    .expect("nothing to split")
            ),
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
        );
    }

    #[test]
    fn _01_no_explode() {
        assert!("[[[[0,7],4],[15,[0,13]]],[1,1]]"
            .parse::<Pair>()
            .expect("not a pair")
            .explode()
            .is_none());
        assert!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
            .parse::<Pair>()
            .expect("not a pair")
            .explode()
            .is_none());
    }

    #[test]
    fn _01_explode() {
        assert_eq!(
            &format!(
                "{}",
                "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
                    .parse::<Pair>()
                    .expect("not a pair")
                    .explode()
                    .expect("nothing to explode")
            ),
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"
        );
        assert_eq!(
            &format!(
                "{}",
                "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"
                    .parse::<Pair>()
                    .expect("not a pair")
                    .explode()
                    .expect("nothing to explode")
            ),
            "[[[[0,7],4],[15,[0,13]]],[1,1]]"
        );
        assert_eq!(
            &format!(
                "{}",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
                    .parse::<Pair>()
                    .expect("not a pair")
                    .explode()
                    .expect("nothing to explode")
            ),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn _01_small_example() {
        let lhs = "[[[[4,3],4],4],[7,[[8,4],9]]]"
            .parse::<Pair>()
            .expect("not a pair");
        let rhs = "[1,1]".parse::<Pair>().expect("not a pair");

        assert_eq!(
            &format!("{}", lhs + rhs),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn _01_large_example() {
        let lhs = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
            .parse::<Pair>()
            .expect("not a pair");
        let rhs = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
            .parse::<Pair>()
            .expect("not a pair");

        assert_eq!(
            &format!("{}", lhs + rhs),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );
    }

    #[test]
    fn _01_example() {
        let lines = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];
        let final_sum = lines
            .iter()
            .map(|line| line.parse::<Pair>().expect("not a pair"))
            .sum::<Pair>();

        assert_eq!(
            &format!("{}", final_sum),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
        assert_eq!(final_sum.magnitude(), 4140);
    }

    #[test]
    fn _02_example() {
        let lines = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];
        let all_pairs = lines
            .iter()
            .map(|line| line.parse::<Pair>().expect("not a pair"))
            .collect::<Vec<_>>();
        let all_combinations = combinations(&all_pairs);
        let (lhs, rhs) = all_combinations
            .iter()
            .max_by_key(|(a, b)| (a + b).magnitude())
            .expect("no pairs");

        assert_eq!(
            &format!("{}", lhs + rhs),
            "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]"
        );
        assert_eq!((lhs + rhs).magnitude(), 3993);
    }
}
