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
    let mut line_lengths = vec![];
    let mut lines_iter = bytes.iter();
    let mut last_line = 0;
    let mut max_line_len = 0;
    while let Some(pos) = lines_iter.position(|&b| b == b'\n')
        && pos != bytes.len() - 1
    {
        line_lengths.push(pos + 1);
        last_line += pos + 1;
        max_line_len = max_line_len.max(pos);
    }
    line_lengths.push(bytes.len() - line_lengths.last().unwrap_or(&0));
    let mut res = 0;
    let mut col = 0;
    let mut add = true;
    let mut mul_acc = 1;
    let mut add_acc = 0;
    let mut found_digit = false;
    while col < max_line_len {
        if !found_digit {
            if add {
                res += add_acc;
            } else {
                res += mul_acc;
            }
            mul_acc = 1;
            add_acc = 0;
            add = match bytes[col + last_line] {
                b'*' => false,
                b'+' => true,
                _ => panic!(
                    "failed to find a valid operation at {} {}",
                    col,
                    col + last_line
                ),
            };
        }
        found_digit = false;
        let mut line = 0;
        let mut cur = col;
        let mut val = 0;
        let mut eol = line_lengths[line];

        while cur < last_line {
            if cur < eol {
                let v = bytes[cur];
                if v.is_ascii_digit() {
                    val *= 10;
                    val += (v - b'0') as usize;
                    found_digit = true;
                }
            }
            cur += line_lengths[line];
            line += 1;
            eol += line_lengths[line];
        }
        col += 1;
        if found_digit {
            add_acc += val;
            mul_acc *= val;
        }
    }
    if add {
        res += add_acc;
    } else {
        res += mul_acc;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "),
            3263827
        )
    }
}
