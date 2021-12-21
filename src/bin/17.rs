use sscanf::*;
use std::io;
use std::str::FromStr;

use aoc_2021::input::*;

fn main() -> io::Result<()> {
    for line in lines()? {
        let target_area = line.parse::<Rect>().expect("not a target area");

        println!(
            "{}",
            highest_possible_y(&target_area).expect("no solution found")
        );
        println!("{}", count_solutions(&target_area));
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Rect {
    x: (isize, isize),
    y: (isize, isize),
}

impl FromStr for Rect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((min_x, max_x, min_y, max_y)) = scanf!(
            s,
            "target area: x={}..{}, y={}..{}",
            isize,
            isize,
            isize,
            isize
        ) {
            Ok(Rect {
                x: (min_x, max_x),
                y: (min_y, max_y),
            })
        } else {
            Err(())
        }
    }
}

impl Rect {
    fn new(x: (isize, isize), y: (isize, isize)) -> Self {
        Self { x, y }
    }

    fn size(&self) -> (isize, isize) {
        ((self.x.1 - self.x.0).abs(), (self.y.1 - self.y.0).abs())
    }
}

struct Probe {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Probe {
    fn new(velocity: (isize, isize)) -> Self {
        Probe {
            position: (0, 0),
            velocity,
        }
    }

    fn is_beyond(&self, target_area: &Rect) -> bool {
        (self.position.0 > target_area.x.1 && self.velocity.0 >= 0)
            || (self.position.1 < target_area.y.0 && self.velocity.1 <= 0)
    }

    fn is_inside(&self, target_area: &Rect) -> bool {
        self.position.0 >= target_area.x.0
            && self.position.0 <= target_area.x.1
            && self.position.1 >= target_area.y.0
            && self.position.1 <= target_area.y.1
    }

    #[inline]
    fn step(&self) -> Probe {
        let new_x = self.position.0 + self.velocity.0;
        let new_y = self.position.1 + self.velocity.1;
        let new_v_x = if self.velocity.0 == 0 {
            0
        } else {
            self.velocity.0 - self.velocity.0.signum()
        };
        let new_v_y = self.velocity.1 - 1;

        Probe {
            position: (new_x, new_y),
            velocity: (new_v_x, new_v_y),
        }
    }
}

fn is_feasible(mut probe: Probe, target_area: &Rect) -> Option<isize> {
    let mut max_y = probe.position.1;

    while !probe.is_beyond(target_area) {
        if probe.is_inside(target_area) {
            return Some(max_y);
        }

        probe = probe.step();
        max_y = max_y.max(probe.position.1);
    }

    None
}

fn highest_possible_y(target_area: &Rect) -> Option<isize> {
    let mut best_cost = None;
    let min_vx = 0;
    let max_vx = target_area.x.1 + 1;
    let min_vy = target_area.y.0 - 1;
    let max_vy = target_area.y.0.abs();

    for vx in min_vx..max_vx {
        for vy in min_vy..max_vy {
            if let Some(new_y) = is_feasible(Probe::new((vx, vy)), target_area) {
                if best_cost.map(|prev_y| new_y > prev_y).unwrap_or(true) {
                    best_cost = Some(new_y);
                }
            }
        }
    }

    best_cost
}

fn count_solutions(target_area: &Rect) -> usize {
    let mut count = 0;
    let min_vx = 0;
    let max_vx = target_area.x.1 + 1;
    let min_vy = target_area.y.0 - 1;
    let max_vy = target_area.y.0.abs();

    for vx in min_vx..max_vx {
        for vy in min_vy..max_vy {
            if let Some(_) = is_feasible(Probe::new((vx, vy)), target_area) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01_is_feasible() {
        let target_area = Rect::new((20, 30), (-10, -5));

        assert!(is_feasible(Probe::new((7, 2)), &target_area).is_some());
        assert!(is_feasible(Probe::new((6, 3)), &target_area).is_some());
        assert!(is_feasible(Probe::new((9, 0)), &target_area).is_some());
        assert!(is_feasible(Probe::new((17, -4)), &target_area).is_none());
    }

    #[test]
    fn _01_target_area() {
        let target_area = "target area: x=20..30, y=-10..-5"
            .parse::<Rect>()
            .expect("not a target area");

        assert_eq!(
            target_area,
            Rect {
                x: (20, 30),
                y: (-10, -5)
            }
        )
    }

    #[test]
    fn _01_example() {
        let target_area = "target area: x=20..30, y=-10..-5"
            .parse::<Rect>()
            .expect("not a target area");

        assert_eq!(highest_possible_y(&target_area), Some(45));
    }

    #[test]
    fn _02_example() {
        let target_area = "target area: x=20..30, y=-10..-5"
            .parse::<Rect>()
            .expect("not a target area");

        assert_eq!(count_solutions(&target_area), 112);
    }
}
