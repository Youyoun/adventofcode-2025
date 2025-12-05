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
    while let Some(val) = parser.parse_usize() {
        debug_assert_eq!(parser.peek(), Some(&b'-'));
        parser.cur += 1;
        let max = parser
            .parse_usize()
            .unwrap_or_else(|| panic!("failed to parse near {}", parser.cur));
        ranges.push((val, max));
        parser.cur += 1;
    }
    ranges.sort_unstable();
    let mut res = 0;
    let (mut min, mut max) = ranges[0];
    for &(a, b) in &ranges[1..] {
        if a <= max {
            if b >= max {
                max = b;
            }
        } else {
            res += max - min + 1;
            min = a;
            max = b;
        }
    }
    res += max - min + 1;
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
            14
        )
    }
}
