//! Collection of macros designed to reduce the amount of code duplicated for each days
//!
//! Since the Advent of Code is a daily challenge, we need to duplicate some boilerplate every day.
//! Those macros are intended to reduce the amount of code to be copied / pasted each day.
//!
//! You'll usually want to mainly use the `solution!` macro inside your main function.
//! Or use the `test_common!` and `test!` macros in your tests module.

///Execute an expression and return it with its execution Duration.
///
/// #Example
///```
/// use std::time::Duration;
///
/// fn expensive() -> i32 {
///    std::thread::sleep(Duration::from_secs(2));
///    42
///}
///
/// let (result, duration): (i32, Duration) = aoc::time!(expensive());
///
/// assert_eq!(result, 42);
/// assert_eq!(duration.as_secs(), 2)
///
/// ```
#[macro_export]
macro_rules! time {
    ($e:expr) => {{
        use ::std::time::Instant;

        let start = Instant::now();
        let result = $e;
        let elapsed = start.elapsed();

        (result, elapsed)
    }};
}

/// Utility macro that calls [crate::Solution::run] and displays it's output
///
/// # Example
/// ```
/// use aoc::Solution;
///# use aoc::solution::SolutionError;
///
/// struct DayXX;
/// impl Solution for DayXX {
///     //-- snip --
///#     const TITLE: &'static str = "";const DAY: u8 = 0;
///#     type Input = ();type P1 = usize; type P2 = usize;
///#
///#     fn parse(input: &str) -> Result<Self::Input, SolutionError> {
///#         Ok(())
///#         }
///#
///#     fn part1(input: &Self::Input) -> Option<Self::P1> {
///#         Some(123)
///#     }
///#
///#     fn part2(input: &Self::Input) -> Option<Self::P2> {
///#         Some(456)
///#     }
/// }
///
/// fn run_solution() {
///     aoc::solution!(DayXX);
/// }
/// ```
#[macro_export]
macro_rules! solution {
    ($d: ident) => {{
        match $d::run_par() {
            Ok(result) => {
                println!("{}", result)
            }
            Err(e) => {
                println!("Day {} - {:?} Error: {}", $d::DAY, $d::TITLE, e)
            }
        }
    }};
}
/// Wraps aoc::solution! inside a main function
///
/// Helper function when the main is only in charge of running 1 solution.
///
/// @example
/// ```
/// use aoc::Solution;
///# use aoc::solution::SolutionError;
///
/// struct DayXX;
/// impl Solution for DayXX {
///     //-- snip --
///#     const TITLE: &'static str = "";const DAY: u8 = 0;
///#     type Input = ();type P1 = usize; type P2 = usize;
///#
///#     fn parse(input: &str) -> Result<Self::Input, SolutionError> {
///#         Ok(())
///#         }
///#
///#     fn part1(input: &Self::Input) -> Option<Self::P1> {
///#         Some(123)
///#     }
///#
///#     fn part2(input: &Self::Input) -> Option<Self::P2> {
///#         Some(456)
///#     }
/// }
///
/// aoc::run!(DayXX);
/// ```
///
#[macro_export]
macro_rules! run {
    ($d:ident) => {
        fn main() {
            ::aoc::solution!($d)
        }
    }
}


/// Wrapper for `impl Solution for $name {}`
///
/// This wrapper will create the struct and implementation.
/// The only parts left to fill are the dynamic information:
///  - name             - name of the struct. Eg: Day00
///  - title            - title of day's puzzle
///  - day              - puzzle's day
///  - parse function   - parse input into Self::Input
///  - part_1 function  - solve part 1 of puzzle
///  - part_2 function  - solve part 2 of puzzle
/// @example
/// ```
///use itertools::Itertools;
///use aoc::solution::SolutionError;
///
///aoc::implement! {
///    name: Day00;
///    title: "addition or product";
///    day: 0;
///#    input : "12345".to_owned();
///    parse   -> Vec<u32> : |input: &str| input.chars().map(|c| c.to_digit(10).ok_or(SolutionError::ParseError)).collect();
///    part_1  -> u32      : |input: &Self::Input| input.iter().sum1();
///    part_2  -> u32      : |input: &Self::Input| input.iter().product1();
///}
/// ```
///
#[macro_export]
macro_rules! implement {
    (
        name    :   $name:ident;
        title   :   $title:expr;
        day     :   $day:expr;
        $(input :   $input:expr;)?
        parse   -> $ti:ty :   $parse:expr;
        part_1  ->$tp1:ident :   $part1:expr;
        part_2  ->$tp2:ident :   $part2:expr;

    ) => {
        use aoc::Solution;
        struct $name;

        impl Solution for $name {
                const TITLE: &'static str = $title;
                const DAY: u8 = $day;
                type Input = $ti;
                type P1 = $tp1;
                type P2 = $tp2;

                fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
                    let fun = $parse;
                    fun(input)
                }

                fn part1(input: &Self::Input) -> Option<Self::P1> {
                    let fun = $part1;
                    fun(input)
                }

                fn part2(input: &Self::Input) -> Option<Self::P2> {
                   let fun = $part2;
                    fun(input)
                }
            $(
                fn get_input() -> aoc::solution::Result<String> {
                    Ok($input)
                }
            )?
        }
    }
}

/// Wrapper/Simplification over the test! macro
/// This simplifies the test! macro usage and hides some of its caveats.
///
/// This macro now creates the whole tests module and populate tests for each example line
///
/// @example
/// ```
/// use aoc::Solution;
///# use aoc::solution::SolutionError;
///
/// struct DayXX;
/// impl Solution for DayXX {
///     //-- snip --
///#     const TITLE: &'static str = "";const DAY: u8 = 0;
///#     type Input = ();type P1 = usize; type P2 = usize;
///#
///#     fn parse(input: &str) -> Result<Self::Input, SolutionError> {
///#         Ok(())
///#         }
///#
///#     fn part1(input: &Self::Input) -> Option<Self::P1> {
///#         Some(123)
///#     }
///#
///#     fn part2(input: &Self::Input) -> Option<Self::P2> {
///#         Some(456)
///#     }
/// }
///
/// aoc::example! {
///     [DayXX]
///     example: "123" => Some(123) => Some(456)
/// }
/// ```
///
#[macro_export]
macro_rules! example {
    (
        [$d:ident]
        $(
            $name:ident: $input:expr
                => $part1:expr
                $(=> $part2:expr)?
        )+
    ) => {
       $(
        ::concat_idents::concat_idents!(mod_name = tests, _, $name {
            #[cfg(test)]
            mod mod_name {
                 use crate::*;
                 use crate::{$d};

                 #[test]
                 fn part1() {
                     let (r, _) = $d::test_part1($input).expect("couldn't run test:");
                     assert_eq!(r, $part1);
                 }

             $(
                 #[test]
                 fn part2() {
                     let (r, _) = $d::test_part2($input).expect("couldn't run test:");
                     assert_eq!(r, $part2);
                 }
             )?
            }
        });
       )+
    }
}


/// Repeating tests that can be run for each Solution.
///
/// Compared to `aoc::test!` macro, this one is expected to exists only once per tests module.
/// The reason is that the test name are not generated based on input. Calling the macro twice
/// will throw a compilation error.
///
/// # Example
/// ```
/// #[cfg(test)]
/// mod tests {
///   use crate::*;
///   use crate::{DayXX as day_xx};
///
///   aoc::test_common!(day_xx);
/// }
/// ```
///
#[macro_export]
macro_rules! test_common {
    ($d: ident) => {
        #[test]
        fn input_exists() {
            $d::get_input().expect("An input is required");
        }
    };
}
/// Helper macro to generate tests for a Solution
///
/// Will:
/// - generate tests for test_part1 and test_part2
/// - call [crate::Solution::test_part1] and [crate::Solution::test_part2] under the hood
/// - assert for result equality
///
/// Example
/// -------
/// ```
/// use aoc::Solution;
///# use aoc::solution::SolutionError;
///
/// struct DayXX;
/// impl Solution for DayXX {
///     //-- snip --
///#     const TITLE: &'static str = "";const DAY: u8 = 0;
///#     type Input = ();type P1 = usize; type P2 = usize;
///#
///#     fn parse(input: &str) -> Result<Self::Input, SolutionError> {
///#         Ok(())
///#         }
///#
///     fn part1(input: &Self::Input) -> Option<Self::P1> { Some(123) }
///     fn part2(input: &Self::Input) -> Option<Self::P2> { Some(456) }
/// }
///
/// #[cfg(test)]
/// mod tests {
///   use crate::*;
///   use crate::{DayXX as day_xx};
///
///   aoc::test! {
///      day_xx:
///      [case_1]
///         - "Some Input" => Some(123) => Some(456);
///      [case_2]
///         - "Other Input" => Some(123) => Some(456);
///     }
///
///   //alternate syntax
///   aoc::test!(
///     day_xx,
///     "Another Input",
///     Some(123), //expected result for part 1
///     Some(456),  //expected result for part 2
///     //add a unique suffix when macro
///     // is used multiple times in the same module
///     "optional_suffix"
///   );
/// }
///
/// ```
#[macro_export]
macro_rules! test {
    (
        $d:ident:
        $(
            $( [$name:ident] )?
            - $input: expr => $part1:expr => $part2: expr $(;)?
        )+
     ) => {
       $(
         $crate::test!($d, $input, $part1, $part2 $(, $name )?);
       )+
    };
    ($d:ident, $input:expr, $e1:expr, $e2:expr $(, $name:expr )? ) => {
        ::concat_idents::concat_idents!(test_name = $d, _part1, $( _, $name)? {
            #[test]
            fn test_name() {
                let (r, _) = $d::test_part1($input).expect("couldn't run test:");
                assert_eq!(r, $e1);
            }
        });

        ::concat_idents::concat_idents!(test_name = $d, _part2, $( _, $name)? {
            #[test]
            fn test_name() {
                let (r, _) = $d::test_part2($input).expect("couldn't run test:");
                assert_eq!(r, $e2);
            }
        });
    };
}

#[cfg(test)]
mod tests {
    use crate::solution::SolutionError;
    use crate::*;
    use std::thread;
    use std::time::{Duration, Instant};

    struct Demo;
    impl Solution for Demo {
        const TITLE: &'static str = "";
        const DAY: u8 = 0;
        type Input = ();
        type P1 = ();
        type P2 = usize;

        fn parse(_input: &str) -> Result<Self::Input, SolutionError> {
            Ok(())
        }

        fn part1(_input: &Self::Input) -> Option<Self::P1> {
            None
        }

        fn part2(_input: &Self::Input) -> Option<Self::P2> {
            Some(123)
        }
    }

    #[test]
    fn time_macro() {
        let expr = || {
            thread::sleep(Duration::from_millis(10));
            42
        };

        let start = Instant::now();
        let (result, time) = time!(expr());
        let elapsed = start.elapsed();

        assert_eq!(result, 42);
        assert!(
            time <= elapsed,
            "bench time should be lower or equal than outer scope duration"
        );
    }

    use Demo as test_macro;

    test! {
        test_macro:
        [case_1]
        - "Some Input" => None => Some(123);
        [case_2]
        - "Other Input" => None => Some(123);
    }

    test! {
        test_macro:
        - "Some Input" => None => Some(123);
    }

    test!(test_macro, "Some Input", None, Some(123), "with_suffix");
}
