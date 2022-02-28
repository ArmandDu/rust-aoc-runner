use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

use humantime::format_duration;
use thiserror::Error;

use crate::time;

#[derive(Debug, Error)]
pub enum SolutionError {
    #[error("Input Parse error")]
    ParseError,
    #[error("Input file is missing")]
    NoInput(#[from] std::io::Error),
    #[error("Error while running solution")]
    Run,
}

pub struct SolutionResult<P1, P2> {
    title: &'static str,
    day: u8,
    part1: Option<P1>,
    part2: Option<P2>,
    parse_duration: Duration,
    part1_duration: Duration,
    part2_duration: Duration,
}

impl<P1: Display, P2: Display> Display for SolutionResult<P1, P2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let heading = {
            let title = format!("Day {:02}: {:?}", self.day, self.title,);
            let sep: String = (0..=(title.len() + 1)).map(|_| '=').collect();

            format!("{}\n {}\n{}", sep, title, sep)
        };

        match (&self.part1, &self.part2) {
            (Some(p1), Some(p2)) => {
                write!(
                    f,
                   "{}\nPart 1: '{}'\nPart 2: '{}'\n----\nTime1:\t\t{}\nTime2:\t\t{}\nParse Time:\t{}\nTotal Time:\t{}",
                   heading,
                    p1,
                    p2,
                   format_duration(self.part1_duration).to_string(),
                   format_duration(self.part2_duration).to_string(),
                   format_duration(self.parse_duration).to_string(),
                   format_duration(self.part1_duration + self.part2_duration + self.parse_duration).to_string(),
                )
            }
            (Some(p1), _) => {
                write!(
                    f,
                    "{}\nPart 1: '{}'\n----\nTime1:\t\t{}\nParse Time:\t{}\nTotal Time:\t{}",
                    heading,
                    p1,
                    format_duration(self.part1_duration).to_string(),
                    format_duration(self.parse_duration).to_string(),
                    format_duration(self.part1_duration + self.parse_duration).to_string(),
                )
            }
            _ => {
                write!(
                    f,
                    "{}\n  {}\tParsing time",
                    heading,
                    format_duration(self.parse_duration).to_string(),
                )
            }
        }
    }
}

pub trait Solution {
    const TITLE: &'static str;
    const DAY: u8;

    type Input: Sync;
    type P1: Send;
    type P2: Send;

    fn parse(input: &str) -> Result<Self::Input, SolutionError>;

    fn part1(input: &Self::Input) -> Option<Self::P1>;
    fn part2(input: &Self::Input) -> Option<Self::P2>;

    fn test_part1(input: &str) -> Result<(Option<Self::P1>, Duration), SolutionError> {
        let (input, parse_time) = time!(Self::parse(input)?);
        let (r, time) = time!(Self::part1(&input));

        Ok((r, parse_time + time))
    }

    fn test_part2(input: &str) -> Result<(Option<Self::P2>, Duration), SolutionError> {
        let (input, parse_time) = time!(Self::parse(input)?);
        let (r, time) = time!(Self::part2(&input));

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
    /// use aoc::Solution;
    ///# use aoc::solution::SolutionError;
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     //snip implementation...
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///#     type Input = ();type P1 = usize; type P2 = usize;
    ///#
    ///#     fn parse(input: &str) -> Result<Self::Input, SolutionError> {
    ///#         Ok(())
    ///#         }
    ///#
    ///#     fn part1(input: &Self::Input) -> Option<Self::P1> {
    ///#         Some(0)
    ///#     }
    ///#
    ///#     fn part2(input: &Self::Input) -> Option<Self::P2> {
    ///#         Some(0)
    ///#     }
    /// }
    ///
    /// fn run_solution() {
    ///     match DayXX::run() {
    ///         Ok(solution) => println!("{}", solution),
    ///         Err(e) => println!("Failed to solve day: {}", e)
    ///     }   
    /// }
    ///
    /// ```
    fn run() -> Result<SolutionResult<Self::P1, Self::P2>, SolutionError> {
        let input = std::fs::read_to_string(&Self::get_input_path())?;

        let (input, parse_time) = time!(Self::parse(input.trim())?);
        let (p1, t1) = time!(Self::part1(&input));
        let (p2, t2) = time!(Self::part2(&input));

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

    /// Parallel Solution runner
    ///
    /// Runs [Solution::part1] and [Solution::part2] in parallel to optimize execution speed
    ///
    /// See [Solution::run] for reference
    ///
    /// Example
    /// -------
    /// ```
    /// use aoc::Solution;
    ///# use aoc::solution::SolutionError;
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     //snip implementation...
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///#     type Input = ();type P1 = usize;type P2 = usize;
    ///#
    ///#     fn parse(input: &str) -> Result<Self::Input, SolutionError> {
    ///#         Ok(())
    ///#         }
    ///#
    ///#     fn part1(input: &Self::Input) -> Option<Self::P1> {
    ///#         Some(0)
    ///#     }
    ///#
    ///#     fn part2(input: &Self::Input) -> Option<Self::P2> {
    ///#         Some(0)
    ///#     }
    /// }
    ///
    /// fn run_solution() {
    ///     match DayXX::run_par() {
    ///         Ok(solution) => println!("{}", solution),
    ///         Err(e) => println!("Failed to solve day: {}", e)
    ///     }   
    /// }
    ///
    /// ```    
    fn run_par() -> Result<SolutionResult<Self::P1, Self::P2>, SolutionError> {
        let input = std::fs::read_to_string(&Self::get_input_path())?;

        let (input, parse_time) = time!(Self::parse(input.trim())?);

        let scope = crossbeam_utils::thread::scope(|s| {
            let solve1 = s.spawn(|_| time!(Self::part1(&input)));
            let solve2 = s.spawn(|_| time!(Self::part2(&input)));

            let solve1 = solve1.join();
            let solve2 = solve2.join();

            (solve1, solve2)
        })
        .map_err(|_| SolutionError::Run)?;

        match scope {
            (Ok((part1, part1_duration)), Ok((part2, part2_duration))) => Ok(SolutionResult {
                title: Self::TITLE,
                day: Self::DAY,
                parse_duration: parse_time,
                part1,
                part1_duration,
                part2,
                part2_duration,
            }),
            _ => Err(SolutionError::Run),
        }
    }
}
