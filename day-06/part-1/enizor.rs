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

enum Operation {
    Add,
    Mul(usize),
}

fn run(input: &str) -> usize {
    let mut parser = Parser::from_input(&input);
    // find last line
    assert!(parser.find_str("\n"));
    let line_len = parser.cur;
    let nb_lines = (parser.bytes.len() + (line_len/8)) / line_len;
    let mut ops = vec![];
    parser.cur = (nb_lines - 1) * line_len -(line_len/8);
    assert!(parser.find_str("\n"));
    parser.skip_whitespace();
    while !parser.eof() {
        ops.push({
            match parser.peek() {
                Some(b'+') => Operation::Add,
                Some(b'*') => Operation::Mul(1),
                _ => panic!("Failed to parse input"),
            }
        });
        parser.cur += 1;
        parser.skip_whitespace();
    }
    parser.cur = 0;
    parser.skip_whitespace();
    let mut col = 0;
    let mut res = 0;
    while let Some(val) = parser.parse_usize() {
        match &mut ops[col] {
            Operation::Add => res += val,
            Operation::Mul(x) => *x *= val,
        }
        col += 1;
        if col == ops.len() {
            col = 0;
        }
        parser.skip_whitespace();
    }
    for op in ops {
        match op {
            Operation::Mul(x) => res += x,
            Operation::Add => {}
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "),
            4277556
        )
    }
}
