use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_input(input: &str) -> impl Iterator<Item = i64> {
    input.lines().map(|entry| {
        let (direction, offset_str) = entry.split_at(1);
        let offset = offset_str.parse::<i64>().expect("can't parse entry");
        match direction {
            "R" => offset,
            "L" => -offset,
            _ => unreachable!("unknown direction"),
        }
    })
}

fn run(input: &str) -> isize {
    let mut result: i64 = 0;
    let offsets = parse_input(input);
    let mut curr_pos: i64 = 50;
    for offset in offsets {
        let next_pos = curr_pos + offset;
        let mut zero_visits = (next_pos / 100).abs();
        if next_pos <= 0 && curr_pos > 0 {
            zero_visits += 1;
        }
        result += zero_visits;
        curr_pos = next_pos % 100;
        if curr_pos < 0 {
            curr_pos += 100;
        }
    }
    result as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example"), 0)
    }
}
