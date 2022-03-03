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
        let elapsed = Instant::now().duration_since(start);

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
                print!("Day {} - {:?} Error: {}", $d::DAY, $d::TITLE, e)
            }
        }
    }};
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
            let path = $d::get_input_path();

            ::std::fs::metadata(&path).expect(&format!("File: {:?} missing", path));
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
///   aoc::test!(
///     day_xx,
///     INPUT,
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
    ($d: ident, $input: expr, $e1: expr, $e2: expr $(,$name: expr)? ) => {
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
        let elapsed = Instant::now().duration_since(start);

        assert_eq!(result, 42);
        assert!(
            time <= elapsed,
            "bench time should be lower or equal than outer scope duration"
        );
    }

    use Demo as test_macro;
    test!(test_macro, "Some Input", None, Some(123), "with_suffix");
    test!(test_macro, "Some Input", None, Some(123));
}
