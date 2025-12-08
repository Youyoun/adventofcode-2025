use std::env::args;
use std::time::Instant;

use aoc::enizor::bitset::{VecBitSet, bitset_size};
use aoc::enizor::parser::Parser;
use aoc::enizor::utils::get_2_mut;

fn main() {
    let now = Instant::now();
    let output = run::<1000>(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run<const N: usize>(input: &str) -> usize {
    let mut parser = Parser::from_input(&input);
    let mut points = vec![];
    while let Some(x) = parser.parse_usize() {
        parser.cur += 1;
        let y = parser
            .parse_usize()
            .unwrap_or_else(|| panic!("failed to parse near {}", parser.cur));
        parser.cur += 1;
        let z = parser
            .parse_usize()
            .unwrap_or_else(|| panic!("failed to parse near {}", parser.cur));
        points.push((x, y, z));
        parser.cur += 1;
    }

    let mut dists = Vec::with_capacity(points.len() * points.len());
    for (i, &(x1, y1, z1)) in points.iter().enumerate() {
        for (j, &(x2, y2, z2)) in points.iter().enumerate().skip(i + 1) {
            let dx = x2.abs_diff(x1);
            let dy = y2.abs_diff(y1);
            let dz = z2.abs_diff(z1);
            dists.push((dx * dx + dy * dy + dz * dz, i, j));
        }
    }
    dists.sort_unstable();
    let mut connex_comp: Vec<(VecBitSet, usize)> = vec![];
    let mut to_merge = Vec::with_capacity(2);
    'outer: for &(_, i, j) in &dists[0..N] {
        to_merge.clear();
        let mut add_i = false;
        for (m, c) in connex_comp.iter().enumerate() {
            let l = c.0.test(i);
            let r = c.0.test(j);
            if l && r {
                continue 'outer;
            }
            if l || r {
                add_i = r;
                to_merge.push(m);
                if to_merge.len() == 2 {
                    break;
                }
            }
        }
        match to_merge.len() {
            0 => {
                let mut new_comp = VecBitSet::new(bitset_size(points.len()));
                new_comp.set(i);
                new_comp.set(j);
                connex_comp.push((new_comp, 2));
            }
            1 => {
                let chg = if add_i { i } else { j };
                connex_comp[to_merge[0]].0.set(chg);
                connex_comp[to_merge[0]].1 += 1;
            }
            2 => {
                let (m0, m1) = get_2_mut(&mut connex_comp, to_merge[0], to_merge[1]);
                m0.0 |= &m1.0;
                m0.1 += &m1.1;
                connex_comp.remove(to_merge[1]);
            }
            _ => unreachable!(),
        }
    }
    let (max_1, max_2, max_3) = connex_comp
        .iter()
        .fold((1, 1, 1), |(m1, m2, m3), (_comp, s)| {
            if *s > m1 {
                (*s, m1, m2)
            } else if *s > m2 {
                (m1, *s, m2)
            } else if *s > m3 {
                (m1, m2, *s)
            } else {
                (m1, m2, m3)
            }
        });
    max_1 * max_2 * max_3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_enizor_day8() {
        assert_eq!(
            run::<10>(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            ),
            40
        )
    }
}
