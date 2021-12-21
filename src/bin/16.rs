use std::str::FromStr;
use std::io;

use aoc_2021::input::*;

fn main() -> io::Result<()> {
    for line in lines()? {
        println!("{}", line.parse::<Transmission>().ok().and_then(|mut t| t.total_version()).expect("no packets"));
        println!("{}", line.parse::<Transmission>().ok().and_then(|mut t| t.evaluate()).expect("no packets"));
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum OpType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo
}

impl OpType {
    fn from_number(n: usize) -> Option<Self> {
        match n {
            0 => Some(Self::Sum),
            1 => Some(Self::Product),
            2 => Some(Self::Minimum),
            3 => Some(Self::Maximum),
            5 => Some(Self::GreaterThan),
            6 => Some(Self::LessThan),
            7 => Some(Self::EqualTo),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal { version: usize, value: usize },
    Op { version: usize, op_type: OpType, subs: Vec<Packet> }
}

impl Packet {
    fn total_version(&self) -> usize {
        match self {
            Self::Literal { version, value: _ } => *version,
            Self::Op { version, op_type: _, subs } => {
                *version + subs.iter().map(|sub_packet| sub_packet.total_version()).sum::<usize>()
            }
        }
    }

    fn evaluate(&self) -> Option<usize> {
        match self {
            Self::Literal { version: _, value } => Some(*value),
            Self::Op { version: _, op_type, subs } => {
                let mut values = subs.iter().map(|sub_packet| sub_packet.evaluate().expect("no sub-packets"));

                Some(
                    match op_type {
                        OpType::Sum => { values.sum::<usize>() },
                        OpType::Product => { values.product::<usize>() },
                        OpType::Minimum => { values.min().expect("no sub-packets") },
                        OpType::Maximum => { values.max().expect("no sub-packets") },
                        OpType::GreaterThan => {
                            let a = values.next()?;
                            let b = values.next()?;

                            if a > b { 1 } else { 0 }
                        },
                        OpType::LessThan => {
                            let a = values.next()?;
                            let b = values.next()?;

                            if a < b { 1 } else { 0 }
                        },
                        OpType::EqualTo => {
                            let a = values.next()?;
                            let b = values.next()?;

                            if a == b { 1 } else { 0 }
                        },
                    }
                )
            }
        }
    }
}

struct Transmission {
    bits: Vec<bool>,
    current: usize
}

impl Transmission {
    #[cfg(test)]
    fn as_slice<'a>(&'a self) -> &[bool] {
        &self.bits
    }

    fn total_version(&mut self) -> Option<usize> {
        Some(self.consume_packet()?.total_version())
    }

    fn evaluate(&mut self) -> Option<usize> {
        self.consume_packet()?.evaluate()
    }

    fn consume(&mut self, n: usize) -> Option<Vec<bool>> {
        if self.current + n > self.bits.len() {
            None
        } else {
            let slice = &self.bits[self.current..(self.current + n)];
            self.current += n;

            Some(slice.to_vec())
        }
    }

    fn consume_packet(&mut self) -> Option<Packet> {
        let version = to_decimal(&self.consume(3)?);
        let type_id = to_decimal(&self.consume(3)?);

        if type_id == 4 { // literal value
            let mut data = vec! [];

            loop {
                let group = self.consume(5)?;
                data.append(&mut group[1..].to_vec());
                if !group[0] {
                    break
                }
            }

            Some(Packet::Literal { version: version, value: to_decimal(&data) })
        } else {
            let length_type_id = self.consume(1)?[0];
            let op_type = OpType::from_number(type_id)?;
            let subs = if length_type_id == false {
                let num_bits = to_decimal(&self.consume(15)?);
                let mut sub_transmission = Self { bits: self.consume(num_bits)?, current: 0 };
                let mut sub_packets = vec! [];

                while let Some(sub_packet) = sub_transmission.consume_packet() {
                    sub_packets.push(sub_packet);
                }

                sub_packets
            } else {
                let num_sub_packets = to_decimal(&self.consume(11)?);

                (0..num_sub_packets)
                    .map(|_| self.consume_packet().expect("not a sub-packet"))
                    .collect::<Vec<_>>()
            };

            Some(Packet::Op { version: version, op_type: op_type, subs: subs })
        }
    }
}

impl FromStr for Transmission {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = Vec::with_capacity(4 * s.len());

        for ch in s.chars() {
            match ch.to_ascii_lowercase() {
                '0' => bits.append(&mut vec![false, false, false, false]),
                '1' => bits.append(&mut vec![false, false, false, true]),
                '2' => bits.append(&mut vec![false, false, true,  false]),
                '3' => bits.append(&mut vec![false, false, true,  true]),
                '4' => bits.append(&mut vec![false, true,  false, false]),
                '5' => bits.append(&mut vec![false, true,  false, true]),
                '6' => bits.append(&mut vec![false, true,  true,  false]),
                '7' => bits.append(&mut vec![false, true,  true,  true]),
                '8' => bits.append(&mut vec![true,  false, false, false]),
                '9' => bits.append(&mut vec![true,  false, false, true]),
                'a' => bits.append(&mut vec![true,  false, true,  false]),
                'b' => bits.append(&mut vec![true,  false, true,  true]),
                'c' => bits.append(&mut vec![true,  true,  false, false]),
                'd' => bits.append(&mut vec![true,  true,  false, true]),
                'e' => bits.append(&mut vec![true,  true,  true,  false]),
                'f' => bits.append(&mut vec![true,  true,  true,  true]),
                _ => return Err(())
            };
        }

        Ok(Self { bits, current: 0 })
    }
}

fn to_decimal(bits: &[bool]) -> usize {
    let mut out = 0;

    for (i, &b) in bits.iter().rev().enumerate() {
        if b {
            out += 2_usize.pow(i as u32);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01_to_binary() {
        let bv = "D2FE28".parse::<Transmission>().expect("not a transmission");

        assert_eq!(
            bv.as_slice(),
            &[
                true, true, false, true,
                false, false, true, false,
                true, true, true, true,
                true, true, true, false,
                false, false, true, false,
                true, false, false, false
            ]
        );
    }

    #[test]
    fn _01_to_dec() {
        let _4 = [true, false, false];
        let _2021 = [true, true, true, true, true, true, false, false, true, false, true];

        assert_eq!(to_decimal(&_4), 4);
        assert_eq!(to_decimal(&_2021), 2021);
    }

    #[test]
    fn _01_parse_literal() {
        let mut transission = "D2FE28".parse::<Transmission>().expect("not a transmission");
        let packet = transission.consume_packet().expect("not a packet");

        assert_eq!(packet, Packet::Literal { version: 6, value: 2021 });
    }

    #[test]
    fn _01_parse_operator_0() {
        let mut transmission = "38006F45291200".parse::<Transmission>().expect("not a transmission");
        let packet = transmission.consume_packet().expect("not a packet");

        assert_eq!(
            packet,
            Packet::Op {
                version: 1,
                op_type: OpType::LessThan,
                subs: vec! [
                    Packet::Literal { version: 6, value: 10 },
                    Packet::Literal { version: 2, value: 20 },
                ]
            }
        );
    }

    #[test]
    fn _01_parse_operator_1() {
        let mut transmission = "EE00D40C823060".parse::<Transmission>().expect("not a transmission");
        let packet = transmission.consume_packet().expect("not a packet");

        assert_eq!(
            packet,
            Packet::Op {
                version: 7,
                op_type: OpType::Maximum,
                subs: vec! [
                    Packet::Literal { version: 2, value: 1 },
                    Packet::Literal { version: 4, value: 2 },
                    Packet::Literal { version: 1, value: 3 },
                ]
            }
        );
    }

    #[test]
    fn _01_example() {
        assert_eq!(
            "8A004A801A8002F478".parse::<Transmission>().ok().and_then(|mut t| t.total_version()),
            Some(16)
        );
        assert_eq!(
            "620080001611562C8802118E34".parse::<Transmission>().ok().and_then(|mut t| t.total_version()),
            Some(12)
        );
        assert_eq!(
            "C0015000016115A2E0802F182340".parse::<Transmission>().ok().and_then(|mut t| t.total_version()),
            Some(23)
        );
        assert_eq!(
            "A0016C880162017C3686B18A3D4780".parse::<Transmission>().ok().and_then(|mut t| t.total_version()),
            Some(31)
        );
    }

    #[test]
    fn _02_example() {
        assert_eq!(
            "C200B40A82".parse::<Transmission>().ok().and_then(|mut t| t.evaluate()),
            Some(3)
        );
        assert_eq!(
            "04005AC33890".parse::<Transmission>().ok().and_then(|mut t| t.evaluate()),
            Some(54)
        );
        assert_eq!(
            "880086C3E88112".parse::<Transmission>().ok().and_then(|mut t| t.evaluate()),
            Some(7)
        );
        assert_eq!(
            "CE00C43D881120".parse::<Transmission>().ok().and_then(|mut t| t.evaluate()),
            Some(9)
        );
        assert_eq!(
            "D8005AC2A8F0".parse::<Transmission>().ok().and_then(|mut t| t.evaluate()),
            Some(1)
        );
        assert_eq!(
            "F600BC2D8F".parse::<Transmission>().ok().and_then(|mut t| t.evaluate()),
            Some(0)
        );
        assert_eq!(
            "9C005AC2F8F0".parse::<Transmission>().ok().and_then(|mut t| t.evaluate()),
            Some(0)
        );
        assert_eq!(
            "9C0141080250320F1802104A08".parse::<Transmission>().ok().and_then(|mut t| t.evaluate()),
            Some(1)
        );
    }
}
