use std::env::args;
use std::time::Instant;

use aoc::enizor::parser::Parser;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut parser = Parser::from_input(&input);
    let mut ranges = vec![];
    let mut res = 0;
    while let Some(val) = parser.parse_usize() {
        debug_assert_eq!(parser.peek(), Some(&b'-'));
        parser.cur += 1;
        let max = parser
            .parse_usize()
            .unwrap_or_else(|| panic!("failed to parse near {}", parser.cur));
        ranges.push((val, max));
        parser.cur += 1;
    }
    parser.cur += 1;
    while let Some(val) = parser.parse_usize() {
        for &(a, b) in &ranges {
            if a <= val && val <= b {
                res += 1;
                break;
            }
        }
        parser.cur += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("3-5
10-14
16-20
12-18

1
5
8
11
17
32"),
            3
        )
    }
}
