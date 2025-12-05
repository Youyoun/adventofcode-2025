use std::collections::{HashMap, HashSet};
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_input(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.split(',').map(|range| {
        range.split_once('-').expect("could not split input")
    })
}

fn count_digits(x: i64) -> i64 {
    let mut limit = 10;
    let mut digits = 1;
    while x >= limit {
        digits += 1;
        limit *= 10;
    }
    digits
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Mask {
    value: i64,
    length: u32,
}

impl Mask {
    fn matches(&self, x: i64) -> bool {
        (x % 10_i64.pow(self.length)) * self.value == x
    }
}

fn generate_all_masks(max_digits: u32) -> HashMap<u32, HashSet<Mask>> {
    let mut result : HashMap<u32,HashSet<Mask>> = HashMap::new();

    for curr_pattern_len in 1..max_digits {
        // generate all masks for the given pattern length
        // for example:
        // curr_pattern_len=1 generates 11, 111, 1111, ..
        // curr_pattern_len=2 generates 101, 10101, 1010101, 101010101, etc..
        let mut curr_mask: i64 = 1;
        let mult: i64 = 10_i64.pow(curr_pattern_len);
        let mut current_digits = curr_pattern_len;
        loop {
            curr_mask = curr_mask * mult + 1;
            current_digits += curr_pattern_len;
            let mask = Mask{
                value: curr_mask,
                length: curr_pattern_len,
            };
            result.entry(current_digits)
            .or_default().insert(mask);
            if current_digits > max_digits {
                break;
            }
        }
    }
    result
}

fn run(input: &str) -> isize {
    let ranges = parse_input(input);
    let masks_by_digit = generate_all_masks(15);

    ranges.map(|(start, end)| {
        let start_num = start.parse::<i64>().unwrap();
        let end_num = end.parse::<i64>().unwrap();

        (start_num..end_num+1).map(|x| {
            let digits: u32= count_digits(x).try_into().unwrap();
            match masks_by_digit.get(&digits) {
                Some(masks) => {
                    if masks.into_iter().any(|m| { m.matches(x)}) {
                        x
                    } else {
                        0
                    }
                },
                None => 0,
            }
        }).sum::<i64>()
    }).sum::<i64>().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example"), 0)
    }

    #[test]
    fn count_digits_test() {
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(9), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(123), 3);
    }

    #[test]
    fn matches_test() {
        let mask = Mask {
            value: 101,
            length: 2,
        };
        assert!(mask.matches(1414));
    }

    #[test]
    fn generate_all_masks_test() {
        let masks_by_digits = generate_all_masks(6);

        let result = masks_by_digits.get(&2_u32).unwrap();
        let expected: HashSet<Mask> = vec![
            Mask{
                value: 11,
                length: 1,
            },
            ].into_iter().collect();
        assert_eq!(result, &expected);
    }
}