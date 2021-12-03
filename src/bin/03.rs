use std::collections::HashMap;
use std::hash::Hash;
use std::io;

use aoc_2021::input::*;

pub fn main() -> io::Result<()> {
    let dianostic_report = lines()?
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{}", power_consumption(&dianostic_report).unwrap());
    println!("{}", life_support_rating(&dianostic_report).unwrap());

    Ok(())
}

fn power_consumption(diagnostic_report: &[Vec<char>]) -> Option<usize> {
    let num_bits = diagnostic_report.first()?.len();
    let gamma_rate = gamma_rate(diagnostic_report, num_bits)?;
    let epsilon_rate = epsilon_rate(diagnostic_report, num_bits)?;

    Some(gamma_rate * epsilon_rate)
}

fn epsilon_rate(diagnostic_report: &[Vec<char>], num_bits: usize) -> Option<usize> {
    let epsilon_rate = (0..num_bits)
        .map(|i| least_common(diagnostic_report.iter().map(|x| x[i])).expect("no bits"))
        .collect::<String>();

    usize::from_str_radix(&epsilon_rate, 2).ok()
}

fn gamma_rate(diagnostic_report: &[Vec<char>], num_bits: usize) -> Option<usize> {
    let gamma_rate = (0..num_bits)
        .map(|i| most_common(diagnostic_report.iter().map(|x| x[i])).expect("no bits"))
        .collect::<String>();

    usize::from_str_radix(&gamma_rate, 2).ok()
}

fn life_support_rating(diagnostic_report: &Vec<Vec<char>>) -> Option<usize> {
    let oxygen_generator_rating = oxygen_generator_rating(&diagnostic_report)?;
    let co2_scrubber_rating = co2_scrubber_rating(&diagnostic_report)?;

    Some(oxygen_generator_rating * co2_scrubber_rating)
}

fn last_retained_match(
    diagnostic_report: &[Vec<char>],
    bit_criteria: impl Fn(&[char]) -> Option<char>,
) -> Option<usize> {
    let mut diagnostic_report = diagnostic_report.to_vec();
    let mut n = 0;

    while diagnostic_report.len() > 1 {
        let nth_bits = diagnostic_report.iter().map(|s| s[n]).collect::<Vec<_>>();
        let criteria = bit_criteria(&nth_bits);
        diagnostic_report.retain(|s| s.get(n).cloned() == criteria);
        n += 1;
    }

    usize::from_str_radix(
        &diagnostic_report.first()?.into_iter().collect::<String>(),
        2,
    )
    .ok()
}

fn oxygen_generator_rating(diagnostic_report: &Vec<Vec<char>>) -> Option<usize> {
    last_retained_match(diagnostic_report, |nth_bits| {
        most_common(nth_bits.iter().copied())
    })
}

fn co2_scrubber_rating(diagnostic_report: &Vec<Vec<char>>) -> Option<usize> {
    last_retained_match(diagnostic_report, |nth_bits| {
        least_common(nth_bits.iter().copied())
    })
}

fn most_common<I: Iterator<Item = char>>(elements: I) -> Option<char> {
    let occurances = count_occurances(elements);

    if occurances.get(&'0') == occurances.get(&'1') {
        Some('1')
    } else {
        occurances
            .iter()
            .max_by_key(|&(_, occ)| *occ)
            .map(|(key, _)| key.clone())
    }
}

fn least_common<I: Iterator<Item = char>>(elements: I) -> Option<char> {
    let occurances = count_occurances(elements);

    if occurances.get(&'0') == occurances.get(&'1') {
        Some('0')
    } else {
        occurances
            .iter()
            .min_by_key(|&(_, occ)| *occ)
            .map(|(key, _)| key.clone())
    }
}

fn count_occurances<T: Clone + Eq + Hash, I: Iterator<Item = T>>(elements: I) -> HashMap<T, usize> {
    elements.fold(HashMap::new(), |mut occurances, x| {
        *occurances.entry(x).or_insert(0) += 1;
        occurances
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIAGNOSTIC_REPORT: [&'static str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    fn nth_bit<'a>(diagnostic_report: &'a [&str], i: usize) -> impl Iterator<Item = char> + 'a {
        diagnostic_report
            .iter()
            .map(move |x| x.chars().nth(i).expect("too short"))
            .clone()
    }

    #[test]
    fn _01_most_common() {
        assert_eq!(most_common(nth_bit(&DIAGNOSTIC_REPORT, 0)), Some('1'));
        assert_eq!(most_common(nth_bit(&DIAGNOSTIC_REPORT, 1)), Some('0'));
        assert_eq!(most_common(nth_bit(&DIAGNOSTIC_REPORT, 2)), Some('1'));
        assert_eq!(most_common(nth_bit(&DIAGNOSTIC_REPORT, 3)), Some('1'));
        assert_eq!(most_common(nth_bit(&DIAGNOSTIC_REPORT, 4)), Some('0'));
    }

    #[test]
    fn _01_least_common() {
        assert_eq!(least_common(nth_bit(&DIAGNOSTIC_REPORT, 0)), Some('0'));
        assert_eq!(least_common(nth_bit(&DIAGNOSTIC_REPORT, 1)), Some('1'));
        assert_eq!(least_common(nth_bit(&DIAGNOSTIC_REPORT, 2)), Some('0'));
        assert_eq!(least_common(nth_bit(&DIAGNOSTIC_REPORT, 3)), Some('0'));
        assert_eq!(least_common(nth_bit(&DIAGNOSTIC_REPORT, 4)), Some('1'));
    }

    #[test]
    fn _01_power_consumption() {
        let diagnostic_report = DIAGNOSTIC_REPORT
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(power_consumption(&diagnostic_report), Some(198));
    }

    #[test]
    fn _02_oxygen_generator_rating() {
        let diagnostic_report = DIAGNOSTIC_REPORT
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(oxygen_generator_rating(&diagnostic_report), Some(23));
    }

    #[test]
    fn _02_co2_scrubber_rating() {
        let diagnostic_report = DIAGNOSTIC_REPORT
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(co2_scrubber_rating(&diagnostic_report), Some(10));
    }

    #[test]
    fn _02_life_support_rating() {
        let diagnostic_report = DIAGNOSTIC_REPORT
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(life_support_rating(&diagnostic_report), Some(230));
    }
}
