use ndarray::Array2;
use std::io;

use aoc_2021::input::*;

fn main() -> io::Result<()> {
    let lines = lines()?;
    let lines = lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let caves = CaveSystem::parse(&lines).expect("no caves");

    println!("{}", traverse(&caves, SimpleLimiter::new()).len());
    println!("{}", traverse(&caves, ComplexLimiter::new()).len());

    Ok(())
}

trait Limiter {
    fn is_valid(&self, path: &Path, candidate: usize) -> bool;
}

struct SimpleLimiter;

impl SimpleLimiter {
    fn new() -> Self {
        Self {}
    }
}

impl Limiter for SimpleLimiter {
    fn is_valid(&self, path: &Path, candidate: usize) -> bool {
        let limit = if path.caves.is_big(candidate) {
            usize::MAX
        } else {
            1
        };

        path.occurences[candidate] < limit
    }
}

struct ComplexLimiter;

impl ComplexLimiter {
    fn new() -> Self {
        Self {}
    }
}

impl Limiter for ComplexLimiter {
    fn is_valid(&self, path: &Path, candidate: usize) -> bool {
        let limit = if path.caves.is_big(candidate) {
            usize::MAX
        } else if path.caves.is_special(candidate) {
            1
        } else if path.all_lowercase_unique() {
            2
        } else {
            1
        };

        path.occurences[candidate] < limit
    }
}

#[derive(Debug, PartialEq)]
struct Cave {
    name: String,
    is_special: bool,
    is_big: bool,
}

impl Cave {
    fn parse(name: &str) -> Option<Self> {
        Some(Self {
            name: name.to_string(),
            is_special: name == "start" || name == "end",
            is_big: name.chars().all(char::is_uppercase),
        })
    }
}

#[derive(Debug)]
struct CaveSystem {
    caves: Vec<Cave>,
    connected: Array2<bool>,
}

impl CaveSystem {
    fn is_big(&self, index: usize) -> bool {
        self.caves[index].is_big
    }

    fn is_special(&self, index: usize) -> bool {
        self.caves[index].is_special
    }

    fn len(&self) -> usize {
        self.caves.len()
    }

    fn index_of(&self, cave: &str) -> Option<usize> {
        self.caves
            .iter()
            .enumerate()
            .filter(|(_, c)| c.name == cave)
            .map(|(i, _)| i)
            .next()
    }

    fn parse(lines: &[&str]) -> Option<Self> {
        let mut cave_system = Self {
            caves: Vec::with_capacity(lines.len()),
            connected: Array2::default([lines.len(), lines.len()]),
        };

        for line in lines {
            let mut parts = line.split('-');
            let src = parts.next()?;
            let src_idx = if let Some(idx) = cave_system.index_of(&src) {
                idx
            } else {
                cave_system.caves.push(Cave::parse(src)?);
                cave_system.caves.len() - 1
            };
            let dst = parts.next()?;
            let dst_idx = if let Some(idx) = cave_system.index_of(&dst) {
                idx
            } else {
                cave_system.caves.push(Cave::parse(dst)?);
                cave_system.caves.len() - 1
            };

            cave_system.connected[(src_idx, dst_idx)] = true;
            cave_system.connected[(dst_idx, src_idx)] = true;
        }

        Some(cave_system)
    }
}

#[derive(Clone, Debug)]
struct Path<'a> {
    caves: &'a CaveSystem,
    visited: Vec<usize>,
    occurences: Vec<usize>,
}

impl<'a> Path<'a> {
    fn new(caves: &'a CaveSystem, index: usize) -> Self {
        let mut occurences = vec![0; caves.len()];
        occurences[index] += 1;

        Self {
            caves,
            visited: vec![index],
            occurences,
        }
    }

    fn all_lowercase_unique(&self) -> bool {
        for &i in &self.visited {
            if !self.caves.is_big(i) {
                if self.occurences[i] > 1 {
                    return false;
                }
            }
        }

        true
    }

    fn last(&self) -> usize {
        self.visited[self.visited.len() - 1]
    }

    fn with_cave(&self, index: usize) -> Self {
        let mut new_path = self.clone();
        new_path.visited.push(index);
        new_path.occurences[index] += 1;
        new_path
    }
}

fn traverse(cave_system: &CaveSystem, limit: impl Limiter) -> Vec<Path> {
    let starting_point = cave_system.index_of("start").expect("no starting point");
    let end_point = cave_system.index_of("end").expect("no end point");

    let mut to_probe = vec![Path::new(cave_system, starting_point)];
    let mut paths = vec![];

    while let Some(next_path) = to_probe.pop() {
        let i = next_path.last();

        if i == end_point {
            paths.push(next_path);
        } else {
            for (j, _) in cave_system.caves.iter().enumerate() {
                if cave_system.connected[(i, j)] && limit.is_valid(&next_path, j) {
                    to_probe.push(next_path.with_cave(j));
                }
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: [&str; 7] = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];

    const EXAMPLE_2: [&str; 10] = [
        "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
        "kj-dc",
    ];

    const EXAMPLE_3: [&str; 18] = [
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    #[test]
    fn _01_example_1() {
        let cave_system = CaveSystem::parse(&EXAMPLE_1).expect("no caves");
        assert_eq!(cave_system.caves.len(), 6);
        assert_eq!(traverse(&cave_system, SimpleLimiter::new()).len(), 10);
    }

    #[test]
    fn _01_example_2() {
        let cave_system = CaveSystem::parse(&EXAMPLE_2).expect("no caves");
        assert_eq!(cave_system.caves.len(), 7);
        assert_eq!(traverse(&cave_system, SimpleLimiter::new()).len(), 19);
    }

    #[test]
    fn _01_example_3() {
        let cave_system = CaveSystem::parse(&EXAMPLE_3).expect("no caves");
        assert_eq!(traverse(&cave_system, SimpleLimiter::new()).len(), 226);
    }

    #[test]
    fn _02_example_1() {
        let cave_system = CaveSystem::parse(&EXAMPLE_1).expect("no caves");
        assert_eq!(traverse(&cave_system, ComplexLimiter::new()).len(), 36);
    }

    #[test]
    fn _02_example_2() {
        let cave_system = CaveSystem::parse(&EXAMPLE_2).expect("no caves");
        assert_eq!(traverse(&cave_system, ComplexLimiter::new()).len(), 103);
    }

    #[test]
    fn _02_example_3() {
        let cave_system = CaveSystem::parse(&EXAMPLE_3).expect("no caves");
        assert_eq!(traverse(&cave_system, ComplexLimiter::new()).len(), 3509);
    }
}
