use std::io;

use aoc_2021::input::*;

pub fn main() -> io::Result<()> {
    let fish = lines()?.iter().map(|line| parse_lanternfish(&line)).fold(
        vec![],
        |mut orig, mut new_fish| {
            orig.append(&mut new_fish);
            orig
        },
    );

    println!("{}", LanternFishSet::new(&fish).age(80).len());
    println!("{}", LanternFishSet::new(&fish).age(256).len());

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct LanternFish {
    internal_timer: usize,
}

impl LanternFish {
    fn new(internal_timer: usize) -> Self {
        Self { internal_timer }
    }

    fn try_age(&mut self) -> Option<LanternFish> {
        if self.internal_timer > 0 {
            self.internal_timer -= 1;

            None
        } else {
            self.internal_timer = 6;

            Some(LanternFish::new(8))
        }
    }
}

struct LanternFishSet {
    fishes: Vec<(LanternFish, usize)>,
}

impl LanternFishSet {
    fn new(fish: &Vec<LanternFish>) -> Self {
        let mut set = LanternFishSet { fishes: vec![] };
        for f in fish {
            set.add(f, 1);
        }

        set
    }

    fn age(&mut self, n: usize) -> &Self {
        for _ in 0..n {
            let mut new_fish = vec![];

            for (other_fish, count) in self.fishes.iter_mut() {
                if let Some(f) = other_fish.try_age() {
                    new_fish.push((f, *count));
                }
            }

            for (other_fish, count) in &new_fish {
                self.add(other_fish, *count);
            }
        }

        self
    }

    fn add(&mut self, fish: &LanternFish, count: usize) {
        for (f, n) in self.fishes.iter_mut() {
            if f == fish {
                *n += count;
                return;
            }
        }

        self.fishes.push((fish.clone(), count));
    }

    fn len(&self) -> usize {
        self.fishes.iter().map(|(_, n)| n).sum::<usize>()
    }
}

fn parse_lanternfish(s: &str) -> Vec<LanternFish> {
    s.split(',')
        .map(|n| LanternFish::new(n.parse::<usize>().expect("bad number")))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn _01_parse() {
        let fish = parse_lanternfish(EXAMPLE);

        assert_eq!(fish.len(), 5);
        assert_eq!(fish[0], LanternFish::new(3));
        assert_eq!(fish[1], LanternFish::new(4));
        assert_eq!(fish[2], LanternFish::new(3));
        assert_eq!(fish[3], LanternFish::new(1));
        assert_eq!(fish[4], LanternFish::new(2));
    }

    #[test]
    fn _01_age() {
        let mut fish = LanternFishSet::new(&parse_lanternfish(EXAMPLE));

        assert_eq!(fish.len(), 5);
        assert_eq!(fish.age(1).len(), 5);
        assert_eq!(fish.age(1).len(), 6);
        assert_eq!(fish.age(1).len(), 7);
        assert_eq!(fish.age(1).len(), 9);
        assert_eq!(fish.age(1).len(), 10);
        assert_eq!(fish.age(1).len(), 10);
        assert_eq!(fish.age(1).len(), 10);
        assert_eq!(fish.age(1).len(), 10);
        assert_eq!(fish.age(1).len(), 11);
        assert_eq!(fish.age(1).len(), 12);
        assert_eq!(fish.age(1).len(), 15);
        assert_eq!(fish.age(1).len(), 17);
        assert_eq!(fish.age(1).len(), 19);
        assert_eq!(fish.age(1).len(), 20);
        assert_eq!(fish.age(1).len(), 20);
        assert_eq!(fish.age(1).len(), 21);
        assert_eq!(fish.age(1).len(), 22);
        assert_eq!(fish.age(1).len(), 26);
    }

    #[test]
    fn _01_example() {
        let mut fish = LanternFishSet::new(&parse_lanternfish(EXAMPLE));

        assert_eq!(fish.age(80).len(), 5934);
    }

    #[test]
    fn _02_example() {
        let mut fish = LanternFishSet::new(&parse_lanternfish(EXAMPLE));

        assert_eq!(fish.age(256).len(), 26984457539);
    }
}
