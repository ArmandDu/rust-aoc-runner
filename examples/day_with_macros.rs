use itertools::Itertools;
use aoc::solution::SolutionError;

aoc::implement! {
    name: Day00;
    title: "addition or product";
    day: 0;
    input : "12345".to_owned();
    parse   -> Vec<u32> : |input: &str| input.chars().map(|c| c.to_digit(10).ok_or(SolutionError::ParseError)).collect();
    part_1  -> u32      : |input: &Self::Input| input.iter().sum1();
    part_2  -> u32      : |input: &Self::Input| input.iter().product1();
}

aoc::run!(Day00);


aoc::example! {
    [Day00]
    example: "1234" => Some(1+2+3+4) => Some(1*2*3*4)
    bigger: "123456789" => Some(1+2+3+4+5+6+7+8+9) => Some(1*2*3*4*5*6*7*8*9)
}