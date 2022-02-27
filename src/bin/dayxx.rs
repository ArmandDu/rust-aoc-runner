use aoc::solution::{Solution, SolutionError};

struct DayXX;

impl Solution for DayXX {
    const TITLE: &'static str = "Hello World!";
    const DAY: u8 = 0;
    type Input = String;
    type P1 = String;
    type P2 = String;

    fn parse(input: &str) -> Result<Self::Input, SolutionError> {
        Ok(input.to_owned())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(input.to_lowercase())
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(input.to_uppercase())
    }
}

fn main() {
    aoc::solution!(DayXX);
}

#[cfg(test)]
mod tests {
    use crate::DayXX as day_xx;
    use crate::*;

    aoc::test_common!(day_xx);

    aoc::test!(
        day_xx,
        "Hello",
        Some("hello".to_owned()),
        Some("HELLO".to_owned())
    );
}
