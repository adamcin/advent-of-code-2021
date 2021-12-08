#[cfg(test)]
mod day08test {
    use regex::Regex;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum SignalWire {
        A = 1,
        B = 2,
        C = 4,
        D = 8,
        E = 16,
        F = 32,
        G = 64,
    }

    impl FromStr for SignalWire {
        type Err = ();

        fn from_str(input: &str) -> Result<SignalWire, Self::Err> {
            match input {
                "a" => Ok(SignalWire::A),
                "b" => Ok(SignalWire::B),
                "c" => Ok(SignalWire::C),
                "d" => Ok(SignalWire::D),
                "e" => Ok(SignalWire::E),
                "f" => Ok(SignalWire::F),
                "g" => Ok(SignalWire::G),
                _ => Err(()),
            }
        }
    }

    type PackedSignalSets = [u8; 10];
    type PackedSignalOuts = [u8; 4];
    type SignalsInput = (PackedSignalSets, PackedSignalOuts);

    impl SignalWire {
        pub fn iterator() -> impl Iterator<Item = SignalWire> {
            [
                SignalWire::A,
                SignalWire::B,
                SignalWire::C,
                SignalWire::D,
                SignalWire::E,
                SignalWire::F,
                SignalWire::G,
            ]
            .iter()
            .copied()
        }

        pub fn pack(as_vec: &Vec<SignalWire>) -> u8 {
            let mut int: u8 = 0;
            for &wire in as_vec {
                int |= wire as u8;
            }
            return int;
        }

        pub fn unpack(as_int: &u8) -> Vec<SignalWire> {
            let mut wires: Vec<SignalWire> = Vec::new();
            for wire in SignalWire::iterator() {
                if *as_int & (wire as u8) == wire as u8 {
                    wires.push(wire);
                }
            }
            return wires;
        }

        pub fn parse(input: &str) -> Vec<SignalWire> {
            let re = Regex::new(r"^[a-g]+$").unwrap();
            assert!(re.is_match(input), "expect lower alpha a-g only");
            return input.split("").flat_map(|c| c.parse().ok()).collect();
        }

        pub fn parse_line(line: &str) -> Option<SignalsInput> {
            if let Some((sets_s, outs_s)) = line.split_once(" | ") {
                let mut sets: PackedSignalSets = [0; 10];
                for (index, sig) in sets_s
                    .trim()
                    .split(' ')
                    .map(|sig_s| SignalWire::parse(sig_s))
                    .map(|sig_v| SignalWire::pack(&sig_v))
                    .take(10)
                    .enumerate()
                {
                    sets[index] = sig;
                }

                let mut outs: PackedSignalOuts = [0; 4];
                for (index, sig) in outs_s
                    .trim()
                    .split(' ')
                    .map(|sig_s| SignalWire::parse(sig_s))
                    .map(|sig_v| SignalWire::pack(&sig_v))
                    .take(4)
                    .enumerate()
                {
                    outs[index] = sig;
                }

                return Some((sets, outs));
            }
            return None;
        }

        fn to_signal(raw: u8) -> Option<SignalWire> {
            for sig in SignalWire::iterator() {
                if raw == sig as u8 {
                    return Some(sig);
                }
            }
            None
        }

        fn decode_top(encoder: &[Option<u8>; 10]) -> Option<SignalWire> {
            if let (Some(one), Some(seven)) = (encoder[1], encoder[7]) {
                return SignalWire::to_signal(seven - one);
            }
            None
        }

        fn decode_bottom_left(encoder: &[Option<u8>; 10]) -> Option<SignalWire> {
            if let (Some(one), Some(seven)) = (encoder[1], encoder[7]) {
                return SignalWire::to_signal(seven - one);
            }
            None
        }

        fn decode(input: SignalsInput) -> usize {
            let (sets, outs) = input;
            let decoder = SignalWire::make_decoder(sets);
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
            let mut encoder: [Option<u8>; 10] = [None; 10];
            let mut usets: Vec<Vec<SignalWire>> = sets
                .iter()
                .cloned()
                .map(|set| SignalWire::unpack(&set))
                .collect();
            let mut usets_i: Vec<usize> = Vec::new();
            for (index, sets_v) in usets.iter().enumerate() {
                let val_o: Option<usize> = match sets_v.len() {
                    2 => Some(1),
                    3 => Some(7),
                    4 => Some(4),
                    7 => Some(8),
                    _ => None,
                };
                if let Some(val) = val_o {
                    let packed = SignalWire::pack(&sets_v);
                    encoder[val] = Some(packed);
                    usets_i.push(index);
                }
            }
            usets_i.sort();
            for &uset_i in usets_i.iter().rev() {
                //usets.remove(uset_i);
            }

            assert_ne!(None, encoder[1], "expect 1 decoded");
            assert_ne!(None, encoder[4], "expect 4 decoded");
            assert_ne!(None, encoder[7], "expect 7 decoded");
            assert_ne!(None, encoder[8], "expect 8 decoded");

            let top = SignalWire::decode_top(&encoder);
            assert_ne!(None, top, "expect top wire decoded");

            let mut bottom_right: Option<SignalWire> = None;
            let mut top_right: Option<SignalWire> = None;
            let mut top_left: Option<SignalWire> = None;
            let mut bottom: Option<SignalWire> = None;
            let mut middle: Option<SignalWire> = None;
            
            usets_i = Vec::new();
            for (index, sets_v) in usets.iter().enumerate() {
                match sets_v.len() {
                    6 => {
                        // could be 0, 6, or 9
                        if let (Some(one), Some(eight)) = (encoder[1], encoder[8]) {
                            let packed = SignalWire::pack(sets_v);
                            let masked = one & packed;
                            let unpacked = SignalWire::unpack(&masked);
                            if unpacked.len() == 1 {
                                // this is a six
                                bottom_right = Some(unpacked[0]);
                                let packed_top_right = one - (unpacked[0] as u8);
                                let unpacked_top_right = SignalWire::unpack(&packed_top_right);
                                assert_eq!(
                                    1,
                                    unpacked_top_right.len(),
                                    "expect unpacked_top_right to be len 1"
                                );
                                top_right = Some(unpacked_top_right[0]);
                                encoder[6] = Some(packed);
                                usets_i.push(index);
                            }
                        }
                    }
                    5 => {
                        // could be 2, 3, or 5
                        if let (Some(one), Some(four), Some(eight), Some(utop)) =
                            (encoder[1], encoder[4], encoder[8], top.map(|s| s as u8))
                        {
                            let packed = SignalWire::pack(sets_v);
                            let mask_one = packed | one;

                            if packed == mask_one { // this is a three
                                encoder[3] = Some(packed);
                                usets_i.push(index);
                                let packed_bottom = (packed | four) - four - utop;
                                let unpacked_bottom = SignalWire::unpack(&packed_bottom);
                                assert_eq!(
                                    1,
                                    unpacked_bottom.len(),
                                    "expect unpacked_bottom to be len 1"
                                );
                                bottom = Some(unpacked_bottom[0]);

                                let packed_top_left = (packed | four) - packed;
                                let unpacked_top_left = SignalWire::unpack(&packed_top_left);
                                assert_eq!(
                                    1,
                                    unpacked_top_left.len(),
                                    "expect unpacked_top_left to be len 1"
                                );
                                top_left = Some(unpacked_top_left[0]);

                                let packed_middle = four - packed_top_left - one;
                                let unpacked_middle = SignalWire::unpack(&packed_middle);
                                assert_eq!(
                                    1,
                                    unpacked_middle.len(),
                                    "expect unpacked_middle to be len 1"
                                );
                                middle = Some(unpacked_middle[0]);
                            }
                        }
                    }
                    _ => {},
                };
            }
            usets_i.sort();
            for &uset_i in usets_i.iter().rev() {
                usets.remove(uset_i);
            }

            assert_ne!(None, bottom_right, "expect bottom_right wire decoded");
            assert_ne!(None, top_left, "expect top_left wire decoded");
            assert_ne!(None, top_right, "expect top_right wire decoded");
            assert_ne!(None, middle, "expect middle wire decoded");
            assert_ne!(None, bottom, "expect bottom wire decoded");

            let mut bottom_left: Option<SignalWire> = None;
            if let (Some(eight), Some(utop), Some(utop_left), Some(utop_right), Some(umiddle), Some(ubottom), Some(ubottom_right)) = 
            (encoder[8], top, top_left, top_right, middle, bottom, bottom_right) {
                let packed_bottom_left = eight - (utop as u8 | utop_left as u8 | utop_right as u8 | umiddle as u8 | ubottom as u8 | ubottom_right as u8);
                let unpacked_bottom_left = SignalWire::unpack(&packed_bottom_left);
                assert_eq!(
                    1,
                    unpacked_bottom_left.len(),
                    "expect unpacked_bottom_left to be len 1"
                );
                bottom_left = Some(unpacked_bottom_left[0]);

                encoder[9] = Some(eight - packed_bottom_left);
                encoder[0] = Some(eight - (umiddle as u8));
                encoder[5] = Some(eight - packed_bottom_left - (utop_right as u8));
                encoder[2] = Some(eight - (utop_left as u8) - (ubottom_right as u8));
            }

            assert_ne!(None, bottom_left, "expect bottom_left wire decoded");

            let mut decoder: HashMap<u8, usize> = HashMap::new();
            for (index, &sig) in encoder.iter().enumerate() {
                assert_ne!(None, sig, "encoder should have some sig in index {}", index);
                if let Some(value) = sig {
                    decoder.insert(value, index);
                }
            }
            
            return decoder;
        }
    }

    fn day08read(filename: &str) -> Vec<(PackedSignalSets, PackedSignalOuts)> {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().flat_map(|line_r| line_r.ok()).collect();

        let pairs: Vec<SignalsInput> = lines
            .iter()
            .flat_map(|line| SignalWire::parse_line(line))
            .clone()
            .collect();
        pairs
    }

    #[test]
    fn day08test_unpack() {
        let one: u8 = 1;
        let one_unpacked = SignalWire::unpack(&one);
        assert_eq!(1, one_unpacked.len(), "expect one for one");
        assert_eq!(vec!{SignalWire::A}, one_unpacked, "expect A for one");
        assert_eq!(1, SignalWire::pack(&one_unpacked), "expect same for pack 1");
        let two: u8 = 2;
        let two_unpacked = SignalWire::unpack(&two);
        assert_eq!(1, two_unpacked.len(), "expect one for two");
        assert_eq!(vec!{SignalWire::B}, two_unpacked, "expect B for two");
        assert_eq!(2, SignalWire::pack(&two_unpacked), "expect same for pack 2");
        let three: u8 = 3;
        let three_unpacked = SignalWire::unpack(&three);
        assert_eq!(2, three_unpacked.len(), "expect two for three");
        assert_eq!(vec!{SignalWire::A,SignalWire::B}, three_unpacked, "expect B,A for three");
        assert_eq!(3, SignalWire::pack(&three_unpacked), "expect same for pack 3");
    }

    #[test]
    fn day08test_three_from_one() {
        let one_unpacked = vec!{SignalWire::C, SignalWire::F};
        let two_unpacked = vec!{SignalWire::A, SignalWire::C, SignalWire::D, SignalWire::E, SignalWire::G};
        let three_unpacked = vec!{SignalWire::A, SignalWire::C, SignalWire::D, SignalWire::F, SignalWire::G};

        let one = SignalWire::pack(&one_unpacked);
        let two = SignalWire::pack(&two_unpacked);
        let three = SignalWire::pack(&three_unpacked);

        assert_ne!(two, two | one, "expect two bitor one to not equal two");
        assert_eq!(three, three | one, "expect three bitor one to equal three");
    }

    #[test]
    fn day08part1() {
        let inputs = day08read("data/day-08/input.txt");
        assert_eq!(200, inputs.len(), "expect number of inputs");

        let selected: Vec<u8> = inputs
            .iter()
            .map(|(_, outs)| outs)
            .flat_map(|&outs| -> Vec<Vec<SignalWire>> {
                outs.iter().map(|sig| SignalWire::unpack(sig)).collect()
            })
            .filter(|uout: &Vec<SignalWire>| -> bool {
                uout.len() == 2 || uout.len() == 3 || uout.len() == 4 || uout.len() == 7
            })
            .map(|uout| SignalWire::pack(&uout))
            .collect();

        assert_eq!(456, selected.len(), "expect number of 1s, 4s, 7s, and 8s");
    }

    #[test]
    fn day08part2() {
        let inputs = day08read("data/day-08/input.txt");
        assert_eq!(200, inputs.len(), "expect number of inputs");

        let decoded: Vec<usize> = inputs
            .iter()
            .cloned()
            .map(|input| SignalWire::decode(input))
            .collect();

        assert_eq!(200, decoded.len(), "expect number of decoded inputs");

        let sum: usize = decoded.iter().fold(0, |a,v| a+v);
        assert_eq!(1091609, sum, "expect sum of decoded outputs");

    }
}
