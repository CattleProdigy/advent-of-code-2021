//! AoC 2021 - 02

use std::{env, error::Error, str::FromStr};

/// Direction
#[derive(Debug)]
enum Dir {
    Forward,
    Up,
    Down,
}

impl FromStr for Dir {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Dir::Forward),
            "up" => Ok(Dir::Up),
            "down" => Ok(Dir::Down),
            _ => Err("Unknown Dir string")?,
        }
    }
}

/// Command
#[derive(Debug)]
struct Command {
    pub dir: Dir,
    pub dist: i64,
}

impl FromStr for Command {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokened = s.split_whitespace().collect::<Vec<_>>();
        if tokened.len() != 2 {
            Err("Incorrect number of tokens")?
        }

        let dir = Dir::from_str(tokened[0])?;
        let dist = tokened[1].parse::<i64>()?;

        Ok(Self { dir, dist })
    }
}

fn parse_file(s: &str) -> Vec<Command> {
    s.lines().map(|l| Command::from_str(l).unwrap()).collect()
}

fn run_p1(commands: &[Command]) -> (i64, i64) {
    commands.iter().fold((0, 0), |mut acc, comm| {
        match comm.dir {
            Dir::Forward => acc.0 += comm.dist,
            Dir::Up => acc.1 -= comm.dist,
            Dir::Down => acc.1 += comm.dist,
        };
        acc
    })
}

fn run_p2(commands: &[Command]) -> (i64, i64, i64) {
    commands.iter().fold((0, 0, 0), |mut acc, comm| {
        match comm.dir {
            Dir::Forward => {
                acc.0 += comm.dist;
                acc.1 += acc.2 * comm.dist
            }
            Dir::Up => acc.2 -= comm.dist,
            Dir::Down => acc.2 += comm.dist,
        };
        acc
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");

    let commands = parse_file(&file_str);

    let p1_pos = run_p1(&commands);
    let p1_prod = p1_pos.0 * p1_pos.1;
    println!("p1: {}", p1_prod);

    let p2_pos = run_p2(&commands);
    let p2_prod = p2_pos.0 * p2_pos.1;
    println!("p2: {}", p2_prod);
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

        let commands = parse_file(example);
        let res = run_p1(&commands);
        assert_eq!(15, res.0);
        assert_eq!(10, res.1);

        let res2 = run_p2(&commands);
        assert_eq!(15, res2.0);
        assert_eq!(60, res2.1);
    }
}
