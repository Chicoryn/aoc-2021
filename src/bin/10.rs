use std::io;

use aoc_2021::input::*;

fn main() -> io::Result<()> {
    let lines = lines()?;
    let lines = lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let scores = score_autocomplete_lines(&lines);

    println!("{}", score_syntax_errors(&lines));
    println!("{}", middle_autocomplete_score(&scores));
    Ok(())
}

#[derive(Debug, PartialEq)]
enum ChunkParseErr {
    UnexpectedCharacter { expected: char, found: char },
    UnexpectedEOF { expected: char },
}

#[derive(Debug, PartialEq)]
struct Chunk {
    chunks: Vec<Chunk>,
    length: usize,
}

impl Chunk {
    fn expected_ending(opening_character: char) -> char {
        match opening_character {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        }
    }
}

fn try_skip_ch(s: &str, offset: &mut usize, expected: &str) -> Option<char> {
    if let Some(opening_character) = s[*offset..].chars().next() {
        if expected.contains(opening_character) {
            *offset += 1;
            Some(opening_character)
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_chunks(s: &str) -> Result<Vec<Chunk>, ChunkParseErr> {
    let mut chunks = vec![];
    let mut offset = 0;

    while let Some(opening_character) = try_skip_ch(s, &mut offset, "([{<") {
        let expected_ending_character = Chunk::expected_ending(opening_character);
        let new_chunks = parse_chunks(&s[offset..])?;
        let length = new_chunks.iter().map(|c| c.length).sum::<usize>();
        offset += length;

        if let Some(ending_character) = s[offset..].chars().next() {
            if ending_character != expected_ending_character {
                return Err(ChunkParseErr::UnexpectedCharacter {
                    found: ending_character,
                    expected: expected_ending_character,
                });
            }

            offset += 1;
            chunks.push(Chunk {
                chunks: new_chunks,
                length: 2 + length,
            });
        } else {
            return Err(ChunkParseErr::UnexpectedEOF {
                expected: expected_ending_character,
            });
        }
    }

    Ok(chunks)
}

fn score_syntax_errors(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| match parse_chunks(line) {
            Err(ChunkParseErr::UnexpectedCharacter { found, expected: _ }) => match found {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            },
            _ => 0,
        })
        .sum::<usize>()
}

fn autocomplete_line(line: &str) -> String {
    let mut new_str = line.to_string();
    let original_len = new_str.len();

    while let Err(ChunkParseErr::UnexpectedEOF { expected }) = parse_chunks(&new_str) {
        new_str.push(expected);
    }

    new_str[original_len..].to_string()
}

fn score_autocomplete_lines(lines: &[&str]) -> Vec<usize> {
    lines
        .iter()
        .filter_map(|line| {
            let added = autocomplete_line(line);

            if added == "" {
                None
            } else {
                Some(
                    added
                        .chars()
                        .map(|ch| match ch {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => unreachable!(),
                        })
                        .fold(0, |acc, next| 5 * acc + next),
                )
            }
        })
        .collect::<Vec<_>>()
}

fn middle_autocomplete_score(scores: &[usize]) -> usize {
    let mut scores = scores.to_vec();
    scores.sort();

    if scores.len() % 2 == 0 {
        unreachable!()
    } else {
        scores[scores.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 10] = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn _01_parse() {
        assert!(parse_chunks("()").is_ok());
        assert!(parse_chunks("[]").is_ok());
        assert!(parse_chunks("([])").is_ok());
        assert!(parse_chunks("{()()()}").is_ok());
        assert!(parse_chunks("<([{}])>").is_ok());
        assert!(parse_chunks("[<>({}){}[([])<>]]").is_ok());
        assert!(parse_chunks("(((((((((())))))))))").is_ok());
        assert!(parse_chunks("(]").is_err());
        assert!(parse_chunks("{()()()>").is_err());
        assert!(parse_chunks("(((()))}").is_err());
        assert!(parse_chunks("<([]){()}[{}])").is_err());
    }

    #[test]
    fn _01_incomplete() {
        assert_eq!(
            parse_chunks("{([(<{}[<>[]}>{[]{[(<()>"),
            Err(ChunkParseErr::UnexpectedCharacter {
                expected: ']',
                found: '}'
            })
        );
        assert_eq!(
            parse_chunks("[[<[([]))<([[{}[[()]]]"),
            Err(ChunkParseErr::UnexpectedCharacter {
                expected: ']',
                found: ')'
            })
        );
        assert_eq!(
            parse_chunks("[{[{({}]{}}([{[{{{}}([]"),
            Err(ChunkParseErr::UnexpectedCharacter {
                expected: ')',
                found: ']'
            })
        );
        assert_eq!(
            parse_chunks("[<(<(<(<{}))><([]([]()"),
            Err(ChunkParseErr::UnexpectedCharacter {
                expected: '>',
                found: ')'
            })
        );
        assert_eq!(
            parse_chunks("<{([([[(<>()){}]>(<<{{"),
            Err(ChunkParseErr::UnexpectedCharacter {
                expected: ']',
                found: '>'
            })
        );
    }

    #[test]
    fn _01_example() {
        assert_eq!(score_syntax_errors(&EXAMPLE), 26397);
    }

    #[test]
    fn _02_complete() {
        assert_eq!(autocomplete_line("[({(<(())[]>[[{[]{<()<>>"), "}}]])})]");
        assert_eq!(autocomplete_line("[(()[<>])]({[<{<<[]>>("), ")}>]})");
        assert_eq!(autocomplete_line("(((({<>}<{<{<>}{[]{[]{}"), "}}>}>))))");
        assert_eq!(autocomplete_line("{<[[]]>}<{[{[{[]{()[[[]"), "]]}}]}]}>");
        assert_eq!(autocomplete_line("<{([{{}}[<[[[<>{}]]]>[]]"), "])}>");
    }

    #[test]
    fn _02_scores() {
        assert_eq!(
            score_autocomplete_lines(&EXAMPLE),
            vec![288957, 5566, 1480781, 995444, 294]
        );
    }

    #[test]
    fn _02_example() {
        let scores = score_autocomplete_lines(&EXAMPLE);

        assert_eq!(middle_autocomplete_score(&scores), 288957);
    }
}
