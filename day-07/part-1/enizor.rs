use std::env::args;
use std::time::Instant;

use aoc::enizor::bitset::{VecBitSet, bitset_size};

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let width = bytes
        .iter()
        .position(|&b| b == b'\n')
        .expect("did not find a LF")
        + 1;
    let lines = (bytes.len() + 1) / width;
    let start = bytes
        .iter()
        .position(|&b| b == b'S')
        .expect("did not find a start");
    let mut tachyons = VecBitSet::new(bitset_size(width));
    let mut new_tachyons = VecBitSet::new(bitset_size(width));
    tachyons.set(start % width);
    let mut res = 0;
    for line in ((start / width) + 1)..lines {
        for x in 0..width {
            if tachyons.test(x) {
                if bytes[x + line * width] == b'^' {
                    res += 1;
                    if x > 0 {
                        new_tachyons.set(x - 1);
                    }
                    if x < width {
                        new_tachyons.set(x + 1);
                    }
                } else {
                    new_tachyons.set(x);
                }
            }
        }
        tachyons.clear();
        std::mem::swap(&mut tachyons, &mut new_tachyons);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."),
            21
        )
    }
}
