use std::cmp::Ordering;
use std::io;

use aoc_2021::input::*;
use aoc_2021::ordering_iter::*;
use aoc_2021::windowed::*;

pub fn main() -> io::Result<()> {
    let sonar_sweep = lines()?
        .into_iter()
        .map(|x| x.parse::<isize>().expect("not a number"))
        .collect::<Vec<_>>();
    let windowed_sum_sweep = Windowed::new(&sonar_sweep, 3).map(|w| w.iter().sum::<isize>());

    println!(
        "{}",
        OrderingIter::new(sonar_sweep.iter())
            .filter(|&d| d == Ordering::Greater)
            .count()
    );
    println!(
        "{}",
        OrderingIter::new(windowed_sum_sweep)
            .filter(|&d| d == Ordering::Greater)
            .count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01_part_1() {
        let sonar_sweep: [isize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let num_increases = OrderingIter::new(sonar_sweep.iter())
            .filter(|&d| d == Ordering::Greater)
            .count();

        assert_eq!(num_increases, 7);
    }

    #[test]
    fn _01_part_2() {
        let sonar_sweep: [isize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let windowed_sum = Windowed::new(&sonar_sweep, 3)
            .map(|w| w.iter().cloned().sum::<isize>())
            .collect::<Vec<_>>();
        let num_increases = OrderingIter::new(windowed_sum.iter())
            .filter(|&d| d == Ordering::Greater)
            .count();

        assert_eq!(num_increases, 5);
    }
}
