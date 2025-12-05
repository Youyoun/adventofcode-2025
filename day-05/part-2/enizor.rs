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

#[derive(Debug, Default, Clone)]
struct Ranges {
    ranges: Vec<(usize, usize)>,
    count: usize,
}

impl Ranges {
    fn insert(&mut self, min: usize, max: usize) {
        for &(a, b) in &self.ranges {
            if min >= a && min <= b {
                if max <= b {
                    return;
                } else {
                    self.insert(b + 1, max);
                    return;
                }
            }
            if max >= a && max <= b {
                if min >= a {
                    return;
                } else {
                    self.insert(min, a - 1);
                    return;
                }
            }
            if min < a && max > b {
                self.insert(min, a - 1);
                self.insert(b + 1, max);
                return;
            }
        }
        self.ranges.push((min, max));
        self.count += max - min + 1;
    }
    fn count(&self) -> usize {
        self.count
    }
}

fn run(input: &str) -> usize {
    let mut parser = Parser::from_input(&input);
    let mut ranges = Ranges::default();
    while let Some(val) = parser.parse_usize() {
        debug_assert_eq!(parser.peek(), Some(&b'-'));
        parser.cur += 1;
        let max = parser
            .parse_usize()
            .unwrap_or_else(|| panic!("failed to parse near {}", parser.cur));
        ranges.insert(val, max);
        parser.cur += 1;
    }
    ranges.count()
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
