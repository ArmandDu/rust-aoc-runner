//! Main module for this library
//!
//! Contains the [Solution] trait and related structures.
//!
//! You usually want to use this module when you need to implement the [Solution] trait.
//!

use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

use humantime::format_duration;
use thiserror::Error;

use crate::time;

#[derive(Debug, Error)]
pub enum SolutionError {
    #[error("Invalid Puzzle input")]
    ParseError,
    #[error("Missing Puzzle input")]
    PuzzleInput(#[from] std::io::Error),
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

pub type Result<T> = std::result::Result<T, SolutionError>;

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
                   format_duration(self.part1_duration),
                   format_duration(self.part2_duration),
                   format_duration(self.parse_duration),
                   format_duration(self.part1_duration + self.part2_duration + self.parse_duration),
                )
            }
            (Some(p1), _) => {
                write!(
                    f,
                    "{}\nPart 1: '{}'\n----\nTime1:\t\t{}\nParse Time:\t{}\nTotal Time:\t{}",
                    heading,
                    p1,
                    format_duration(self.part1_duration),
                    format_duration(self.parse_duration),
                    format_duration(self.part1_duration + self.parse_duration),
                )
            }
            _ => {
                write!(
                    f,
                    "{}\n  {}\tParsing time",
                    heading,
                    format_duration(self.parse_duration),
                )
            }
        }
    }
}

/// Main trait for Advent of Code Daily challenges.
///
/// This trait includes a generic runner part, a utility part and methods to implement each day.
///
/// ### The runner part includes:
/// - [Solution::test_part1]
/// - [Solution::test_part2]
/// - [Solution::run]
/// - [Solution::run_par]
///
/// Those associated methods are implemented by default and are intended to be used as is.
///
/// ### The utility part includes:
/// - [Solution:get_input]
///
/// The utility part comes pre define but can be ovewritten if one chooses to.
///
/// ### The methods to be implemented each day are:
/// - [Solution::TITLE] - used for displaying the solution
/// - [Solution::DAY] - used by [Solution::get_input]'s default implementation
/// - [Solution::parse] - pre process the puzzle input for the other parts
/// - [Solution::part1] - solution for part 1
/// - [Solution::part2] - solution for part 2
///
/// # Example
/// ```
/// use aoc::Solution;
/// use aoc::solution::Result;
/// struct DayXX;
///
/// impl Solution for DayXX {
///     const TITLE: &'static str = "";
///     const DAY: u8 = 0;
///     
///     type Input = ();
///     type P1 = ();
///     type P2 = ();
///
///     fn parse(input: &str) -> Result<Self::Input> {
///         Ok(())
///     }
///
///     fn part1(input: &Self::Input) -> Option<Self::P1> {
///         None
///     }
///
///     fn part2(input: &Self::Input) -> Option<Self::P2> {
///         None
///     }
/// }
///
/// DayXX::run();
/// ```
///
pub trait Solution {
    const TITLE: &'static str;
    const DAY: u8;

    /// Puzzle input type.
    /// it's the output value of [Solution::parse]
    /// and is consumed by [Solution::part1] and [Solution::part2]
    type Input: Sync;

    /// Part 1 Solution type.
    /// it's the output value of [Solution::part1]
    type P1: Send + Debug;

    /// Part 2 Solution type.
    /// it's the output value of [Solution::part2]
    type P2: Send + Debug;

    /// Takes the puzzle input as &str and parses it to something more flexible
    /// to solve the exercices.
    ///
    /// # Example
    /// ```
    /// use aoc::Solution;
    ///# use aoc::solution::Result;
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     //--snip--
    ///
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///     type Input = Vec<usize>;
    ///#     type P1 = usize; type P2 = usize;
    ///
    ///     fn parse(input: &str) -> Result<Self::Input> {
    ///         Ok(input
    ///             .lines()
    ///             .filter_map(|line| line.parse().ok())
    ///             .collect())
    ///      }
    ///#
    ///#     fn part1(input: &Self::Input) -> Option<Self::P1> {
    ///#         Some(0)
    ///#     }
    ///#
    ///#     fn part2(input: &Self::Input) -> Option<Self::P2> {
    ///#         Some(0)
    ///#     }
    /// }
    /// ```
    ///
    fn parse(input: &str) -> Result<Self::Input>;

    /// Takes the [Solution::parse]'s output and return the solution for part 1
    ///
    /// You must implement this method. If the method cannot be implemented,
    /// return None as a placeholder
    ///
    /// # Example
    /// ```
    /// use aoc::Solution;
    ///# use aoc::solution::Result;
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     //-- snip --
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///#     type Input = ();type P1 = (); type P2 = ();
    ///#
    ///#     fn parse(input: &str) -> Result<Self::Input> {
    ///#         Ok(())
    ///#         }
    ///#
    ///     fn part1(input: &Self::Input) -> Option<Self::P1> {
    ///         None
    ///     }
    ///#
    ///#     fn part2(input: &Self::Input) -> Option<Self::P2> {
    ///#         None
    ///#     }
    /// }    
    ///```
    ///
    fn part1(input: &Self::Input) -> Option<Self::P1>;
    /// Takes the [Solution::parse]'s output and return the solution for part 2
    ///
    /// You must implement this method. If the method cannot be implemented
    /// (eg: part 2 unvavailable), return None as a placeholder    
    /// # Example
    /// ```
    /// use aoc::Solution;
    ///# use aoc::solution::Result;
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     //-- snip --
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///#     type Input = ();type P1 = (); type P2 = ();
    ///#
    ///#     fn parse(input: &str) -> Result<Self::Input> {
    ///#         Ok(())
    ///#         }
    ///#
    ///#     fn part1(input: &Self::Input) -> Option<Self::P1> {
    ///#         None
    ///#     }
    ///#
    ///     fn part2(input: &Self::Input) -> Option<Self::P2> {
    ///         None
    ///     }
    /// }    
    ///```
    fn part2(input: &Self::Input) -> Option<Self::P2>;

    /// Utility method used to test Part 1.
    ///
    /// This is generaly used in unit tests but can also be used in the main function
    /// Use it to test your solution against smaller inputs and for debugging.
    ///
    /// # Example
    /// ```
    /// use aoc::Solution;
    ///# use aoc::solution::{Result, SolutionError};
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     // -- snip --
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///#     type Input = usize; type P1 = usize; type P2 = usize;
    ///#
    ///    fn parse(input: &str) -> Result<Self::Input> {
    ///        match input.parse() {
    ///           Ok(num) => Ok(num),
    ///           Err(_) => Err(SolutionError::ParseError)
    ///        }   
    ///     }
    ///#
    ///     fn part1(input: &Self::Input) -> Option<Self::P1> {
    ///         Some(*input)
    ///     }
    ///#
    ///#     fn part2(input: &Self::Input) -> Option<Self::P2> {
    ///#         Some(0)
    ///#     }
    /// }
    ///
    /// #[cfg(test)]
    /// mod tests {
    ///
    /// # use std::assert_eq;
    /// #[test]
    ///     fn test() {
    ///         let (actual, _) = DayXX::test_part1("123");
    ///         assert_eq!(actual, Some(123));
    ///     }
    /// }
    ///
    /// ```
    fn test_part1(input: &str) -> Result<(Option<Self::P1>, Duration)> {
        let (input, parse_time) = time!(Self::parse(input)?);
        let (actual, time) = time!(Self::part1(&input));
        let total_time = time + parse_time;

        println!(
            "Part1: {:?} (in {})",
            actual,
            format_duration(total_time)
        );

        Ok((actual, total_time))
    }

    /// Utility method used to test Part 2.
    ///
    /// This is generaly used in unit tests but can also be used in the main function
    /// Use it to test your solution against smaller inputs and for debugging.
    ///
    /// # Example
    /// ```
    /// use aoc::Solution;
    ///# use aoc::solution::{Result, SolutionError};
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     // -- snip --
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///#     type Input = i32; type P1 = i32; type P2 = i32;
    ///#
    ///    fn parse(input: &str) -> Result<Self::Input> {
    ///        match input.parse() {
    ///           Ok(num) => Ok(num),
    ///           Err(_) => Err(SolutionError::ParseError)
    ///        }   
    ///     }
    ///#
    ///#     fn part1(input: &Self::Input) -> Option<Self::P1> {
    ///#         Some(*input)
    ///#     }
    ///#
    ///     fn part2(input: &Self::Input) -> Option<Self::P2> {
    ///         Some(-input)
    ///     }
    /// }
    ///
    /// #[cfg(test)]
    /// mod tests {
    ///
    /// # use std::assert_eq;
    /// #[test]
    ///     fn test() {
    ///         let (actual, _) = DayXX::test_part2("123");
    ///         assert_eq!(actual, Some(-123));
    ///     }
    /// }
    ///
    /// ```
    fn test_part2(input: &str) -> Result<(Option<Self::P2>, Duration)> {
        let (input, parse_time) = time!(Self::parse(input)?);
        let (actual, time) = time!(Self::part2(&input));
        let total_time = time + parse_time;

        println!(
            "Part2: {:?} (in {})",
            actual,
            format_duration(total_time)
        );

        Ok((actual, total_time))
    }

    /// Optional overridable method.
    /// By default, the Self::get_input() will seek an input file under `"<root>/inputs/DAY_<XX>.txt"`
    ///
    /// The `<XX>` part corresponds to the [Solution::DAY] value.
    ///
    /// If one wants to overwrite the input file for a given solution, then it's possible to
    /// overwrite this method.
    ///
    /// Example
    /// -------
    /// ```
    /// use aoc::solution::Result;
    ///
    /// fn get_input() -> Result<String> {
    ///     Ok("Some Hardcoded Input".to_owned())
    /// }
    ///
    /// ```
    fn get_input() -> Result<String> {
        let path = format!("inputs/DAY_{:02}.txt", Self::DAY);
        let input = std::fs::read_to_string(&path)?;

        Ok(input)
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
    ///# use aoc::solution::Result;
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     // -- snip --
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///#     type Input = ();type P1 = usize; type P2 = usize;
    ///#
    ///#     fn parse(input: &str) -> Result<Self::Input> {
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
    fn run() -> Result<SolutionResult<Self::P1, Self::P2>> {
        let input = Self::get_input()?;

        let (input, parse_time) = time!(Self::parse(&input)?);
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
    ///# use aoc::solution::Result;
    ///
    /// struct DayXX;
    /// impl Solution for DayXX {
    ///     // -- snip --
    ///#     const TITLE: &'static str = "";const DAY: u8 = 0;
    ///#     type Input = ();type P1 = usize;type P2 = usize;
    ///#
    ///#     fn parse(input: &str) -> Result<Self::Input> {
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
    fn run_par() -> Result<SolutionResult<Self::P1, Self::P2>> {
        let input = Self::get_input()?;

        let (input, parse_time) = time!(Self::parse(&input)?);

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
