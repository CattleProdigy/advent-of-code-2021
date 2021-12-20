//! AoC 2021 - 19

use na::Point3;
use na::Vector3;
use nalgebra as na;
use petgraph::graphmap::UnGraphMap;
use std::env;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DIR_PERMS: Vec<na::Matrix3<i64>> = perms();
}

fn perms() -> Vec<na::Matrix3<i64>> {
    let dirs: [Vector3<i64>; 6] = [
        Vector3::<i64>::x(),
        Vector3::<i64>::y(),
        Vector3::<i64>::z(),
        -Vector3::<i64>::x(),
        -Vector3::<i64>::y(),
        -Vector3::<i64>::z(),
    ];
    let mut perms = vec![];
    for x in dirs.iter() {
        for y in dirs.iter() {
            if *y == *x || *y == -*x {
                continue;
            }
            for z in dirs.iter() {
                if *z == *x || *z == -*x {
                    continue;
                }
                if *z == *y || *z == -*y {
                    continue;
                }
                let rot = na::Matrix3::<i64>::from_columns(&[*x, *y, *z]);
                let float = rot.cast::<f32>();
                let det = float.determinant();
                if det < 0.0 {
                    continue;
                }

                perms.push(rot)
            }
        }
    }
    perms
}

fn parse(s: &str) -> Vec<Vec<Point3<i64>>> {
    s.split("\n\n")
        .map(|b| {
            let mut line_iter = b.lines();
            let first = line_iter.next().unwrap();
            let _scanner_idx = {
                let mut slice = &first[12..];
                let space_idx = slice.find(' ').unwrap();
                slice = &slice[..space_idx];
                slice.parse::<i64>().unwrap()
            };

            line_iter
                .map(|l| {
                    let mut comma_iter = l.split(",");
                    let x = comma_iter.next().unwrap().parse::<i64>().unwrap();
                    let y = comma_iter.next().unwrap().parse::<i64>().unwrap();
                    let z = comma_iter.next().unwrap().parse::<i64>().unwrap();
                    Point3::<i64>::new(x, y, z)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn attempt_pair_alignment(
    p1: &[Point3<i64>],
    p2: &[Point3<i64>],
) -> Option<(na::Matrix3<i64>, na::Vector3<i64>)> {
    for p1_from_p2 in DIR_PERMS.iter() {
        for p1_pt in p1.iter() {
            for p2_pt in p2.iter() {
                let p2_pt_in_p1 = p1_from_p2 * p2_pt;
                let translation = p1_pt - p2_pt_in_p1;

                let mut count = 0;
                let mut matches = vec![];
                for p2_pt_eval in p2.iter() {
                    let p2_pt_tranformed = p1_from_p2 * p2_pt_eval + translation;
                    if p1.iter().find(|p| *p.coords == *p2_pt_tranformed).is_some() {
                        matches.push(p2_pt_tranformed);
                        count += 1;
                    }
                    if count >= 12 {
                        return Some((p1_from_p2.clone(), translation));
                    }
                }
            }
        }
    }
    None
}

fn p1_p2(scanners: &Vec<Vec<Point3<i64>>>) {
    let mut mappings = Vec::<(usize, usize, na::Matrix3<i64>, na::Vector3<i64>)>::new();
    for (i, scan_i) in scanners.iter().enumerate() {
        for (j, scan_j) in scanners.iter().enumerate().skip(i + 1) {
            // transform maps j to i, so tranform is valid if j > i
            // aka i_from_j
            if let Some((r, t)) = attempt_pair_alignment(scan_i, scan_j) {
                mappings.push((i, j, r, t));
            }
        }
    }
    let mut graph = UnGraphMap::<usize, (na::Matrix3<i64>, na::Vector3<i64>)>::new();
    for (i, j, r, t) in mappings.iter().copied() {
        graph.add_node(i);
        graph.add_node(j);
        graph.add_edge(i, j, (r, t));
    }
    let mut pts_in_zero_frame = std::collections::HashSet::<na::Point3<i64>>::new();
    let max_scanner = scanners.len();
    for i in 1..max_scanner {
        let paths =
            petgraph::algo::simple_paths::all_simple_paths::<Vec<_>, _>(&graph, i, 0, 0, None)
                .collect::<Vec<_>>();
        let path = &paths[0];
        let mut rot = na::Matrix3::<i64>::identity();
        let mut trans = na::Vector3::<i64>::zeros();
        for from_to in path.windows(2) {
            let from = from_to[0];
            let to = from_to[1];
            let (mut edge_rot, mut edge_trans) = graph.edge_weight(from, to).unwrap().clone();
            if from < to {
                edge_rot = edge_rot.transpose();
                edge_trans = -edge_rot * edge_trans;
            }
            // compose
            let new_rot = edge_rot * rot;
            let new_trans = edge_rot * trans + edge_trans;
            rot = new_rot;
            trans = new_trans;
        }
        for pt in scanners[i].iter() {
            let transformed = rot * pt + trans;
            pts_in_zero_frame.insert(transformed);
        }
    }
    for pt in scanners[0].iter() {
        pts_in_zero_frame.insert(*pt);
    }
    eprintln!("p1: {}", pts_in_zero_frame.len());

    // p2
    let mut max_manhatten = 0;
    for i in 0..max_scanner {
        for j in (i + 1)..max_scanner {
            let paths =
                petgraph::algo::simple_paths::all_simple_paths::<Vec<_>, _>(&graph, i, j, 0, None)
                    .collect::<Vec<_>>();
            let path = &paths[0];
            let mut rot = na::Matrix3::<i64>::identity();
            let mut trans = na::Vector3::<i64>::zeros();
            for from_to in path.windows(2) {
                let from = from_to[0];
                let to = from_to[1];
                let (mut edge_rot, mut edge_trans) = graph.edge_weight(from, to).unwrap().clone();
                if from < to {
                    edge_rot = edge_rot.transpose();
                    edge_trans = -edge_rot * edge_trans;
                }
                // compose
                let new_rot = edge_rot * rot;
                let new_trans = edge_rot * trans + edge_trans;
                rot = new_rot;
                trans = new_trans;
            }
            let manhatten = trans.abs().sum();
            max_manhatten = max_manhatten.max(manhatten);
        }
    }
    eprintln!("p2: {}", max_manhatten);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let scanners = parse(&file_str);
    p1_p2(&scanners);
}

#[cfg(test)]
mod test_day19 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;

        let scanners = parse(&example);

        {
            let scan0 = &scanners[0];
            eprintln!("scan0: {:?}", scan0);
            let scan1 = &scanners[1];
            eprintln!("scan1: {:?}", scan1);
            p1_p2(&scanners);
        }
    }
}
