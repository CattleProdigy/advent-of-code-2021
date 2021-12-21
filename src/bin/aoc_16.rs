//! AoC 2021 - 16

use std::env;

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    pub version: u8,
    pub payload: Payload,
}

#[derive(Debug, PartialEq, Eq)]
struct Operator {
    operator: u8,
    subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq, Eq)]
enum Payload {
    Literal(u64),
    Operator(Operator),
}

const BIG_END_HEX: [[u8; 4]; 16] = [
    [0, 0, 0, 0],
    [0, 0, 0, 1],
    [0, 0, 1, 0],
    [0, 0, 1, 1],
    [0, 1, 0, 0],
    [0, 1, 0, 1],
    [0, 1, 1, 0],
    [0, 1, 1, 1],
    [1, 0, 0, 0],
    [1, 0, 0, 1],
    [1, 0, 1, 0],
    [1, 0, 1, 1],
    [1, 1, 0, 0],
    [1, 1, 0, 1],
    [1, 1, 1, 0],
    [1, 1, 1, 1],
];

fn assemble<const B: usize>(bits: &[u8]) -> u64 {
    let mut x: u64 = 0;
    for (i, exp) in (0..B).rev().enumerate() {
        x += (bits[i] as u64) << exp;
    }
    x
}

fn parse_packet(stream: &[u8]) -> (Packet, usize) {
    let mut i = 0;
    let version = assemble::<3>(stream);
    i += 3;

    let packet_type = assemble::<3>(&stream[i..]);
    i += 3;
    let packet: Packet;

    if packet_type == 4 {
        let mut literal = [0u8; 64];
        let mut count = 0;
        loop {
            let literal_chunk_with_continuation = &stream[i..i + 5];
            i += 5;
            literal[count * 4..(count + 1) * 4]
                .clone_from_slice(&literal_chunk_with_continuation[1..5]);
            count += 1;
            if literal_chunk_with_continuation[0] == 0 {
                break;
            }
        }
        let bit_count = count * 4;
        literal.rotate_right(64 - bit_count);

        let literal = assemble::<64>(&literal[..]);
        packet = Packet {
            version: version as u8,
            payload: Payload::Literal(literal),
        };
    } else {
        let mut subpackets = Vec::<Packet>::new();
        let length_id = stream[i];
        i += 1;
        if length_id == 0 {
            let length_bits = assemble::<15>(&stream[i..]);
            i += 15;
            assert!(length_bits != 0);
            let mut total_bits = 0;
            loop {
                let (subpacket, bits) = parse_packet(&stream[i..]);
                i += bits;
                total_bits += bits;
                subpackets.push(subpacket);
                if total_bits >= length_bits as usize {
                    break;
                }
            }
        } else {
            let length_packets = assemble::<11>(&stream[i..]);
            i += 11;
            for _p in 0..length_packets {
                let (subpacket, bits) = parse_packet(&stream[i..]);
                i += bits;
                subpackets.push(subpacket);
            }
        }
        packet = Packet {
            version: version as u8,
            payload: Payload::Operator(Operator {
                operator: packet_type as u8,
                subpackets,
            }),
        }
    }

    (packet, i)
}

fn parse_file(s: &str) -> Packet {
    let bit_stream = s
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| {
            BIG_END_HEX[usize::from_str_radix(&c.to_string(), 16).unwrap()]
                .iter()
                .copied()
        })
        .flatten()
        .collect::<Vec<_>>();

    parse_packet(&bit_stream).0
}

fn p1(p: &Packet) -> usize {
    let mut version_sum = 0;
    let mut stack = Vec::<&Packet>::new();
    stack.push(p);
    while let Some(cur_p) = stack.pop() {
        version_sum += cur_p.version as usize;
        match &cur_p.payload {
            Payload::Literal(_) => {}
            Payload::Operator(op) => {
                for sp in op.subpackets.iter() {
                    stack.push(&sp);
                }
            }
        }
    }

    version_sum
}

fn p2(p: &Packet) -> u64 {
    match &p.payload {
        Payload::Literal(i) => *i,
        Payload::Operator(op) => {
            let operands = op.subpackets.iter().map(|sp| p2(sp)).collect::<Vec<_>>();
            if op.operator == 0 {
                operands.iter().sum::<u64>()
            } else if op.operator == 1 {
                operands.iter().product::<u64>()
            } else if op.operator == 2 {
                operands.iter().copied().min().unwrap()
            } else if op.operator == 3 {
                operands.iter().copied().max().unwrap()
            } else if op.operator == 5 {
                if operands[0] > operands[1] {
                    1
                } else {
                    0
                }
            } else if op.operator == 6 {
                if operands[0] < operands[1] {
                    1
                } else {
                    0
                }
            } else if op.operator == 7 {
                if operands[0] == operands[1] {
                    1
                } else {
                    0
                }
            } else {
                panic!("invalid op");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let packet = parse_file(&file_str);
    let p1 = p1(&packet);
    println!("p1: {}", p1);
    let p2 = p2(&packet);
    println!("p2: {}", p2);
}

#[cfg(test)]
mod test_day16 {
    use super::*;

    #[test]
    fn test() {
        {
            let example = "D2FE28";
            parse_file(&example);
        }
        {
            let example = "38006F45291200";
            parse_file(&example);
        }
        {
            let example = "EE00D40C823060";
            parse_file(&example);
        }
        {
            let example = "8A004A801A8002F478";
            assert_eq!(p1(&parse_file(&example)), 16);
        }
        {
            let example = "C0015000016115A2E0802F182340";
            assert_eq!(p1(&parse_file(&example)), 23);
        }
        {
            let example = "A0016C880162017C3686B18A3D4780";
            assert_eq!(p1(&parse_file(&example)), 31);
        }
    }
}
