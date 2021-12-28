//! AoC 2021 - 24

use std::{collections::HashMap, env};
type RegFile = HashMap<char, i64>;

#[derive(Debug, PartialEq, Eq)]
enum RegOrLiteral {
    Reg(char),
    Literal(i64),
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Input(char),
    AddAssign(char, RegOrLiteral),
    MulAssign(char, RegOrLiteral),
    DivAssign(char, RegOrLiteral),
    ModAssign(char, RegOrLiteral),
    EqAssign(char, RegOrLiteral),
}

fn parse_reg_or_lit(s: &str) -> RegOrLiteral {
    if let Ok(l) = s.parse::<i64>() {
        RegOrLiteral::Literal(l)
    } else {
        RegOrLiteral::Reg(s.chars().nth(0).unwrap())
    }
}

fn parse(s: &str) -> Vec<Instruction> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut w = l.split(' ');
            let opcode = w.next().unwrap();
            let first_op = w.next().unwrap().chars().nth(0).unwrap();
            match opcode {
                "inp" => Instruction::Input(first_op),
                "add" => Instruction::AddAssign(first_op, parse_reg_or_lit(w.next().unwrap())),
                "mul" => Instruction::MulAssign(first_op, parse_reg_or_lit(w.next().unwrap())),
                "div" => Instruction::DivAssign(first_op, parse_reg_or_lit(w.next().unwrap())),
                "mod" => Instruction::ModAssign(first_op, parse_reg_or_lit(w.next().unwrap())),
                "eql" => Instruction::EqAssign(first_op, parse_reg_or_lit(w.next().unwrap())),
                _ => {
                    unreachable!("unknown opcode string");
                }
            }
        })
        .collect::<Vec<_>>()
}

fn run_digit(instructions: &[Instruction], reg_file: &mut HashMap<char, i64>, input: u8) {
    for i in instructions.iter() {
        match i {
            Instruction::Input(first_op) => {
                *reg_file.get_mut(first_op).unwrap() = input as i64;
            }
            Instruction::AddAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = first + second
            }
            Instruction::MulAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = first * second;
            }
            Instruction::DivAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = first / second;
            }
            Instruction::ModAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = first % second;
            }
            Instruction::EqAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = if first == second { 1 } else { 0 };
            }
        }
    }
}

fn search_digits(instructions: &[Instruction]) {
    let progress_so_far = vec![];
    let mut sols = None;
    let mut reg_file = RegFile::new();
    reg_file.insert('x', 0);
    reg_file.insert('y', 0);
    reg_file.insert('z', 0);
    reg_file.insert('w', 0);
    reg_file.shrink_to_fit();
    let mut invalid_z = vec![vec![]; 14];
    search_digits_impl_low(
        instructions,
        &progress_so_far,
        &mut sols,
        &reg_file,
        &mut invalid_z,
    );
    eprintln!("sols: {:?}", sols);
}

fn search_digits_impl_low(
    instructions: &[Instruction],
    progress_so_far: &Vec<u8>,
    solution: &mut Option<Vec<u8>>,
    reg_file_so_far: &RegFile,
    invalid_z: &mut Vec<Vec<i64>>,
) -> bool {
    let level = progress_so_far.len();
    if let Some(sol) = solution.as_ref() {
        if sol < progress_so_far {
            eprintln!("Prunning {:?}: Sol: {:?}", progress_so_far, sol);
            return false; // already worse than the solution
        }
    }
    if progress_so_far.len() >= 14 {
        eprintln!("{:?}: Sol: {}", progress_so_far, solution.is_some());
        if reg_file_so_far[&'z'] == 0 {
            //if progress_so_far > solution { // already check?
            *solution = Some(progress_so_far.clone());
            // }
            return true;
        } else {
            return false;
        };
    }

    let mut new_prog = progress_so_far.clone();
    new_prog.push(0);
    let mut new_reg_file = reg_file_so_far.clone();
    let old_z = reg_file_so_far[&'z'];
    for new_digit in 1..=9 {
        new_prog.pop();
        new_prog.push(new_digit);
        *new_reg_file.get_mut(&'z').unwrap() = old_z;
        run_digit(
            &instructions[(18 * level)..((level + 1) * 18)],
            &mut new_reg_file,
            new_digit,
        );
        let current_z = &new_reg_file[&'z'];
        let invalid_zs_for_this_level = &invalid_z[level];
        let search_res = invalid_zs_for_this_level.binary_search(current_z);

        if search_res.is_ok() {
            continue;
        }

        let recur_result =
            search_digits_impl_low(instructions, &new_prog, solution, &new_reg_file, invalid_z);
        if recur_result {
            // do something
        } else {
            let invalid_zs_for_this_level = &mut invalid_z[level];
            let search_res = invalid_zs_for_this_level.binary_search(&current_z);
            assert!(search_res.is_err());
            invalid_zs_for_this_level.insert(search_res.unwrap_err(), *current_z);
            //assert!(invalid_zs_for_this_level.windows(2).all(|x| x[0] <= x[1]));
        }

        //let res = run(&instructions[(18 * 3)..(4 * 18)], &digits);
    }

    false
}

fn search_digits_impl(
    instructions: &[Instruction],
    progress_so_far: &Vec<u8>,
    solution: &mut Option<Vec<u8>>,
    reg_file_so_far: &RegFile,
    invalid_z: &mut Vec<Vec<i64>>,
) -> bool {
    let level = progress_so_far.len();
    if let Some(sol) = solution.as_ref() {
        let is_prefix = {
            let sol_prefix = &sol[0..level];
            sol_prefix == progress_so_far
        };
        if !is_prefix && sol > progress_so_far {
            eprintln!("Prunning {:?}: Sol: {:?}", progress_so_far, sol);
            return false; // already worse than the solution
        }
    }
    if progress_so_far.len() >= 14 {
        eprintln!("{:?}: Sol: {}", progress_so_far, solution.is_some());
        if reg_file_so_far[&'z'] == 0 {
            //if progress_so_far > solution { // already check?
            *solution = Some(progress_so_far.clone());
            // }
            return true;
        } else {
            return false;
        };
    }

    let mut new_prog = progress_so_far.clone();
    new_prog.push(0);
    let mut new_reg_file = reg_file_so_far.clone();
    let old_z = reg_file_so_far[&'z'];
    for new_digit in (1..=9).rev() {
        new_prog.pop();
        new_prog.push(new_digit);
        *new_reg_file.get_mut(&'z').unwrap() = old_z;
        run_digit(
            &instructions[(18 * level)..((level + 1) * 18)],
            &mut new_reg_file,
            new_digit,
        );
        let current_z = &new_reg_file[&'z'];
        let invalid_zs_for_this_level = &invalid_z[level];
        let search_res = invalid_zs_for_this_level.binary_search(current_z);

        if search_res.is_ok() {
            continue;
        }

        let recur_result =
            search_digits_impl(instructions, &new_prog, solution, &new_reg_file, invalid_z);
        if recur_result {
            // do something
        } else {
            let invalid_zs_for_this_level = &mut invalid_z[level];
            let search_res = invalid_zs_for_this_level.binary_search(&current_z);
            assert!(search_res.is_err());
            invalid_zs_for_this_level.insert(search_res.unwrap_err(), *current_z);
            //assert!(invalid_zs_for_this_level.windows(2).all(|x| x[0] <= x[1]));
        }

        //let res = run(&instructions[(18 * 3)..(4 * 18)], &digits);
    }

    false
}

fn run(instructions: &[Instruction], input_buf: &[i64]) -> i64 {
    let mut reg_file = std::collections::HashMap::<char, i64>::new();
    reg_file.insert('x', 0);
    reg_file.insert('y', 0);
    reg_file.insert('z', 0);
    reg_file.insert('w', 0);
    reg_file.shrink_to_fit();
    let mut buff_count = 0;

    for i in instructions.iter() {
        match i {
            Instruction::Input(first_op) => {
                *reg_file.get_mut(first_op).unwrap() = input_buf[buff_count];
                buff_count += 1;
                eprintln!("DIGIT");
            }
            Instruction::AddAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = first + second;
                if *first_op == 'z' {
                    eprintln!(
                        "x: {}, y: {}, z: {}, w: {}",
                        reg_file[&'x'], reg_file[&'y'], reg_file[&'z'], reg_file[&'w']
                    );
                }
            }
            Instruction::MulAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = first * second;
            }
            Instruction::DivAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = first / second;
            }
            Instruction::ModAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = first % second;
            }
            Instruction::EqAssign(first_op, second_op) => {
                let first = reg_file[first_op].clone();
                let second = match *second_op {
                    RegOrLiteral::Reg(reg) => reg_file[&reg].clone(),
                    RegOrLiteral::Literal(l) => l,
                };
                *reg_file.get_mut(first_op).unwrap() = if first == second { 1 } else { 0 };
            }
        }
    }
    eprintln!(
        "x: {}, y: {}, z: {}, w: {}",
        reg_file[&'x'], reg_file[&'y'], reg_file[&'z'], reg_file[&'w']
    );

    reg_file[&'z']
}

fn p1(instructions: &[Instruction]) -> i64 {
    for mn in (11111111111111..=99999999999999).rev() {
        //for mn in (1..=9).rev() {
        eprintln!("mn: {}", mn);
        let mn_str = mn.to_string();
        if mn_str.chars().any(|c| c == '0') {
            continue;
        }
        let digits = mn_str
            .chars()
            .map(|c| c.to_string().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        //let res = run(&instructions[(18 * 3)..(4 * 18)], &digits);
        eprintln!("{:?}", digits);
        let res = run(instructions, &digits);
        if res == 0 {
            eprintln!("mn: {}", mn);
            return mn;
        }
    }
    eprintln!("didn'at find");

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let instrs = parse(&file_str);
    eprintln!("instrs: {:?}", instrs);
    // p1(&instrs);
    search_digits(&instrs)
}

// DIGIT 1
// inp w
// i64 w = digit1
// mul x 0
// add x z
// mod x 26
// div z 1
// i64 x = 0
// i64 z = 0
// add x 11
// x += 11;
// eql x w
// x = x == 11 ? { 1} : {0}
// eql x 0
// x = x == 0 ? { 1} : {0}
// mul y 0
// i64 y = 0;
// add y 25
// y += 26;
// mul y x
// y = x*y
// add y 1
// y += 1
// mul z y
// z = 0
// mul y 0
// y = 0
// add y w
// y = w + 0
// add y 14
// y += 14
// mul y x
// y = y * x
// add z y
// z = z + y
//
// i64 w = digit1
// i64 x = 0
// i64 z = 0
// x = 1
// z = 0
// y = w + 14
// mul y x
// y = y * x
// add z y
// z = z + y
