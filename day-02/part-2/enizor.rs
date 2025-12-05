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

// if k1 | k2, then k2 repeats will already have been spotted by k1 so k2 should be skipped if possible
// => Only consider squarefree k as a non-squarefree number has a strict subset of only 1 squarefree number.
// Start by adding all k = primes.
// However by checking 2 disctinct primes k1 k2 we double check k3=k1×k2
// => instead of adding we remove for k3
// For 3 disctinct primes k1 k2 k3, we have k4=k1×k2×k3 added by k1,k2,k3 and removed by k1×k2, k1×k3, k2×k3
// => we should add k4
// By extension, for a divisor k of l
// - if k is not squarefree, skip it
// - else k is the product of n distinct primes
//     - if n is odd, add it
//     - else n is even, remove it
//
// Finally, by considering input fitting in u64, since 2^64 < 10^20 we only consider lengths <= 19 < 30
// (also length is the product of at most 2 distinct primes, so we could inline the 2 cases instead)

const MAX_LENGTH: usize = 19;
const SMALL_PRIMES: [usize; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
const RECIPE_LUT: [Recipe; MAX_LENGTH + 1] = make_lut();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Recipe {
    Skip,
    Add,
    Remove,
}

// LUT constructors
const fn recipe(n: usize) -> Recipe {
    let mut even_distinct_prime_factors = true;
    let mut i = 0;
    assert!(SMALL_PRIMES[SMALL_PRIMES.len() - 1] >= n);
    while i < SMALL_PRIMES.len() {
        let p = SMALL_PRIMES[i];
        if p > n {
            break;
        }
        if n % p == 0 {
            if (n / p) % p == 0 {
                return Recipe::Skip;
            }
            even_distinct_prime_factors = !even_distinct_prime_factors;
        }
        i += 1;
    }
    if even_distinct_prime_factors {
        Recipe::Remove
    } else {
        Recipe::Add
    }
}

const fn make_lut() -> [Recipe; MAX_LENGTH + 1] {
    let mut res = [Recipe::Skip; MAX_LENGTH + 1];
    let mut n = 2;
    while n <= MAX_LENGTH {
        res[n] = recipe(n);
        n += 1;
    }
    res
}

fn sum_invalid_ids(lhs: &[u8], rhs: &[u8]) -> usize {
    let mut l = lhs.len();
    assert!(l >= 1);
    let r = rhs.len();
    if r < l {
        return 0;
    }
    assert!(r <= MAX_LENGTH);
    let min = read_number(lhs) - 1;
    let max = read_number(rhs);
    if max < min + 1 {
        return 0;
    }
    assert_ne!(lhs[0], 0);
    assert_ne!(rhs[0], 0);
    let mut res = 0usize;
    if l == 1 {
        l += 1;
    }
    while l <= r {
        for (k, &recipe) in RECIPE_LUT.iter().enumerate() {
            if recipe == Recipe::Skip || l % k != 0 {
                continue;
            }
            let d = l / k;
            // Count the number of multiples of div=10.010..001 with k ones and l/k-1 zeroes between them in ]min, max=999...999 with l nines]
            let base_pow = 10usize.pow(d as u32);
            let mut div = 0;
            let mut pow = 1;
            for _ in 0..k {
                div += pow;
                pow *= base_pow;
            }
            // max multiplier below interval
            let min_div = if l == lhs.len() {
                min / div
            } else {
                base_pow / 10 - 1
            };
            // max multiplier in interval
            let max_div = if l == rhs.len() {
                max / div
            } else {
                base_pow - 1
            };
            // Now add/remove Σ_{k=min_div+1}^{k=max_div} k×div
            let local_sum = div * (min_div + 1 + max_div) * (max_div - min_div) / 2;
            if recipe == Recipe::Add {
                res = res.wrapping_add(local_sum);
            } else {
                res = res.wrapping_sub(local_sum);
            }
        }
        l += 1;
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
        'outer: for k in 1..=v.len() / 2 {
            if v.len() % k == 0 {
                for i in 0..k {
                    for q in 1..v.len() / k {
                        if v[v.len() - 1 - i] != v[v.len() - 1 - i - k * q] {
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
    fn test_helper(lhs: &[u8], rhs: &[u8]) {
        assert_eq!(naive(lhs, rhs), sum_invalid_ids(lhs, rhs));
    }

    #[test]
    fn run_test() {
        test_helper(b"74024", b"113072");
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
            4174379265
        )
    }
}
