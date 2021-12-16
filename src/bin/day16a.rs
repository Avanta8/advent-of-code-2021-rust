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

    fn parse_literal(&mut self) {
        while let Some(rem) = self.bits.next() {
            self.count += 1;
            self.readn(4);
            if !rem {
                break;
            }
        }
    }

    fn parse_operator(&mut self) -> u64 {
        let length_type = self.readn(1) != 0;

        let mut total = 0;
        if length_type {
            for _ in 0..self.readn(11) {
                total += self.parse_packet();
            }
        } else {
            let length = self.readn(15);

            let count = self.count;
            while self.count - count < length as usize {
                total += self.parse_packet();
            }
        }

        total
    }

    fn parse_packet(&mut self) -> u64 {
        let mut version = self.readn(3);
        let type_id = self.readn(3);
        if type_id == 4 {
            self.parse_literal();
        } else {
            version += self.parse_operator();
        }
        version
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
