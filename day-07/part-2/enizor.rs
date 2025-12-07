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
    let mut tachyons = vec![0; width];
    let mut new_tachyons = vec![0; width];
    tachyons[start % width] = 1;
    for line in ((start / width) + 1)..lines {
        for x in 0..width {
            if tachyons[x] > 0 {
                if bytes[x + line * width] == b'^' {
                    if x > 0 {
                        new_tachyons[x - 1] += tachyons[x]
                    }
                    if x < width {
                        new_tachyons[x + 1] += tachyons[x]
                    }
                } else {
                    new_tachyons[x] += tachyons[x];
                }
                tachyons[x] = 0;
            }
        }
        std::mem::swap(&mut tachyons, &mut new_tachyons);
    }
    tachyons.iter().sum()
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
            40
        )
    }
}
