use std::io;

use aoc_2021::input::*;
use aoc_2021::submarine::*;
use aoc_2021::submarine_command::*;

pub fn main() -> io::Result<()> {
    let planned_course = lines()?
        .iter()
        .map(|command| command.parse::<Command>().expect("unrecognized command"))
        .collect::<Vec<_>>();

    println!(
        "{}",
        planned_course
            .iter()
            .fold(SubmarineV1::default(), |curr, command| execute(
                curr, command
            ))
            .product()
    );
    println!(
        "{}",
        planned_course
            .iter()
            .fold(SubmarineV2::default(), |curr, command| execute(
                curr, command
            ))
            .product()
    );
    Ok(())
}

fn execute<'a, S: Submarine<S>>(mut sub: S, command: &Command) -> S {
    match *command {
        Command::Forward(dh) => sub.forward(dh),
        Command::Down(dd) => sub.down(dd),
        Command::Up(dd) => sub.up(dd),
    };
    sub
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planned_course() -> Vec<Command> {
        let planned_course = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        planned_course
            .iter()
            .map(|s| s.parse::<Command>().expect("unrecognized command"))
            .collect::<Vec<_>>()
    }

    #[test]
    fn _01_position() {
        let final_position = planned_course()
            .into_iter()
            .fold(SubmarineV1::default(), |curr, command| {
                execute(curr, &command)
            });
        assert_eq!(final_position.product(), 150);
    }

    #[test]
    fn _02_position() {
        let final_position = planned_course()
            .into_iter()
            .fold(SubmarineV2::default(), |curr, command| {
                execute(curr, &command)
            });
        assert_eq!(final_position.product(), 900);
    }
}
