use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> isize {
    let mut count = 0;
    let mut dial_pos = 50isize;
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        while i < bytes.len() && bytes[i] != b'L' && bytes[i] != b'R' {
            i += 1;
        };
        if i >= bytes.len() {
            break;
        }
        let add = bytes[i] == b'R';
        let mut tens = 0;
        let mut units = 0;
        i += 1;
        while i < bytes.len() {
            match bytes[i] {
                digit @ b'0'..=b'9' => {
                    count += tens;
                    tens = units;
                    units = (digit - b'0') as isize;
                }
                _ => break,
            }
            i += 1;
        }
        let num = 10 * tens + units;
        if add {
            dial_pos += num;
            if dial_pos >= 100 {
                count += 1;
                dial_pos -= 100;
            }
        } else if num > 0 {
            if dial_pos > num {
                dial_pos -= num;
            } else if dial_pos < num {
                if dial_pos > 0 {
                    count += 1;
                }
                dial_pos += 100 - num
            } else {
                dial_pos = 0;
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("L68 L30 R48 L5 R60 L55 L1 L99 R14L82"), 6)
    }
    #[test]
    fn run_test2() {
        assert_eq!(run("L50 R50 R50 R50 L50 L50 R50 L50 R54"), 5)
    }
    #[test]
    fn run_test3() {
        assert_eq!(run("L50 L100 L200 L200 R200 R300"), 11)
    }
}
