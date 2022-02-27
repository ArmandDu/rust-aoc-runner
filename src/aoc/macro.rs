#[macro_export]
macro_rules! bench {
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
        match $d::run() {
            Ok(result) => {
                println!("{}", result)
            }
            Err(e) => {
                print!("Day {} - {:?} Error: {}", $d::DAY, $d::TITLE, e)
            }
        }
    }};
}

#[macro_export]
macro_rules! test {
    ($d: ident, $i: expr, $e1: expr, $e2: expr) => {
        concat_idents::concat_idents!(test_name = test_, $d, _part1_, $i {
            #[test]
            fn test_name() {
                let (r, t) = $d::test_part1($i).expect("couldn't run test:");

                println!("Part1: {:?} (in {}ms)", r, t.as_millis());
                assert_eq!(r, $e1);
            }
        });

        concat_idents::concat_idents!(test_name = test_, $d, _part2_, $i {
            #[test]
            fn test_name() {
                let (r, t) = $d::test_part2($i).expect("couldn't run test:");

                println!("Part2: {:?} (in {}ms)", r, t.as_millis());
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
    fn bench_macro() {
        let expr = || {
            thread::sleep(Duration::from_millis(10));
            42
        };

        let start = Instant::now();
        let (result, time) = bench!(expr());
        let elapsed = Instant::now().duration_since(start);

        assert_eq!(result, 42);
        assert!(
            time <= elapsed,
            "bench time should be lower or equal than outer scope duration"
        );
    }
}
