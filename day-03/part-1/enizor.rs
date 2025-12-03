use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    input
        .as_bytes()
        .split(|b| !b.is_ascii_digit())
        .map(solve_bank)
        .sum()
}

fn solve_bank(bytes: &[u8]) -> usize {
    if bytes.len() < 2 {
        return 0;
    }
    let mut cur = 0;
    let mut tens = 0;
    let mut units = 0;
    let mut tens_cur = 0;
    while cur < bytes.len() - 1 {
        let b = (bytes[cur] - b'0') as usize;
        if b > tens {
            tens = b;
            tens_cur = cur;
            if tens == 9 {
                break;
            }
        }
        cur += 1;
    }
    cur = tens_cur + 1;
    while cur < bytes.len() {
        let b = (bytes[cur] - b'0') as usize;
        if b > units {
            units = b;
            if units == 9 {
                break;
            }
        }
        cur += 1;
    }
    tens * 10 + units
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve_bank(b"987654321111111"), 98);
        assert_eq!(solve_bank(b"811111111111119"), 89);
        assert_eq!(solve_bank(b"234234234234278"), 78);
        assert_eq!(solve_bank(b"818181911112111"), 92);
        assert_eq!(solve_bank(b"717171811112111"), 82);
    }
}
