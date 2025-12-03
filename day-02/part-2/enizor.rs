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
    let bytes = input.as_bytes();
    let mut cur = 0;
    let mut comma = usize::MAX;
    let mut hyphen = 0;
    let mut res = 0;
    let mut ended = false;
    while cur < bytes.len() {
        match bytes[cur] {
            b'0'..=b'9' => {}
            b'-' => hyphen = cur,
            b',' => {
                res += naive(
                    &bytes[comma.wrapping_add(1)..hyphen],
                    &bytes[hyphen + 1..cur],
                );
                comma = cur;
            }
            _ => {
                res += naive(
                    &bytes[comma.wrapping_add(1)..hyphen],
                    &bytes[hyphen + 1..cur],
                );
                ended = true;
                break;
            } // consider any other char as EOF
        }
        cur += 1;
    }
    if !ended {
        res += naive(
            &bytes[comma.wrapping_add(1)..hyphen],
            &bytes[hyphen + 1..cur],
        );
    }
    res
}

fn read_number(input: &[u8]) -> usize {
    let mut res = 0;
    for &b in input {
        res *= 10;
        res += (b - b'0') as usize
    }
    res
}

fn is_invalid(mut x: usize) -> bool {
    let mut v = vec![];
    while x > 0 {
        v.push(x % 10);
        x /= 10;
    }
    'outer: for k in 1..=v.len() / 2 {
        if v.len() % k == 0 {
            for i in 0..k {
                for q in 1..v.len() / k {
                    if v[v.len()-1 -i] != v[v.len()-1 -i - k * q] {
                        continue 'outer;
                    }
                }
            }
            return true;
        }
    }
    false
}
fn naive(lhs: &[u8], rhs: &[u8]) -> usize {
    let min = read_number(lhs);
    let max = read_number(rhs);
    let mut res = 0;
    for i in min..=max {
        if is_invalid(i) {
            res += i;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(naive(b"11", b"22"), 33);
        assert_eq!(naive(b"11", b"99"), 495);
        assert_eq!(
            naive(b"1", b"990"),
            495 + 111 + 222 + 333 + 444 + 555 + 666 + 777 + 888
        );
        assert_eq!(naive(b"44", b"55"), 99);
        assert_eq!(naive(b"55", b"55"), 55);
        assert_eq!(naive(b"55", b"56"), 55);
        assert_eq!(naive(b"54", b"55"), 55);
        assert_eq!(naive(b"54", b"55"), 55);
        assert_eq!(naive(b"54", b"55"), 55);
        assert_eq!(naive(b"54", b"55"), 55);
        assert_eq!(naive(b"95", b"115"), 210);
        assert_eq!(naive(b"998", b"1012"), 999 + 1010);
        assert_eq!(naive(b"1188511880", b"1188511890"), 1188511885);
        assert_eq!(naive(b"222220", b"222224"), 222222);
        assert_eq!(naive(b"1698522", b"1698528"), 0);
        assert_eq!(naive(b"446443", b"446449"), 446446);
        assert_eq!(naive(b"38593856", b"38593862"), 38593859);
        assert_eq!(naive(b"565653", b"565659"), 565656);
        assert_eq!(naive(b"824824821", b"824824827"), 824824824);
        assert_eq!(naive(b"2121212118", b"2121212124"), 2121212121);
        assert_eq!(
            run(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            4174379265
        )
    }
}
