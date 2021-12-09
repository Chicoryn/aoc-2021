use std::cmp::Ordering;
use std::io;
use std::str::FromStr;

use aoc_2021::input::*;

pub fn main() -> io::Result<()> {
    let lines = lines()?;

    println!(
        "{}",
        lines
            .iter()
            .map(|line| {
                let (patterns, output_values) = parse_signals(line).expect("bad example");

                count_num_1478(&patterns, &output_values).expect("no solution")
            })
            .sum::<usize>(),
    );

    println!(
        "{}",
        lines
            .iter()
            .map(|line| {
                let (patterns, output_values) = parse_signals(line).expect("bad example");
                let digits = Mapping::solve(&patterns).unwrap().map(&output_values);

                1000 * digits[0] + 100 * digits[1] + 10 * digits[2] + digits[3]
            })
            .sum::<usize>()
    );

    Ok(())
}

struct Word {
    parts: Vec<char>,
}

impl FromStr for Word {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.chars().collect::<Vec<_>>();

        Ok(Self { parts })
    }
}

impl Word {
    fn len(&self) -> usize {
        self.parts.len()
    }
}

struct MappedWord<'a, 'b> {
    word: &'a Word,
    mapping: &'b [char],
}

impl<'a, 'b> MappedWord<'a, 'b> {
    fn new(word: &'a Word, mapping: &'b [char]) -> Self {
        Self { word, mapping }
    }

    fn eq(&self, word: &[char]) -> bool {
        self.word.len() == word.len()
            && self
                .word
                .parts
                .iter()
                .all(|&ch| word.contains(&self.mapping[(ch as usize) - 97]))
    }
}

fn parse_unique_signal_patterns(s: &str) -> Option<Vec<Word>> {
    Some(
        s.split_whitespace()
            .map_while(|s| s.parse::<Word>().ok())
            .collect::<Vec<_>>(),
    )
}

fn parse_output_values(s: &str) -> Option<Vec<Word>> {
    Some(
        s.split_whitespace()
            .map_while(|s| s.parse::<Word>().ok())
            .collect::<Vec<_>>(),
    )
}

fn parse_signals(s: &str) -> Option<(Vec<Word>, Vec<Word>)> {
    let mut parts = s.split('|');
    let unique_signal_patterns = parse_unique_signal_patterns(parts.next()?)?;
    let output_values = parse_output_values(parts.next()?)?;

    Some((unique_signal_patterns, output_values))
}

struct Mapping {
    wires: Vec<char>,
}

impl Mapping {
    const KNOWN_PATTERNS: [&'static [char]; 10] = [
        &['a', 'b', 'c', 'e', 'f', 'g'],
        &['c', 'f'],
        &['a', 'c', 'd', 'e', 'g'],
        &['a', 'c', 'd', 'f', 'g'],
        &['b', 'c', 'd', 'f'],
        &['a', 'b', 'd', 'f', 'g'],
        &['a', 'b', 'd', 'e', 'f', 'g'],
        &['a', 'c', 'f'],
        &['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        &['a', 'b', 'c', 'd', 'f', 'g'],
    ];

    fn new(wires: &[char]) -> Self {
        let wires = wires.to_vec();

        Self { wires }
    }

    fn solve(unique_signal_patterns: &[Word]) -> Option<Self> {
        let mut wires = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

        while next_permutation(&mut wires) {
            let mapping = Mapping::new(&wires);

            if mapping.is_consistent(unique_signal_patterns) {
                return Some(mapping);
            }
        }

        None
    }

    fn is_consistent(&self, unique_signal_patterns: &[Word]) -> bool {
        for signal_pattern in unique_signal_patterns {
            let mapped = MappedWord::new(signal_pattern, &self.wires);

            if Self::KNOWN_PATTERNS
                .iter()
                .all(|known_pattern| !mapped.eq(known_pattern))
            {
                return false;
            }
        }

        true
    }

    fn map(&self, output_values: &[Word]) -> Vec<usize> {
        output_values
            .iter()
            .map(|output_value| {
                let mapped = MappedWord::new(&output_value, &self.wires);

                Self::KNOWN_PATTERNS
                    .iter()
                    .enumerate()
                    .find_map(|(i, known_pattern)| {
                        if mapped.eq(known_pattern) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .expect("no match?")
            })
            .collect::<Vec<_>>()
    }
}

fn next_permutation(array: &mut [char]) -> bool {
    let last_ascending = match array.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            array.reverse();
            return false;
        }
    };

    let swap_with = array[last_ascending + 1..]
        .binary_search_by(|n| char::cmp(&array[last_ascending], n).then(Ordering::Less))
        .unwrap_err(); // cannot fail because the binary search will never succeed
    array.swap(last_ascending, last_ascending + swap_with);
    array[last_ascending + 1..].reverse();
    true
}

fn count_num_1478(unique_signal_patterns: &[Word], output_values: &[Word]) -> Option<usize> {
    let mapping = Mapping::solve(unique_signal_patterns)?;
    let mut count = 0;

    for output_value in mapping.map(output_values) {
        if output_value == 1 || output_value == 4 || output_value == 7 || output_value == 8 {
            count += 1;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE_EXAMPLE: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const EXAMPLE: [&str; 10] = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn _01_parse() {
        let (patterns, output_values) = parse_signals(EXAMPLE[0]).expect("bad example");

        assert_eq!(patterns.len(), 10);
        assert_eq!(patterns[0].parts, vec!['b', 'e']);
        assert_eq!(output_values.len(), 4);
        assert_eq!(
            output_values[0].parts,
            vec!['f', 'd', 'g', 'a', 'c', 'b', 'e']
        );
    }

    #[test]
    fn _01_example() {
        assert_eq!(
            (0..10)
                .map(|i| {
                    let (patterns, output_values) = parse_signals(EXAMPLE[i]).expect("bad example");

                    count_num_1478(&patterns, &output_values).expect("no solution")
                })
                .sum::<usize>(),
            26
        );
    }

    #[test]
    fn _02_solve() {
        let (patterns, output_values) = parse_signals(LINE_EXAMPLE).expect("bad example");
        let mapping = Mapping::solve(&patterns).unwrap();

        assert_eq!(mapping.map(&output_values), vec![5, 3, 5, 3]);
    }

    #[test]
    fn _02_example() {
        let total = EXAMPLE
            .iter()
            .map(|line| {
                let (patterns, output_values) = parse_signals(line).expect("bad example");
                let digits = Mapping::solve(&patterns).unwrap().map(&output_values);

                1000 * digits[0] + 100 * digits[1] + 10 * digits[2] + digits[3]
            })
            .sum::<usize>();

        assert_eq!(total, 61229);
    }
}
