use std::time::Duration;

use derive_more::Display;
use thiserror::Error;

use crate::bench;

#[derive(Debug, Error)]
pub enum SolutionError {
    #[error("Input Parse error")]
    ParseError,
    #[error("Input file is missing")]
    NoInput(#[from] std::io::Error),
}

#[derive(Display)]
#[display(
    fmt = "Day {:02}: {:?}\n  {:.4}ms\tPart 1: '{:?}'\n  {:.4}ms\tPart 2: '{:?}'\n+ {:.4}ms\tParsing time\n= {:.4}ms\tTotal Time",
    day,
    title,
    "part1_duration.as_millis()",
    part1,
    "part2_duration.as_millis()",
    part2,
    "parse_duration.as_millis()",
    "parse_duration.as_millis() + part1_duration.as_millis() + part2_duration.as_millis()"
)]
pub struct SolutionResult<P1, P2> {
    title: &'static str,
    day: u8,
    part1: Option<P1>,
    part2: Option<P2>,
    parse_duration: Duration,
    part1_duration: Duration,
    part2_duration: Duration,
}

pub trait Solution {
    const TITLE: &'static str;
    const DAY: u8;
    const ASSETS: &'static str = "inputs";

    type Input;
    type P1;
    type P2;

    fn parse(input: &str) -> Result<Self::Input, SolutionError>;

    fn part1(input: &Self::Input) -> Option<Self::P1>;
    fn part2(input: &Self::Input) -> Option<Self::P2>;

    fn test_part1(input: &str) -> Result<(Option<Self::P1>, Duration), SolutionError> {
        let (input, parse_time) = bench!(Self::parse(input)?);
        let (r, time) = bench!(Self::part1(&input));

        Ok((r, parse_time + time))
    }

    fn test_part2(input: &str) -> Result<(Option<Self::P2>, Duration), SolutionError> {
        let (input, parse_time) = bench!(Self::parse(input)?);
        let (r, time) = bench!(Self::part2(&input));

        Ok((r, parse_time + time))
    }

    /// Optional overridable method.
    /// By default, the Self::run() will seek an input file under "<root>/inputs/DAY_<XX>.txt"
    ///
    /// If one wants to overwrite the input file for a given solution, then it's possible to
    /// overwrite this method.
    ///
    /// Example
    /// -------
    /// ```
    /// fn get_input_path() -> String {
    ///     String::from("data/input_day_xx.txt")
    /// }
    ///
    /// ```
    ///
    fn get_input_path() -> String {
        format!("inputs/DAY_{:02}.txt", Self::DAY)
    }

    /// Solution Runner
    ///
    /// This is the main entry point that we want to call for each day.
    ///
    /// This method is in charge of:
    /// - Reading the input file. (path provided by Self::get_input_path)
    /// - Parsing the input file. (Self::parse - must be implemented)
    /// - Solving Part1. (Self::part1 - must be implemented)
    /// - Solving Part2. (Self::part2 - must be implemented)
    /// - Returning a SolutionResult or a SolutionError
    ///
    /// Example
    /// -------
    /// ```
    ///
    /// struct DayXX;
    /// //impl Solution for DayXX {/*...*/}
    ///
    /// fn main() {
    ///     match DayXX::run() {
    ///         Ok(solution) => println!("{}", solution),
    ///         Err(e) => println!("Failed to solve day: {}", e)
    ///     }   
    /// }
    ///
    /// ```
    fn run() -> Result<SolutionResult<Self::P1, Self::P2>, SolutionError> {
        let input = std::fs::read_to_string(&Self::get_input_path())?;

        let (input, parse_time) = bench!(Self::parse(input.trim())?);
        let (p1, t1) = bench!(Self::part1(&input));
        let (p2, t2) = bench!(Self::part2(&input));

        Ok(SolutionResult {
            title: Self::TITLE,
            day: Self::DAY,
            parse_duration: parse_time,
            part1: p1,
            part1_duration: t1,
            part2: p2,
            part2_duration: t2,
        })
    }
}
