use aoc_2021::input::*;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let lines = lines()?;
    let lines = lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let (polymer, rules) = parse_polymerization(&lines).expect("bad example");

    println!("{}", polymer_strenght(&polymer, &rules, 10));
    println!("{}", polymer_strenght(&polymer, &rules, 40));

    Ok(())
}

fn polymer_strenght(polymer: &Polymer, rules: &[PairInsertionRule], steps: usize) -> usize {
    let polymer = (0..steps).fold(polymer.clone(), |polymer, _| polymerize(&polymer, &rules));
    let occurences = polymer.count();
    let most_common = occurences.values().max().expect("no most common");
    let least_common = occurences.values().min().expect("no least common");

    most_common - least_common
}

#[derive(Clone)]
struct Polymer {
    pairs: HashMap<[char; 2], usize>,
    end_token: char,
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }

        Ok(Self::new(&s.chars().collect::<Vec<_>>(), 1))
    }
}

impl Polymer {
    fn new(s: &[char], count: usize) -> Self {
        let mut pairs = HashMap::new();

        if s.is_empty() {
            Self {
                pairs,
                end_token: '\0',
            }
        } else {
            let end_token = s[s.len() - 1];

            for offset in 0..(s.len() - 1) {
                let pair = [s[offset + 0], s[offset + 1]];

                *pairs.entry(pair).or_insert(0) += count;
            }

            Self { pairs, end_token }
        }
    }

    fn merge(polymers: &[Polymer], end_token: char) -> Polymer {
        let mut pairs = HashMap::new();

        for polymer in polymers {
            for (&pair, count) in polymer.iter() {
                *pairs.entry(pair).or_insert(0) += count;
            }
        }

        Self { pairs, end_token }
    }

    fn count(&self) -> HashMap<char, usize> {
        let mut occurences = self
            .pairs
            .iter()
            .fold(HashMap::new(), |mut acc, (ch, &count)| {
                *acc.entry(ch[0]).or_insert(0) += count;
                acc
            });

        *occurences.entry(self.end_token).or_insert(0) += 1;
        occurences
    }

    fn iter(&self) -> impl Iterator<Item = (&[char; 2], &usize)> {
        self.pairs.iter()
    }
}

struct PairInsertionRule {
    pattern: [char; 2],
    to_insert: char,
}

impl FromStr for PairInsertionRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("->");
        let pattern = parts.next().ok_or(())?.trim();
        let to_insert = parts.next().ok_or(())?.trim();

        Ok(Self {
            pattern: [
                pattern.chars().nth(0).ok_or(())?,
                pattern.chars().nth(1).ok_or(())?,
            ],
            to_insert: to_insert.chars().next().ok_or(())?,
        })
    }
}

impl PairInsertionRule {
    fn matches(&self, s: &[char]) -> bool {
        s.len() >= 2 && s[0] == self.pattern[0] && s[1] == self.pattern[1]
    }
}

fn parse_polymerization(lines: &[&str]) -> Option<(Polymer, Vec<PairInsertionRule>)> {
    let polymer = lines[0].parse::<Polymer>().ok()?;
    let rules = lines[2..]
        .iter()
        .map(|line| line.parse::<PairInsertionRule>().expect("bad rule"))
        .collect::<Vec<_>>();

    Some((polymer, rules))
}

fn polymerize(polymer: &Polymer, rules: &[PairInsertionRule]) -> Polymer {
    Polymer::merge(
        &polymer
            .iter()
            .map(|(pair, &count)| {
                let mut new_polymer = vec![pair[0]];

                for rule in rules {
                    if rule.matches(pair) {
                        new_polymer.push(rule.to_insert);
                    }
                }

                new_polymer.push(pair[1]);
                Polymer::new(&new_polymer, count)
            })
            .collect::<Vec<_>>(),
        polymer.end_token,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 18] = [
        "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C",
        "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N",
        "CN -> C",
    ];

    #[test]
    fn _01_occurences() {
        let (polymer, _rules) = parse_polymerization(&EXAMPLE).expect("bad example");
        let occurences = polymer.count();

        assert_eq!(occurences[&'N'], 2);
        assert_eq!(occurences[&'C'], 1);
        assert_eq!(occurences[&'B'], 1);
    }

    #[test]
    fn _01_example() {
        let (polymer, rules) = parse_polymerization(&EXAMPLE).expect("bad example");
        let polymer = (0..10).fold(polymer, |polymer, _| polymerize(&polymer, &rules));
        let occurences = polymer.count();

        assert_eq!(occurences[&'B'], 1749);
        assert_eq!(occurences[&'C'], 298);
        assert_eq!(occurences[&'H'], 161);
        assert_eq!(occurences[&'N'], 865);

        let most_common = occurences.values().max().expect("no most common");
        let least_common = occurences.values().min().expect("no least common");

        assert_eq!(most_common - least_common, 1588);
        assert_eq!(polymer_strenght(&polymer, &rules, 0), 1588);
    }

    #[test]
    fn _02_example() {
        let (polymer, rules) = parse_polymerization(&EXAMPLE).expect("bad example");
        assert_eq!(polymer_strenght(&polymer, &rules, 40), 2188189693529);
    }
}
