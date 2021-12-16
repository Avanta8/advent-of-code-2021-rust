use std::io::Read;

use bit_vec::BitVec;

fn parse_input() -> BitVec {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut bytes = vec![];
    let mut chars = input.trim().split_inclusive(|_| true);
    while let Some(c) = chars.next() {
        let mut n = u8::from_str_radix(c, 16).unwrap() << 4;
        if let Some(d) = chars.next() {
            n += u8::from_str_radix(d, 16).unwrap();
        }
        bytes.push(n);
    }
    BitVec::from_bytes(&bytes)
}

struct Decoder<X: Iterator<Item = bool>> {
    bits: X,
    count: usize,
}

impl<X: Iterator<Item = bool>> Decoder<X> {
    fn readn(&mut self, n: usize) -> u64 {
        let mut res = 0;
        for _ in 0..n {
            res = (res << 1) | self.bits.next().unwrap() as u64;
            self.count += 1;
        }
        res
    }

    fn parse_literal(&mut self) -> u64 {
        let mut total = 0;
        while let Some(rem) = self.bits.next() {
            self.count += 1;
            total = (total << 4) + self.readn(4);
            if !rem {
                break;
            }
        }
        total
    }

    fn parse_operator(&mut self, type_id: u64) -> u64 {
        let length_type = self.readn(1) != 0;

        let mut sub_packets = vec![];
        if length_type {
            for _ in 0..self.readn(11) {
                sub_packets.push(self.parse_packet());
            }
        } else {
            let length = self.readn(15);

            let count = self.count;
            while self.count - count < length as usize {
                sub_packets.push(self.parse_packet());
            }
        }

        match type_id {
            0 => sub_packets.into_iter().sum(),
            1 => sub_packets.into_iter().reduce(|a, b| a * b).unwrap(),
            2 => sub_packets.into_iter().min().unwrap(),
            3 => sub_packets.into_iter().max().unwrap(),
            5 => (sub_packets[0] > sub_packets[1]) as u64,
            6 => (sub_packets[0] < sub_packets[1]) as u64,
            7 => (sub_packets[0] == sub_packets[1]) as u64,
            _ => unreachable!(),
        }
    }

    fn parse_packet(&mut self) -> u64 {
        let _version = self.readn(3);
        let type_id = self.readn(3);
        match type_id {
            4 => self.parse_literal(),
            _ => self.parse_operator(type_id),
        }
    }
}

fn main() {
    let bits = parse_input();
    let mut decoder = Decoder {
        bits: bits.iter(),
        count: 0,
    };

    let ans = decoder.parse_packet();
    println!("{}", ans);
}
