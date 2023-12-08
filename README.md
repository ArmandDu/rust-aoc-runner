# Advent Of Code Runner

## Motivation
When I started learning Rust, I wanted to get some practice.
Then I remembered about [Advent of Code](https://adventofcode.com/) (AOC) challenges which gave me plenty of practice.

After a while, I learned more about Rust and struggled with a few bottlenecks:
- I grew tired of each day, copying the previous day's boilerplate
- When I wanted to improve formatting or structure, I would have to edit each day manually
- Learning with Puzzle was fun but limited.

So I decided to go on a side project and work on tooling around AOC for rust.
This let me work on more than only the daily puzzles and solved other bottlenecks that I had with solving said puzzles!

## Description

This library is a collection of traits and macros that aim to solve the bottlenecks that I encountered while doing Advent of Code puzzles.

This library, mainly provide a trait `aoc::Solution` handle the boilerplate around the puzzle solving:
- reading the input file
- calling the internal parsing, solving and displaying of the day's solution
- logging information I care about, such execution time
- Providing some safety tools such as Error handling.
- Providing a way to test the solutions against an example input

The trait handles all we need for a day's puzzle. But I also added some `macros` to further reduce boilerplate:

- `solution!` that handles calling Solution::run and prints out the result
- `test!` that offers an interface for testing each part easily
- `test_common!` to add common tests across each day.


## Usage

### 1. Add the AOC runner in your dependencies

 ```toml
 [dependencies]
 aoc-runner = { git = "https://github.com/ArmandDu/rust-aoc-runner.git" }
 concat-idents = "1.1.3" #peer dep but needed here until I find a fix for it.
 ```
<b>Warning</b>: [contact-idents](https://docs.rs/concat-idents/1.1.5/concat_idents/index.html) is used in some macros and is needed as peer dependency.

### 2. In a .rs file, create a new `struct` that implements `aoc::Solution`
 ````rust
 use aoc::solution::{Result, Solution};
 
 struct Day01;
 
 impl Solution for Day01 {
     const TITLE: &'static str = "An example";
     const DAY: u8 = 1;
     type Input = ();//return type of parse
     type P1 = (); //return type of part1
     type P2 = (); //return type of part2
 
     fn parse(input: &str) -> Result<Self::Input> {
         // parse input to your liking here
         todo!()
     }
 
     fn part1(input: &Self::Input) -> Option<Self::P1> {
         //solution for part 1 here
         todo!()
     }
 
     fn part2(input: &Self::Input) -> Option<Self::P2> {
         // solution for part 2 here
         todo!()
     }
 }
 ````
### 3. Inside your main function, call in `aoc::solution!` macro
 ```rust
 fn main() {
     aoc::solution!(Day01);
 }
 ```
### 4. create an inputs folder and input file for that day

By default, the runner will look for files under `"inputs/DAY_{DAY:02}.txt"`.
Where {DAY:02} means `const DAY:u8` attribute twice padded.

```
 0 => 00
 1 => 01
...
10 => 10
11 => 11
...
```

For example, to get the input for day 01. It is possible to do something like:
```shell
mkdir inputs
echo "puzzle input" > inputs/DAY_01.txt
```

Alternatively, if using curl, and knowing how to get the Advent of Code's Session cookie,
it is possible to do:
```shell
curl https://adventofcode.com/2015/day/2/input --cookie "session=<my_session>"
```
### 5. run `cargo run`

If all went well, it'll run the main function, run the day's solution and print the result!

## Testing
To facilitate testing, I have included a `test!` macro. 
```rust
#[cfg(test)]
mod tests {
    //caveat: it requires the struct to be snake_case
    use crate::Day01 as day_01;
    use crate::*;
    
    aoc::test! {
        day_01:
        [hello]
        - "Hello"
            => Some("hello".to_owned())
            => Some("HELLO".to_owned());
        [special_chars]
        - "/*-+"
            => Some("/*-+".to_owned())
            => Some("/*-+".to_owned());
    }
}
```

The macro reads as follows:

```
test for $day: 
Test Case [$case]
- $input => expected_part1 => $expected_part2
```

It uses the `$day` to name the test cases and to call its methods.

`[$case]` is also used to name the test case. It's optional but needed when using more than 1 example.
It's part of a function name and needs to match function naming conventions (alphanumeric, no space, no special characters)

`- $input` is the string literal that will be used as input for the solution. 

`=> $expected_part1` is the expected return value of the part 1 for `$input`

`=> $expected_part2` is the expected return value of the part 2 for `$input`
