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
    while cur < bytes.len() {
        match bytes[cur] {
            b'0'..=b'9' => {}
            b'-' => hyphen = cur,
            b',' => {
                res += sum_invalid_ids(
                    &bytes[comma.wrapping_add(1)..hyphen],
                    &bytes[hyphen + 1..cur],
                );
                comma = cur;
            }
            _ => break, // consider any other char as EOF
        }
        cur += 1;
    }
    res += sum_invalid_ids(
        &bytes[comma.wrapping_add(1)..hyphen],
        &bytes[hyphen + 1..cur],
    );
    res
}

fn sum_invalid_ids(lhs: &[u8], rhs: &[u8]) -> usize {
    let min = read_number(lhs) - 1;
    let max = read_number(rhs);
    if max < min + 1 {
        return 0;
    }
    assert_ne!(lhs[0], 0);
    assert_ne!(rhs[0], 0);
    let mut res = 0;
    let mut l = lhs.len();
    if l % 2 != 0 {
        l += 1;
    }
    let mut low_pow = 10usize.pow((l / 2 - 1) as u32);
    let mut high_pow = 10 * low_pow;
    while l <= rhs.len() {
        // Count the number of multiples of div=10...001 with l/2-1 zeroes in ]99..999 with l-1 nines, max=999...999 with l nines]
        let div = high_pow + 1;
        // max multiplier below interval
        let min_div = if l == lhs.len() {
            min / div
        } else {
            low_pow - 1
        };
        // max multiplier in interval
        let max_div = if l == rhs.len() {
            max / div
        } else {
            high_pow - 1
        };
        // Now add Σ_{k=min_div+1}^{k=max_div} k×div
        res += div * (min_div + 1 + max_div) * (max_div - min_div) / 2;
        l += 2;
        low_pow = high_pow;
        high_pow *= 10;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn is_invalid(mut x: usize) -> bool {
        let mut v = vec![];
        while x > 0 {
            v.push(x % 10);
            x /= 10;
        }
        if v.len() % 2 != 0 {
            return false;
        } else {
            for i in 0..v.len() / 2 {
                if v[i] != v[v.len() / 2 + i] {
                    return false;
                }
            }
            true
        }
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
    fn test_helper(lhs: &[u8], rhs: &[u8]) {
        assert_eq!(naive(lhs, rhs), sum_invalid_ids(lhs, rhs));
    }

    #[test]
    fn run_test() {
        test_helper(b"12", b"22");
        test_helper(b"11", b"22");
        test_helper(b"11", b"99");
        test_helper(b"1", b"990");
        test_helper(b"44", b"55");
        test_helper(b"55", b"55");
        test_helper(b"55", b"56");
        test_helper(b"54", b"55");
        test_helper(b"54", b"55");
        test_helper(b"54", b"55");
        test_helper(b"54", b"55");
        test_helper(b"95", b"115");
        test_helper(b"998", b"1012");
        test_helper(b"1188511880", b"1188511890");
        test_helper(b"222220", b"222224");
        test_helper(b"1698522", b"1698528");
        test_helper(b"446443", b"446449");
        test_helper(b"38593856", b"38593862");
        test_helper(b"565653", b"565659");
        test_helper(b"824824821", b"824824827");
        test_helper(b"2121212118", b"2121212124");
        assert_eq!(
            run(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        )
    }
}
