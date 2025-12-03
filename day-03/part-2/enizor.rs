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

const NB_BATTERIES: usize = 12;

fn solve_bank(bytes: &[u8]) -> usize {
    if bytes.len() < 2 {
        return 0;
    }
    let mut battery_cur = usize::MAX;
    let mut res = 0;
    for battery in 1..=NB_BATTERIES {
        let mut cur = battery_cur.wrapping_add(1);
        let mut battery_value = 0;
        while cur < bytes.len() - NB_BATTERIES + battery {
            let b = (bytes[cur] - b'0') as usize;
            if b > battery_value {
                battery_value = b;
                battery_cur = cur;
                if battery_value == 9 {
                    break;
                }
            }
            cur += 1;
        }
        res *= 10;
        res += battery_value;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve_bank(b"987654321111111"), 987654321111);
        assert_eq!(solve_bank(b"811111111111119"), 811111111119);
        assert_eq!(solve_bank(b"234234234234278"), 434234234278);
        assert_eq!(solve_bank(b"818181911112111"), 888911112111);
        assert_eq!(solve_bank(b"717171811112111"), 777811112111);
    }
}
