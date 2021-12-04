use std::io;

use aoc_2021::input::*;

pub fn main() -> io::Result<()> {
    let (draw, mut boards) = parse_bingo(&lines()?).ok_or(io::ErrorKind::Other)?;

    println!(
        "{}",
        play_bingo(&draw, &mut boards).ok_or(io::ErrorKind::Other)?
    );
    println!(
        "{}",
        lose_bingo(&draw, &mut boards).ok_or(io::ErrorKind::Other)?
    );
    Ok(())
}

fn play_bingo(draw: &[usize], boards: &mut [Board]) -> Option<usize> {
    for &number in draw {
        for board in &mut *boards {
            board.play(number);

            if board.has_won() {
                return Some(board.score() * number);
            }
        }
    }

    None
}

fn lose_bingo(draw: &[usize], boards: &mut [Board]) -> Option<usize> {
    let mut boards = boards.to_vec();

    for &number in draw {
        for board in &mut boards {
            board.play(number);
        }

        if boards.iter().all(|board| board.has_won()) {
            return Some(boards[0].score() * number);
        }

        boards.retain(|board| !board.has_won());
    }

    None
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Number {
    Unmarked(usize),
    Marked(usize),
}

impl PartialEq<usize> for Number {
    fn eq(&self, other: &usize) -> bool {
        match *self {
            Self::Unmarked(n) => n == *other,
            Self::Marked(n) => n == *other,
        }
    }
}

impl Number {
    fn is_marked(&self) -> bool {
        match *self {
            Number::Unmarked(_) => false,
            Number::Marked(_) => true,
        }
    }
}

#[derive(Clone)]
struct Board {
    numbers: [Number; 25], // row-major
}

impl Board {
    fn play(&mut self, number: usize) {
        for n in &mut self.numbers {
            if *n == Number::Unmarked(number) {
                *n = Number::Marked(number);
            }
        }
    }

    fn score(&self) -> usize {
        self.numbers
            .iter()
            .map(|x| match *x {
                Number::Unmarked(n) => n,
                Number::Marked(_) => 0,
            })
            .sum::<usize>()
    }

    fn has_won(&self) -> bool {
        for i in 0..5 {
            if (0..5).all(|j| self.numbers[5 * i + j].is_marked()) {
                return true;
            } else if (0..5).all(|j| self.numbers[5 * j + i].is_marked()) {
                return true;
            }
        }

        false
    }
}

fn parse_bingo_draw(line: &String) -> Option<Vec<usize>> {
    Some(
        line.trim()
            .split(',')
            .map(|n| n.parse::<usize>().expect("not a number"))
            .collect(),
    )
}

fn parse_bingo_board<'a>(lines: &mut impl Iterator<Item = &'a String>) -> Option<Board> {
    let mut board = Board {
        numbers: [Number::Unmarked(0); 25],
    };

    for i in 0..5 {
        for (j, number) in lines
            .next()?
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<usize>())
            .enumerate()
        {
            board.numbers[5 * i + j] = Number::Unmarked(number.ok()?);
        }
    }

    Some(board)
}

fn parse_bingo(lines: &[String]) -> Option<(Vec<usize>, Vec<Board>)> {
    let mut lines = lines.iter().filter(|x| !x.trim().is_empty());
    let draw = parse_bingo_draw(lines.next()?)?;
    let mut boards = vec![];

    while let Some(board) = parse_bingo_board(&mut lines) {
        boards.push(board);
    }

    Some((draw, boards))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn _01_example() {
        let lines = EXAMPLE
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let (draw, mut boards) = parse_bingo(&lines).expect("could not parse");

        assert_eq!(
            draw,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(boards.len(), 3);
        assert_eq!(
            boards[0].numbers,
            [
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19
            ]
        );
        assert_eq!(
            boards[1].numbers,
            [
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6
            ]
        );
        assert_eq!(
            boards[2].numbers,
            [
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7
            ]
        );
        assert_eq!(play_bingo(&draw, &mut boards), Some(4512));
    }

    #[test]
    fn _02_example() {
        let lines = EXAMPLE
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let (draw, mut boards) = parse_bingo(&lines).expect("could not parse");

        assert_eq!(lose_bingo(&draw, &mut boards), Some(1924));
    }
}
