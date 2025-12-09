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
    let mut points = vec![];
    while let Some(x) = parser.parse_usize() {
        parser.cur += 1;
        let y = parser
            .parse_usize()
            .unwrap_or_else(|| panic!("failed to parse near {}", parser.cur));
        parser.cur += 1;
        points.push((x, y));
    }
    let mut max_area = 0;
    for (i, &(x1, y1)) in points.iter().enumerate() {
        for &(x2, y2) in points.iter().skip(i + 1) {
            let dx = x2.abs_diff(x1)+1;
            let dy = y2.abs_diff(y1)+1;
            max_area = max_area.max(dx*dy);
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"),
            50
        )
    }
}
