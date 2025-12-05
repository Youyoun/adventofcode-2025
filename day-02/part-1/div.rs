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

fn find_next_higher_prefix(x: &str) -> i64 {
    let digits = x.len();
    let num = x.parse::<i64>().expect("could not parse integer");

    if (digits % 2) == 0 {
        // half contains the first half digits
        let half = x[0..(digits/2)].parse::<i64>().unwrap();
        if num <= half * 10_i64.pow((digits as u32)/2) + half {
            half
        } else {
            half+1
        }
    } else {
        10_i64.pow((digits as u32)/2)
    }
}

fn find_next_lower_prefix(x: &str) -> i64 {
    let digits = x.len();
    let num = x.parse::<i64>().expect("could not parse integer");

    if (digits % 2) == 0 {
        // half contains the first half digits
        let half = x[0..(digits/2)].parse::<i64>().unwrap();
        if num >= half * 10_i64.pow((digits as u32)/2) + half {
            half
        } else {
            half-1
        }
    } else {
        10_i64.pow((digits as u32)/2)-1
    }
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

fn run(input: &str) -> isize {
    let ranges = parse_input(input);
    ranges.map(|(start, end)| {
        let left = find_next_higher_prefix(start);
        let right = find_next_lower_prefix(end);

        if right < left {
            0
        } else {
            (left..right+1).map(| x| {
                let digits = count_digits(x);
                x * 10_i64.pow(digits as u32) + x
            }).sum()
        }
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
    fn find_next_higher_prefix_test() {
        assert_eq!(find_next_higher_prefix("1"), 1);
        assert_eq!(find_next_higher_prefix("9"), 1);
        assert_eq!(find_next_higher_prefix("10"), 1);
        assert_eq!(find_next_higher_prefix("11"), 1);
        assert_eq!(find_next_higher_prefix("12"), 2);

        assert_eq!(find_next_higher_prefix("156"), 10);

        assert_eq!(find_next_higher_prefix("1567"), 16);
        assert_eq!(find_next_higher_prefix("1616"), 16);
        assert_eq!(find_next_higher_prefix("1617"), 17);
        
        assert_eq!(find_next_higher_prefix("12345"), 100);
    }

    #[test]
    fn find_next_lower_prefix_test() {
        assert_eq!(find_next_lower_prefix("1"), 0);
        assert_eq!(find_next_lower_prefix("9"), 0);
        assert_eq!(find_next_lower_prefix("10"), 0);
        assert_eq!(find_next_lower_prefix("11"), 1);
        assert_eq!(find_next_lower_prefix("12"), 1);

        assert_eq!(find_next_lower_prefix("156"), 9);

        assert_eq!(find_next_lower_prefix("1567"), 15);
        assert_eq!(find_next_lower_prefix("1716"), 16);
        assert_eq!(find_next_lower_prefix("1717"), 17);
        
        assert_eq!(find_next_lower_prefix("12345"), 99);
    }
}
