//! AoC 2021 - 17

use std::env;

fn parse_file(s: &str) -> ((i64, i64), (i64, i64)) {
    let mut l = s.split(", y=");
    let first = l.next().unwrap();
    let second = l.next().unwrap();
    //"target area: x=20..30, y=-10..-5"
    let mut x_iter = first[15..].split("..");
    let xmin = x_iter.next().unwrap().parse::<i64>().unwrap();
    let xmax = x_iter.next().unwrap().parse::<i64>().unwrap();

    let mut y_iter = second.split("..");

    let ymin = y_iter.next().unwrap().parse::<i64>().unwrap();
    let ymax = y_iter
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    ((xmin, xmax), (ymin, ymax))
}

fn sim(range: &((i64, i64), (i64, i64)), vel: &(i64, i64)) -> (bool, i64) {
    let mut vel = *vel;
    let ((tgt_x_min, tgt_x_max), (tgt_y_min, tgt_y_max)) = *range;

    let mut max_height = 0;

    let mut cur = (0, 0);
    while cur.0 <= tgt_x_max && !(vel.0 == 0 && (cur.1 < tgt_y_min)) {
        max_height = max_height.max(cur.1);
        if cur.0 >= tgt_x_min && cur.0 <= tgt_x_max && cur.1 >= tgt_y_min && cur.1 <= tgt_y_max {
            return (true, max_height);
        }
        cur = (cur.0 + vel.0, cur.1 + vel.1);
        vel.0 -= vel.0.signum();
        vel.1 -= 1;
    }

    (false, 0)
}

fn p1(range: &((i64, i64), (i64, i64))) -> ((i64, i64), i64) {
    let (mut vx, mut vy) = (0, 0);
    let mut max_height = i64::MIN;
    for vel_y in -200..=200 {
        for vel_x in 1..=200 {
            let v = (vel_x, vel_y);
            let (hit, max) = sim(range, &v);
            if hit {
                if max > max_height {
                    vx = v.0;
                    vy = v.1;
                    max_height = max;
                }
            }
        }
    }

    ((vx, vy), max_height)
}

fn p2(range: &((i64, i64), (i64, i64))) -> usize {
    let mut count = 0;
    let max_y = range.1 .0.abs().max(range.1 .1.abs());
    let max_x = range.0 .0.abs().max(range.0 .1.abs());
    for vel_y in -max_y..=max_y {
        for vel_x in 1..=max_x {
            let v = (vel_x, vel_y);
            let (hit, _max) = sim(range, &v);
            if hit {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let range = parse_file(&file_str);
    println!("{:?}", p1(&range));
    println!("{:?}", p2(&range));
}

#[cfg(test)]
mod test_day16 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"target area: x=20..30, y=-10..-5"#;
        let range = parse_file(&example);
        assert!(sim(&range, &(7, 2)).0);
        assert!(!sim(&range, &(17, 4)).0);
        assert_eq!(sim(&range, &(6, 9)).1, 45);
        assert_eq!(p2(&range), 112);
    }
}
