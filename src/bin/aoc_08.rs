//! AoC 2021 - 08

use std::env;

fn parse_file(s: &str) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    let inputs_outputs = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut delim = l.split('|');
            let inputs = delim.next().unwrap();
            let outputs = delim.next().unwrap();

            let input_strs = inputs
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let output_strs = outputs
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            (input_strs, output_strs)
        })
        .collect::<Vec<_>>();

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    for (vi, vo) in inputs_outputs.into_iter() {
        inputs.push(vi);
        outputs.push(vo);
    }

    (inputs, outputs)
}

fn p1(outputs: &[Vec<&str>]) -> usize {
    outputs
        .into_iter()
        .map(|s| s)
        .flatten()
        .filter(|s| match s.len() {
            2..=4 => true, // 1 7,4
            7 => true,     // 8
            _ => false,
        })
        .count()
}

// cur_char should not be locked (i.e. it should have more than one possibility)
fn p2_rec(
    possibs: HashMap<char, HashSet<char>>,
    cur_char: char,
    strs: &[&str],
) -> Option<HashMap<char, char>> {
    eprintln!("========");
    eprintln!("{}:", cur_char);
    eprintln!("\t{}: {:?}:", 'a', possibs[&'a']);
    eprintln!("\t{}: {:?}:", 'b', possibs[&'b']);
    eprintln!("\t{}: {:?}:", 'c', possibs[&'c']);
    eprintln!("\t{}: {:?}:", 'd', possibs[&'d']);
    eprintln!("\t{}: {:?}:", 'e', possibs[&'e']);
    eprintln!("\t{}: {:?}:", 'f', possibs[&'f']);
    eprintln!("\t{}: {:?}:", 'g', possibs[&'g']);
    eprintln!("========");

    let cur_char_pos = &possibs[&cur_char];
    assert!(cur_char_pos.len() > 1);

    for p in cur_char_pos.iter() {
        // in each iteration, assign the current character one of it's possibilities
        let mut new_pos = possibs.clone();

        // remove that assigned values from others
        for (_np_k, np_v) in new_pos.iter_mut() {
            np_v.remove(p);
        }

        // the current characters possibilities are locked to this one assigned value
        *new_pos.get_mut(&cur_char).unwrap() = [*p].iter().copied().collect();

        // if this assignment means something else has no possibilities then
        // the assigment is unsatisfiable so skip it
        if new_pos.iter().any(|(_k, v)| v.is_empty()) {
            continue;
        }

        // select the next character to try, we choose the one with the fewest
        // possibilities (but more than one)
        let mut counts = new_pos
            .iter()
            .map(|(&k, v)| (k, v.len()))
            .collect::<Vec<_>>();
        counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        if let Some((chr, _count)) = counts.iter().find(|(_chr, count)| *count > 1) {
            if let Some(res) = p2_rec(new_pos, *chr, strs) {
                return Some(res);
            }
        } else if counts.iter().all(|(_k, v)| *v == 1) {
            // If there's nothing left to try, then we have a mapping that doesn't violate any
            // constraints so far, but it may produce digits that don't exist so check and skip
            // as needed.

            let maybe_result = new_pos
                .iter()
                .map(|(k, v)| (*k, *v.iter().next().unwrap()))
                .collect::<HashMap<_, _>>();
            let invalid = strs.iter().any(|oo| {
                let tgt = oo
                    .chars()
                    .map(|c| *maybe_result.iter().find(|(_k, v)| **v == c).unwrap().0)
                    .collect::<Vec<_>>();
                identify_digit(&tgt).is_none()
            });
            if !invalid {
                return Some(
                    new_pos
                        .iter()
                        .map(|(k, v)| (*k, *v.iter().next().unwrap()))
                        .collect(),
                );
            } else {
                eprintln!("discarding mapping because it's invalid");
            }
        }
    }

    None
}

fn intersect(cur: &mut HashSet<char>, new: &HashSet<char>) {
    let new_set = cur.intersection(new).copied().collect::<HashSet<_>>();
    *cur = new_set;
}

fn identify_digit(chars: &[char]) -> Option<usize> {
    let mut sorted = chars.iter().collect::<Vec<_>>();
    sorted.sort();
    let s: String = sorted.iter().copied().collect();
    eprintln!("mapping: {}", s);
    match s.as_str() {
        "abcefg" => Some(0),
        "cf" => Some(1),
        "acdeg" => Some(2),
        "acdfg" => Some(3),
        "bcdf" => Some(4),
        "abdfg" => Some(5),
        "abdefg" => Some(6),
        "acf" => Some(7),
        "abcdefg" => Some(8),
        "abcdfg" => Some(9),
        _ => {
            None
            //unreachable!("unknown char digit mapping");
        }
    }
}

use std::collections::HashMap;
use std::collections::HashSet;
fn p2(inputs: &[Vec<&str>], outputs: &[Vec<&str>]) -> usize {
    let all_set: HashSet<_> = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
        .iter()
        .map(|x| *x)
        .collect();
    let init_map = vec![
        ('a', all_set.clone()),
        ('b', all_set.clone()),
        ('c', all_set.clone()),
        ('d', all_set.clone()),
        ('e', all_set.clone()),
        ('f', all_set.clone()),
        ('g', all_set.clone()),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut output_numerals = Vec::new();
    for (i, o) in inputs.iter().zip(outputs.iter()) {
        let io = {
            let mut tmp = i.clone();
            tmp.extend(o.iter());
            tmp
        };

        eprintln!("IO: {:?}", io);
        let mut map = init_map.clone();
        for x in io.iter() {
            match x.len() {
                2 => {
                    // 1
                    let chars = x.chars().take(2).collect::<HashSet<_>>();
                    intersect(&mut map.get_mut(&'c').unwrap(), &chars);
                    intersect(&mut map.get_mut(&'f').unwrap(), &chars);
                }
                3 => {
                    // 7
                    let chars = x.chars().take(3).collect::<HashSet<_>>();
                    intersect(&mut map.get_mut(&'a').unwrap(), &chars);
                    intersect(&mut map.get_mut(&'c').unwrap(), &chars);
                    intersect(&mut map.get_mut(&'f').unwrap(), &chars);
                }
                4 => {
                    // 4
                    let chars = x.chars().take(4).collect::<HashSet<_>>();
                    intersect(&mut map.get_mut(&'b').unwrap(), &chars);
                    intersect(&mut map.get_mut(&'c').unwrap(), &chars);
                    intersect(&mut map.get_mut(&'d').unwrap(), &chars);
                    intersect(&mut map.get_mut(&'f').unwrap(), &chars);
                }
                5 => {
                    // 2, 5
                    // Doesn't narrow it
                }
                6 => {
                    // 0, 3, 6
                    // Doesn't narrow it
                }
                7 => {
                    // 8
                    // Doesn't narrow it
                }
                _ => unreachable!("ahhh"),
            }
        }

        let mut counts = map.iter().map(|(&k, v)| (k, v.len())).collect::<Vec<_>>();
        counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let (chr, _count) = counts.iter().find(|(_chr, count)| *count > 1).unwrap();

        let res = p2_rec(map.clone(), *chr, &io).unwrap();

        let digits = o
            .iter()
            .map(|oo| {
                let tgt = oo
                    .chars()
                    .map(|c| *res.iter().find(|(_k, v)| **v == c).unwrap().0)
                    .collect::<Vec<_>>();
                identify_digit(&tgt)
            })
            .collect::<Vec<_>>();
        let numeral: usize = digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, d)| (10usize).pow(i as u32) * d.unwrap())
            .sum();
        eprintln!("{:?}", res);
        eprintln!("{:?}", digits);
        eprintln!("{:?}", numeral);
        output_numerals.push(numeral);
    }

    output_numerals.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let (inputs, outputs) = parse_file(&file_str);
    let p1 = p1(&outputs);
    println!("p1: {}", p1);
    let p2 = p2(&inputs, &outputs);
    println!("p2: {}", p2);
}

#[cfg(test)]
mod test_day8 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
        let (inputs, outputs) = parse_file(&example);
        let p1 = p1(&outputs);
        let p2 = p2(&inputs, &outputs);
        assert_eq!(p1, 26);
        assert_eq!(p2, 61229);
    }
}
