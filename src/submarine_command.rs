use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum CommandParseErr {
    TooFewTokens(String),
    MissingUnits(String),
    UnrecognizedCommand(String),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Command {
    type Err = CommandParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect::<Vec<_>>();

        if parts.len() != 2 {
            Err(CommandParseErr::TooFewTokens(s.to_string()))
        } else {
            let amount = parts[1]
                .parse::<isize>()
                .map_err(|_| CommandParseErr::MissingUnits(parts[1].to_string()))?;

            match parts[0] {
                "forward" => Ok(Command::Forward(amount)),
                "down" => Ok(Command::Down(amount)),
                "up" => Ok(Command::Up(amount)),
                _ => Err(CommandParseErr::UnrecognizedCommand(parts[0].to_string())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01_parse_commands() {
        let planned_course = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        assert_eq!(
            planned_course
                .iter()
                .map(|command| command.parse::<Command>())
                .collect::<Vec<_>>(),
            vec![
                Ok(Command::Forward(5)),
                Ok(Command::Down(5)),
                Ok(Command::Forward(8)),
                Ok(Command::Up(3)),
                Ok(Command::Down(8)),
                Ok(Command::Forward(2))
            ]
        );
    }
}
