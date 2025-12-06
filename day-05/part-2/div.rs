use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    ranges
        .split('\n')
        .map(|line| {
            let mut range_parts = line.split('-').into_iter();
            let left = range_parts.next().unwrap().parse::<usize>().unwrap();
            let right = range_parts.next().unwrap().parse::<usize>().unwrap();
            (left, right)
        })
        .collect()
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum BoundType {
    Start,
    End,
}
fn run(input: &str) -> isize {
    let input_ranges = parse_input(input);

    let mut values: Vec<(usize, BoundType)> = input_ranges
        .into_iter()
        .flat_map(|(left, right)| [(left, BoundType::Start), (right, BoundType::End)].into_iter())
        .collect();
    values.sort();

    let mut count = 0;
    let mut depth = 0;
    let mut start = 0;
    for (val, bound_type) in values {
        match bound_type {
            BoundType::Start => {
                if depth == 0 {
                    start = val;
                }
                depth += 1;
            }
            BoundType::End => {
                depth -= 1;
                assert!(depth >= 0);
                if depth == 0 {
                    count += val - start + 1;
                }
            }
        }
    }
    count.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example"), 0)
    }
}
