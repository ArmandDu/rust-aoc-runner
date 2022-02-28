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
        use std::time::Instant;

        let start = Instant::now();
        let result = $e;
        let elapsed = Instant::now().duration_since(start);

        (result, elapsed)
    }};
}

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

            std::fs::metadata(&path).expect(&format!("File: {:?} missing", path));
        }
    };
}

#[macro_export]
macro_rules! test {
    ($name: expr, $d: ident, $input: expr, $e1: expr, $e2: expr) => {
        concat_idents::concat_idents!(test_name = test_, $d, _part1_, $name {
            #[test]
            fn test_name() {
                use humantime::format_duration;

                let (r, t) = $d::test_part1($input).expect("couldn't run test:");

                println!("Part1: {:?} (in {})", r, format_duration(t).to_string());
                assert_eq!(r, $e1);
            }
        });

        concat_idents::concat_idents!(test_name = test_, $d, _part2_, $name {
            #[test]
            fn test_name() {
                use humantime::format_duration;

                let (r, t) = $d::test_part2($input).expect("couldn't run test:");

                println!("Part2: {:?} (in {})", r, format_duration(t).to_string());
                assert_eq!(r, $e2);
            }
        });
    };
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::{Duration, Instant};

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
}
