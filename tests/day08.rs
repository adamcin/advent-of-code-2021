mod common;

use std::collections::HashMap;
use std::str::FromStr;

fn read() -> Vec<SignalsInput> {
    let values: Vec<SignalsInput> = common::read_test_input("data/day-08/input.txt")
        .iter()
        .flat_map(|line| Signal::parse_line(line))
        .clone()
        .collect();
    values
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Signal {
    A = 1,
    B = 2,
    C = 4,
    D = 8,
    E = 16,
    F = 32,
    G = 64,
}
use Signal::{A, B, C, D, E, F, G};
impl FromStr for Signal {
    type Err = ();

    fn from_str(input: &str) -> Result<Signal, Self::Err> {
        match input {
            "a" => Ok(A),
            "b" => Ok(B),
            "c" => Ok(C),
            "d" => Ok(D),
            "e" => Ok(E),
            "f" => Ok(F),
            "g" => Ok(G),
            _ => Err(()),
        }
    }
}

type PackedSignalSets = [u8; 10];
type PackedSignalOuts = [u8; 4];
type SignalsInput = (PackedSignalSets, PackedSignalOuts);

impl Signal {
    pub fn iterator() -> impl Iterator<Item = Signal> {
        [A, B, C, D, E, F, G].iter().copied()
    }

    pub fn len(value: u8) -> usize {
        let on: Vec<u8> = Signal::iterator()
            .map(|s| s as u8)
            .filter(|&s| s == s & value)
            .collect();
        return on.len();
    }

    pub fn pack(as_vec: Vec<Signal>) -> u8 {
        return Signal::pack_r(&as_vec);
    }

    pub fn pack_r(as_vec: &Vec<Signal>) -> u8 {
        let mut int: u8 = 0;
        for &wire in as_vec {
            int |= wire as u8;
        }
        return int;
    }

    pub fn parse(input: &str) -> Vec<Signal> {
        return input.split("").flat_map(|c| c.parse().ok()).collect();
    }

    pub fn parse_line(line: &str) -> Option<SignalsInput> {
        if let Some((sets_s, outs_s)) = line.split_once(" | ") {
            let mut sets: PackedSignalSets = [0; 10];
            for (index, sig) in sets_s
                .trim()
                .split(' ')
                .map(|sig_s| Signal::parse(sig_s))
                .map(|sig_v| Signal::pack(sig_v))
                .take(10)
                .enumerate()
            {
                sets[index] = sig;
            }

            let mut outs: PackedSignalOuts = [0; 4];
            for (index, sig) in outs_s
                .trim()
                .split(' ')
                .map(|sig_s| Signal::parse(sig_s))
                .map(|sig_v| Signal::pack(sig_v))
                .take(4)
                .enumerate()
            {
                outs[index] = sig;
            }

            return Some((sets, outs));
        }
        return None;
    }

    fn decode_top_right(enc: &[Option<u8>; 10]) -> Option<u8> {
        if let (Some(one), Some(six)) = (enc[1], enc[6]) {
            return Some(one - (one & six));
        }
        None
    }

    fn decode_top_left(enc: &[Option<u8>; 10]) -> Option<u8> {
        if let (Some(three), Some(four)) = (enc[3], enc[4]) {
            return Some((three | four) - three);
        }
        None
    }

    fn decode_middle(enc: &[Option<u8>; 10]) -> Option<u8> {
        if let (Some(one), Some(four), Some(top_left)) =
            (enc[1], enc[4], Signal::decode_top_left(enc))
        {
            return Some(four - top_left - one);
        }
        None
    }

    fn is_1(sigset: u8) -> bool {
        return Signal::len(sigset) == 2;
    }

    fn is_7(sigset: u8) -> bool {
        return Signal::len(sigset) == 3;
    }

    fn is_4(sigset: u8) -> bool {
        return Signal::len(sigset) == 4;
    }

    fn is_8(sigset: u8) -> bool {
        return Signal::len(sigset) == 7;
    }

    fn is_6(enc: &[Option<u8>; 10], sigset: u8) -> bool {
        if Signal::len(sigset) == 6 {
            if let Some(one) = enc[1] {
                return Signal::len(sigset & one) == 1;
            }
        }
        false
    }

    fn is_3(enc: &[Option<u8>; 10], sigset: u8) -> bool {
        if Signal::len(sigset) == 5 {
            if let Some(one) = enc[1] {
                return sigset == sigset | one;
            }
        }
        false
    }

    fn is_2(enc: &[Option<u8>; 10], sigset: u8) -> bool {
        if Signal::len(sigset) == 5 {
            if let (Some(four), Some(six), Some(middle)) =
                (enc[4], enc[6], Signal::decode_middle(enc))
            {
                return sigset == (six ^ four) | middle;
            }
        }
        false
    }

    fn is_5(enc: &[Option<u8>; 10], sigset: u8) -> bool {
        if Signal::len(sigset) == 5 {
            if let (Some(three), Some(top_left), Some(top_right)) = (
                enc[3],
                Signal::decode_top_left(enc),
                Signal::decode_top_right(enc),
            ) {
                return sigset == three - top_right + top_left;
            }
        }
        false
    }

    fn is_0(enc: &[Option<u8>; 10], sigset: u8) -> bool {
        if Signal::len(sigset) == 6 {
            if let (Some(eight), Some(middle)) = (enc[8], Signal::decode_middle(enc)) {
                return sigset == eight - middle;
            }
        }
        false
    }

    fn is_9(enc: &[Option<u8>; 10], sigset: u8) -> bool {
        if Signal::len(sigset) == 6 {
            if let (Some(three), Some(top_left)) = (enc[3], Signal::decode_top_left(enc)) {
                return sigset == three | top_left;
            }
        }
        false
    }

    fn decode(input: SignalsInput) -> usize {
        let (sets, outs) = input;
        let decoder = Signal::make_decoder(sets);
        let mut decoded: usize = 0;
        let outs_len = outs.len();
        let base: usize = 10;
        for (index, out) in outs.iter().enumerate() {
            if let Some(&value) = decoder.get(out) {
                decoded += base.pow((outs_len - 1 - index) as u32) * value;
            }
        }
        return decoded;
    }

    fn make_decoder(sets: PackedSignalSets) -> HashMap<u8, usize> {
        let mut enc: [Option<u8>; 10] = [None; 10];

        // phase 1, no dependencies
        for &sigset in sets.iter() {
            if Signal::is_1(sigset) {
                enc[1] = Some(sigset);
            } else if Signal::is_7(sigset) {
                enc[7] = Some(sigset);
            } else if Signal::is_4(sigset) {
                enc[4] = Some(sigset);
            } else if Signal::is_8(sigset) {
                enc[8] = Some(sigset);
            }
        }

        // phase 2, minimally dependent
        for &sigset in sets.iter() {
            if Signal::is_6(&enc, sigset) {
                enc[6] = Some(sigset);
            } else if Signal::is_3(&enc, sigset) {
                enc[3] = Some(sigset);
            }
        }

        // phase 3, final deciphering
        for &sigset in sets.iter() {
            if Signal::is_2(&enc, sigset) {
                enc[2] = Some(sigset);
            } else if Signal::is_5(&enc, sigset) {
                enc[5] = Some(sigset);
            } else if Signal::is_0(&enc, sigset) {
                enc[0] = Some(sigset);
            } else if Signal::is_9(&enc, sigset) {
                enc[9] = Some(sigset);
            }
        }

        let mut decoder: HashMap<u8, usize> = HashMap::new();
        for (index, &sig) in enc.iter().enumerate() {
            assert_ne!(None, sig, "enc should have some sig in index {}", index);
            if let Some(value) = sig {
                decoder.insert(value, index);
            }
        }
        return decoder;
    }
}

#[test]
fn day08test_three_from_one() {
    let one_unpacked = Signal::parse("cf");
    let two_unpacked = Signal::parse("acdeg");
    let three_unpacked = Signal::parse("acdfg");

    let one = Signal::pack_r(&one_unpacked);
    let two = Signal::pack_r(&two_unpacked);
    let three = Signal::pack_r(&three_unpacked);

    assert_ne!(two, two | one, "expect two bitor one to not equal two");
    assert_eq!(three, three | one, "expect three bitor one to equal three");
}

#[test]
fn day08part1() {
    let inputs = read();
    assert_eq!(200, inputs.len(), "expect number of inputs");

    let selected: Vec<u8> = inputs
        .iter()
        .map(|(_, outs)| outs)
        .flat_map(|&outs| -> Vec<u8> { outs.iter().cloned().collect() })
        .filter(|&uout| -> bool {
            Signal::len(uout) == 2
                || Signal::len(uout) == 3
                || Signal::len(uout) == 4
                || Signal::len(uout) == 7
        })
        .collect();

    assert_eq!(456, selected.len(), "expect number of 1s, 4s, 7s, and 8s");
}

#[test]
fn day08part2() {
    let inputs = read();
    assert_eq!(200, inputs.len(), "expect number of inputs");

    let decoded: Vec<usize> = inputs
        .iter()
        .cloned()
        .map(|input| Signal::decode(input))
        .collect();

    assert_eq!(200, decoded.len(), "expect number of decoded inputs");

    let sum: usize = decoded.iter().fold(0, |a, v| a + v);
    assert_eq!(1091609, sum, "expect sum of decoded outputs");
}
