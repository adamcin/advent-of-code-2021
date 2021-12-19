mod common;
use std::u8;
type Datum = u8;
/// --- Day 16: Packet Decoder ---
///
/// As you leave the cave and reach open waters, you receive a transmission from the Elves back on the
/// ship.
///
/// The transmission was sent using the Buoyancy Interchange Transmission System (BITS), a method of
/// packing numeric expressions into a binary sequence. Your submarine's computer has saved the
/// transmission in hexadecimal (your puzzle input).
fn read() -> Vec<Datum> {
    return common::read_test_input("data/day-16/input.txt")
        .iter()
        .flat_map(|line| read_datums(line))
        .collect();
}

fn read_datums(text: &String) -> Vec<Datum> {
    return (0..(text.len() / 2))
        .map(|x| x * 2)
        .flat_map(|i| u8::from_str_radix(&text[i..=i + 1], 16).ok())
        .collect();
}

/// The first step of decoding the message is to convert the hexadecimal representation into binary. Each character of hexadecimal corresponds to four bits of binary data:
///
/// 0 = 0000
/// 1 = 0001
/// 2 = 0010
/// 3 = 0011
/// 4 = 0100
/// 5 = 0101
/// 6 = 0110
/// 7 = 0111
/// 8 = 1000
/// 9 = 1001
/// A = 1010
/// B = 1011
/// C = 1100
/// D = 1101
/// E = 1110
/// F = 1111
///
/// The BITS transmission contains a single packet at its outermost layer which itself contains
/// many other packets. The hexadecimal representation of this packet might encode a few extra
/// 0 bits at the end; these are not part of the transmission and should be ignored.
///
/// Every packet begins with a standard header: the first three bits encode the packet version,
/// and the next three bits encode the packet type ID. These two values are numbers; all numbers
/// encoded in any packet are represented as binary with the most significant bit first. For example,
/// a version encoded as the binary sequence 100 represents the number 4.
///
/// Packets with type ID 4 represent a literal value. Literal value packets encode a single binary number.
/// To do this, the binary number is padded with leading zeroes until its length is a multiple of
/// four bits, and then it is broken into groups of four bits. Each group is prefixed by a 1 bit
/// except the last group, which is prefixed by a 0 bit. These groups of five bits immediately follow
/// the packet header. For example, the hexadecimal string D2FE28 becomes:
///
/// 110100101111111000101000
/// VVVTTTAAAAABBBBBCCCCC
/// Below each bit is a label indicating its purpose:
///
/// The three bits labeled V (110) are the packet version, 6.
/// The three bits labeled T (100) are the packet type ID, 4, which means the packet is a literal value.
/// The five bits labeled A (10111) start with a 1 (not the last group, keep reading) and contain the
/// first four bits of the number, 0111.
/// The five bits labeled B (11110) start with a 1 (not the last group, keep reading) and contain four
/// more bits of the number, 1110.
/// The five bits labeled C (00101) start with a 0 (last group, end of packet) and contain the last four
/// bits of the number, 0101.
/// The three unlabeled 0 bits at the end are extra due to the hexadecimal representation and should be
/// ignored.
/// So, this packet represents a literal value with binary representation 011111100101, which is 2021 in
/// decimal.
///
/// Every other type of packet (any packet with a type ID other than 4) represent an operator that
/// performs some calculation on one or more sub-packets contained within. Right now, the specific
/// operations aren't important; focus on parsing the hierarchy of sub-packets.
///
/// 3 datums for literal
/// 7 datums for

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    full: bool,
    head: Datum,
    data: Vec<Datum>,
    subs: Vec<Packet>,
    lval: usize,
}

fn fslice(datums: &[Datum]) -> String {
    return datums
        .iter()
        .map(|datum| format!("{:08b}.", datum))
        .collect();
}

///
/// An operator packet contains one or more packets. To indicate which subsequent binary data represents
/// its sub-packets, an operator packet can use one of two modes indicated by the bit immediately after
/// the packet header; this is called the length type ID:
///
/// If the length type ID is 0, then the next 15 bits are a number that represents the total length in
/// bits of the sub-packets contained by this packet.
/// If the length type ID is 1, then the next 11 bits are a number that represents the number of
/// sub-packets immediately contained by this packet.
/// Finally, after the length type ID bit and the 15-bit or 11-bit field, the sub-packets appear.
///
/// For example, here is an operator packet (hexadecimal string 38006F45291200) with length type ID 0
/// that contains two sub-packets:
///
/// 00111000000000000110111101000101001010010001001000000000
/// VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
/// The three bits labeled V (001) are the packet version, 1.
/// The three bits labeled T (110) are the packet type ID, 6, which means the packet is an operator.
/// The bit labeled I (0) is the length type ID, which indicates that the length is a 15-bit number
/// representing the number of bits in the sub-packets.
/// The 15 bits labeled L (000000000011011) contain the length of the sub-packets in bits, 27.
/// The 11 bits labeled A contain the first sub-packet, a literal value representing the number 10.
/// The 16 bits labeled B contain the second sub-packet, a literal value representing the number 20.
/// After reading 11 and 16 bits of sub-packet data, the total length indicated in L (27) is reached,
/// and so parsing of this packet stops.
///
/// As another example, here is an operator packet (hexadecimal string EE00D40C823060) with
/// length type ID 1 that contains three sub-packets:
///
/// 11101110000000001101010000001100100000100011000001100000
/// VVVTTTILLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBCCCCCCCCCCC
/// The three bits labeled V (111) are the packet version, 7.
/// The three bits labeled T (011) are the packet type ID, 3, which means the packet is an operator.
/// The bit labeled I (1) is the length type ID, which indicates that the length is a 11-bit number
/// representing the number of sub-packets.
/// The 11 bits labeled L (00000000011) contain the number of sub-packets, 3.
/// The 11 bits labeled A contain the first sub-packet, a literal value representing the number 1.
/// The 11 bits labeled B contain the second sub-packet, a literal value representing the number 2.
/// The 11 bits labeled C contain the third sub-packet, a literal value representing the number 3.
/// After reading 3 complete sub-packets, the number of sub-packets indicated in L (3) is reached,
/// and so parsing of this packet stops.

/// For now, parse the hierarchy of the packets throughout the transmission and add up all of the
/// version numbers.
///
/// Here are a few more examples of hexadecimal-encoded transmissions:
///
/// 8A004A801A8002F478 represents an operator packet (version 4) which contains an operator packet
/// (version 1) which contains an operator packet (version 5) which contains a literal value (version 6);
/// this packet has a version sum of 16.
/// 620080001611562C8802118E34 represents an operator packet (version 3) which contains two sub-packets;
/// each sub-packet is an operator packet that contains two literal values. This packet has a version
/// sum of 12.
/// C0015000016115A2E0802F182340 has the same structure as the previous example, but the outermost
/// packet uses a different length type ID. This packet has a version sum of 23.
/// A0016C880162017C3686B18A3D4780 is an operator packet that contains an operator packet that contains
/// an operator packet that contains five literal values; it has a version sum of 31.
///
/// Decode the structure of your hexadecimal-encoded BITS transmission; what do you get if you add up
/// the version numbers in all packets?

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Literal {
        version: usize,
        value: usize,
    },
    Sum {
        version: usize,
        subs: Vec<Expr>,
    },
    Product {
        version: usize,
        subs: Vec<Expr>,
    },
    Min {
        version: usize,
        subs: Vec<Expr>,
    },
    Max {
        version: usize,
        subs: Vec<Expr>,
    },
    Greater {
        version: usize,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Less {
        version: usize,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Equal {
        version: usize,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}
use std::cmp;
use Expr::*;

impl Expr {
    
    fn get_version(&self) -> usize {
        return match self {
            Literal { version, .. }
            | Sum { version, .. }
            | Product { version, .. }
            | Min { version, .. }
            | Max { version, .. }
            | Greater { version, .. }
            | Less { version, .. }
            | Equal { version, .. } => *version,
            _ => 0,
        };
    }

    fn get_versions(&self) -> Vec<usize> {
        return match self {
            Literal { .. } => vec![self.get_version()],
            Sum { subs, .. } | Product { subs, .. } | Min { subs, .. } | Max { subs, .. } => vec![
                vec![self.get_version()],
                subs.iter().flat_map(|sub| sub.get_versions()).collect(),
            ]
            .concat(),
            Greater { left, right, .. } | Less { left, right, .. } | Equal { left, right, .. } => {
                vec![
                    vec![self.get_version()],
                    left.get_versions(),
                    right.get_versions(),
                ]
                .concat()
            }
        };
    }

    fn count(&self) -> usize {
        return match self {
            Literal { .. } => 1,
            Greater { left, right, .. } | Less { left, right, .. } | Equal { left, right, .. } => {
                1 + left.count() + right.count()
            }
            Sum { subs, .. } | Product { subs, .. } | Min { subs, .. } | Max { subs, .. } => {
                subs.iter().fold(1, |a, v| a + v.count())
            }
        };
    }

    fn value(&self) -> usize {
        return match self {
            Literal { value, .. } => *value,
            Sum { subs, .. } => subs.iter().fold(0, |a, v| a + v.value()),
            Product { subs, .. } => subs.iter().fold(1, |a, v| a * v.value()),
            Min { subs, .. } => subs.iter().fold(usize::MAX, |a, v| cmp::min(a, v.value())),
            Max { subs, .. } => subs.iter().fold(0, |a, v| cmp::max(a, v.value())),
            Greater { left, right, .. } => {
                if left.value() > right.value() {
                    1
                } else {
                    0
                }
            }
            Less { left, right, .. } => {
                if left.value() < right.value() {
                    1
                } else {
                    0
                }
            }
            Equal { left, right, .. } => {
                if left.value() == right.value() {
                    1
                } else {
                    0
                }
            }
        };
    }
}

///
/// Sum { subs: [
///     Literal { value: 2556 },
///     Max { subs: [
///         Literal { value: 402314 },
///         Literal { value: 15 },
///         Literal { value: 8 },
///         Literal { value: 9 }
///     ] },
///     Product { subs: [
///         Literal { value: 37741 },
///         Greater { left:
///             Literal { value: 287506942933 }, right:
///             Literal { value: 127 }
///         }]
///     }]
/// }
///

#[derive(Debug, Clone)]
enum DataHead {
    One { value: usize },
    Three { value: usize },
    Four { value: usize },
    Eleven { value: usize },
    Fifteen { value: usize },
    Arb { data: Vec<Datum>, width: usize },
}

use DataHead::*;
impl DataHead {
    fn get_width(&self) -> usize {
        return match self {
            One { .. } => 1,
            Three { .. } => 3,
            Four { .. } => 4,
            Eleven { .. } => 11,
            Fifteen { .. } => 15,
            Arb { width, .. } => *width,
        };
    }

    fn get_value(&self) -> usize {
        return match self {
            One { value, .. }
            | Three { value, .. }
            | Four { value, .. }
            | Eleven { value, .. }
            | Fifteen { value, .. } => *value,
            _ => 0,
        };
    }

    fn usize_to_datums(value: usize, width: usize) -> Vec<Datum> {
        let umax = u8::MAX as usize;
        let mut new_data: Vec<Datum> = Vec::new();
        let mut value_rem = value;
        let mut width_rem = width;
        let carry = width_rem % 8;
        if carry > 0 {
            let last_datum: Datum = ((value_rem << (8 - carry)) & umax) as Datum;
            new_data.insert(0, last_datum);
            value_rem = value_rem >> carry;
            width_rem -= carry;
        }
        if width / 8 > 0 {
            while width_rem > 0 {
                let last_datum: Datum = (value_rem & umax) as Datum;
                new_data.insert(0, last_datum);
                value_rem = value_rem >> 8;
                width_rem -= 8;
            }
        }
        return new_data;
    }

    fn to_stream(&self) -> DataStream {
        let new_data = match self {
            Arb { data, .. } => data.to_owned(),
            _ => Self::usize_to_datums(self.get_value(), self.get_width()),
        };
        return DataStream::new(new_data);
    }

    fn new(value: usize, width: usize) -> DataHead {
        assert!(width > 0, "width must be greater than zero");
        return match width {
            1 => One { value: value },
            3 => Three { value: value },
            4 => Four { value: value },
            11 => Eleven { value: value },
            15 => Fifteen { value: value },
            _ => Arb {
                data: Self::usize_to_datums(value, width),
                width: width,
            },
        };
    }
}

#[derive(Debug)]
struct DataStream {
    data: Vec<Datum>,
}

#[derive(Debug, Copy, Clone)]
struct Head {
    version: usize,
    type_id: usize,
}

impl Head {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DataIdx {
    datum: usize,
    skip: usize,
}

impl DataStream {
    fn new(data: Vec<Datum>) -> Self {
        return Self { data: data };
    }

    fn tail<'s>(&'s self) -> Option<DataTail<'s>> {
        if !self.data.is_empty() {
            return Some(DataTail {
                stream: self,
                index: DataIdx { skip: 0, datum: 0 },
                is_read: false,
            });
        }
        None
    }

    /// bit1: 0-7
    /// width: 1-8
    fn read_i(datum: Datum, bit1: usize, width: usize) -> usize {
        assert!(bit1 <= 7, "invalid bit1: {}", bit1);
        assert!(width > 0 && width <= 8, "invalid width: {}", width);
        assert!(bit1 + width <= 8, "invalid bit1 {} + width {}", bit1, width);

        let result = if bit1 == 0 && width == 8 {
            datum as usize
        } else if bit1 == 0 {
            (datum >> (8 - width)) as usize
        } else {
            ((datum << bit1) >> (8 - width)) as usize
        };
        return result;
    }

    fn substream(&self, idx: &DataIdx, width: usize) -> Option<(DataStream, DataIdx)> {
        if width > 0 {
            let DataIdx { datum, skip } = idx;
            assert!(*skip < 8, "skip should be 0-7: {}", *skip);
            let next_skip = (skip + width) % 8;
            let next_datum = datum + ((skip + width) / 8);
            if self.data.len() >= next_datum + cmp::min(1, next_skip) {
                let mut new_data: Vec<Datum> = Vec::new();

                if width >= (8 - *skip + next_skip) {
                    let upper_bound = if *skip > next_skip {
                        next_datum - 1
                    } else {
                        next_datum
                    };
                    for d in *datum..upper_bound {
                        let value = if *skip > 0 {
                            (self.data[d] << *skip) + (self.data[d + 1] >> (8 - *skip))
                        } else {
                            self.data[d]
                        };
                        new_data.push(value);
                    }
                }

                let value = if *skip > next_skip {
                    if next_skip > 0 {
                        (self.data[next_datum - 1] << *skip)
                            + ((self.data[next_datum] >> (8 - next_skip)) << (*skip - next_skip))
                    } else {
                        self.data[next_datum - 1] << *skip
                    }
                } else if *skip < next_skip {
                    (self.data[next_datum] >> (8 - next_skip)) << (*skip + (8 - next_skip))
                } else {
                    0
                };

                if *skip != next_skip {
                    new_data.push(value);
                }

                let new_idx = DataIdx {
                    datum: next_datum,
                    skip: next_skip,
                };
                assert!(
                    new_data.len() * 8 >= width && new_data.len() * 8 < width + 8,
                    "new_data len {} * 8 should be within 8 of width: {}",
                    new_data.len(),
                    width
                );
                return Some((DataStream::new(new_data), new_idx));
            }
        }
        None
    }

    fn read(&self, idx: &DataIdx, width: usize) -> Option<(usize, DataIdx)> {
        if width > 0 {
            let DataIdx { datum, skip } = idx;
            assert!(*skip < 8, "skip should be 0-7: {}", *skip);
            let next_skip = (skip + width) % 8;
            let next_datum = datum + ((skip + width) / 8);
            if self.data.len() >= next_datum + cmp::min(1, next_skip) {
                let mut value = 0;

                if *skip > 0 {
                    let wide = cmp::min(width, 8 - skip);
                    value = value << wide;
                    value = value + Self::read_i(self.data[*datum], *skip, wide);
                }
                let first_full = if *skip > 0 { *datum + 1 } else { *datum };
                if next_datum > first_full {
                    for d in first_full..next_datum {
                        value = value << 8;
                        value = value + Self::read_i(self.data[d], 0, 8);
                    }
                }
                if (*skip == 0 || next_datum > *datum) && next_skip > 0 {
                    value = value << next_skip;
                    value = value + Self::read_i(self.data[next_datum], 0, next_skip);
                }

                let result = Some((
                    value,
                    DataIdx {
                        datum: next_datum,
                        skip: next_skip,
                    },
                ));
                return result;
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone)]
struct DataTail<'s> {
    stream: &'s DataStream,
    index: DataIdx,
    is_read: bool,
}

const SUM_TYPE: usize = 0;
const PRODUCT_TYPE: usize = 1;
const MIN_TYPE: usize = 2;
const MAX_TYPE: usize = 3;
const LITERAL_TYPE: usize = 4;
const GREATER_TYPE: usize = 5;
const LESS_TYPE: usize = 6;
const EQUAL_TYPE: usize = 7;

const BITS_LENGTH_TYPE: usize = 0;
const SUBS_LENGTH_TYPE: usize = 1;

impl<'s> DataTail<'s> {
    fn substream(&self, width: usize) -> Option<(DataStream, DataTail<'s>)> {
        if let Some((substream, new_idx)) = self.stream.substream(&self.index, width) {
            return Some((
                substream,
                DataTail {
                    stream: self.stream,
                    index: new_idx,
                    is_read: false,
                },
            ));
        }
        None
    }
    fn take(&self, width: usize) -> Option<(DataHead, DataTail<'s>)> {
        if let Some((value, new_idx)) = self.stream.read(&self.index, width) {
            return Some((
                DataHead::new(value, width),
                DataTail {
                    stream: self.stream,
                    index: new_idx,
                    is_read: false,
                },
            ));
        }
        None
    }

    fn read_literal(&self, version: usize) -> Option<(Expr, DataTail<'s>)> {
        let mut full_value = 0;
        let mut full_tail = *self;
        let mut do_read = true;
        while do_read {
            if let Some((One { value: flag }, flag_tail)) = full_tail.take(1) {
                do_read = flag != 0;
                if let Some((Four { value: chunk }, chunk_tail)) = flag_tail.take(4) {
                    full_value = full_value << 4;
                    full_value = full_value + chunk;
                    full_tail = chunk_tail;
                    continue;
                }
            }
            return None; // return none if continue
        }
        return Some((
            Literal {
                version: version,
                value: full_value,
            },
            full_tail,
        ));
    }

    fn read_n_subs(&self, subs: &mut Vec<Expr>, n_subs: usize) -> Option<DataTail<'s>> {
        if n_subs > 0 {
            if let Some((sub, sub_tail)) = self.read_expr() {
                subs.push(sub);
                return sub_tail.read_n_subs(subs, n_subs - 1);
            } else {
                return None;
            }
        }
        return Some(DataTail {
            index: self.index,
            is_read: false,
            stream: self.stream,
        });
    }

    fn on_read_n_subs(&self) -> Option<(Vec<Expr>, DataTail<'s>)> {
        if let Some((Eleven { value: n_subs }, len_tail)) = self.take(11) {
            let mut subs: Vec<Expr> = Vec::new();
            if let Some(subs_tail) = len_tail.read_n_subs(&mut subs, n_subs) {
                return Some((subs, subs_tail));
            }
        }
        None
    }

    fn read_n_bits(&self, subs: &mut Vec<Expr>) {
        if let Some((sub, sub_tail)) = self.read_expr() {
            subs.push(sub);
            sub_tail.read_n_bits(subs);
        }
    }

    fn read_subs_n_bits(&self, n_bits: usize) -> Option<(Vec<Expr>, DataTail<'s>)> {
        if let Some((substream, next_tail)) = self.substream(n_bits) {
            let mut subs: Vec<Expr> = Vec::new();
            if let Some(tail) = substream.tail() {
                tail.read_n_bits(&mut subs);
            }
            return Some((subs, next_tail));
        }
        None
    }

    fn on_read_n_bits(&self) -> Option<(Vec<Expr>, DataTail<'s>)> {
        if let Some((Fifteen { value: n_bits }, len_tail)) = self.take(15) {
            return len_tail.read_subs_n_bits(n_bits);
        }
        None
    }

    fn read_subs(&self, expect_two: bool) -> Option<(Vec<Expr>, DataTail<'s>)> {
        if let Some((One { value: len_id }, len_id_tail)) = self.take(1) {
            if len_id == SUBS_LENGTH_TYPE {
                let n_subs_result = len_id_tail
                    .on_read_n_subs()
                    .filter(|(subs, _)| (expect_two && subs.len() == 2) || subs.len() > 0);
                return n_subs_result;
            } else if len_id == BITS_LENGTH_TYPE {
                return len_id_tail
                    .on_read_n_bits()
                    .filter(|(subs, _)| (expect_two && subs.len() == 2) || subs.len() > 0);
            }
        }
        None
    }

    fn read_operator(&self, version: usize, type_id: usize) -> Option<(Expr, DataTail<'s>)> {
        if let Some((subs, subs_tail)) = self.read_subs(type_id > LITERAL_TYPE) {
            let wrapped = match type_id {
                SUM_TYPE => Some(Sum {
                    version: version,
                    subs: subs,
                }),
                PRODUCT_TYPE => Some(Product {
                    version: version,
                    subs: subs,
                }),
                MIN_TYPE => Some(Min {
                    version: version,
                    subs: subs,
                }),
                MAX_TYPE => Some(Max {
                    version: version,
                    subs: subs,
                }),
                GREATER_TYPE => Some(Greater {
                    version: version,
                    left: Box::new(subs[0].to_owned()),
                    right: Box::new(subs[1].to_owned()),
                }),
                LESS_TYPE => Some(Less {
                    version: version,
                    left: Box::new(subs[0].to_owned()),
                    right: Box::new(subs[1].to_owned()),
                }),
                EQUAL_TYPE => Some(Equal {
                    version: version,
                    left: Box::new(subs[0].to_owned()),
                    right: Box::new(subs[1].to_owned()),
                }),
                _ => None,
            };
            return wrapped.map(|expr| (expr, subs_tail));
        }
        None
    }

    fn read_expr(&self) -> Option<(Expr, DataTail<'s>)> {
        if let Some((Three { value: version }, version_tail)) = self.take(3) {
            if let Some((Three { value: type_id }, type_tail)) = version_tail.take(3) {
                if type_id == LITERAL_TYPE {
                    return type_tail.read_literal(version);
                } else {
                    return type_tail.read_operator(version, type_id);
                }
            }
        }
        None
    }
}

/// 01234567.89012345.67890123
/// 10100000.01010010.11100000.01001100.11111101.10011101.11000000.00100100.10010110.10010100.11110000.10100001.00011110.10100010.00000100.01001110.00100000.00001110.10010010.01100110.01110110.01101010.10110000.00000100.10100101.00100101.11111000.01101111.11111100.11011111.01001011.00100101.11011111.11000100.00000001.10100010.00000000.01000011.10100001.00011100.01100001.10000011.10000110.00000000.11111100.01100111.10001101.01010001.10111000.11000000.00011001.10001001.00010000.11101010.00010010.00000000.00000001.00001011.00111110.11101010.01000000.00100100.01101100.10010111.01001110.11110000.00000011.00110011.00010000.00000110.01100001.10011100.00100110.10000100.01000010.00000000.11010100.00010100.10000101.10010000.01001001.01000000.00101101.10011100.11011010.01100100.10111101.11101111.00111100.01001110.01100010.00110011.00110001.11111011.11001100.10100011.11100100.11011111.10111011.11111100.01111001.11100100.00000000.01001101.11101001.01101111.11000011.10110001.11101100.01101101.11100100.00101001.10001101.01011010.00011100.10001111.10011000.11100100.01010010.01100110.01110100.01011011.00111000.00100000.01000000.00011001.00011101.00000000.00110100.01010011.10010110.10000010.11110100.11100101.10100000.10110101.00100111.11111110.10110000.00011000.00000010.10010010.01110111.11001000.10001110.00000000.00111001.10010011.01111101.10001010.11001100.11000110.00100101.01100000.10010010.00000000.01000001.01100101.11010011.01100101.10000110.11001100.00000001.00111010.00000000.10000110.00100101.10100010.11010111.00111001.01001010.01011011.00011101.11100001.01101100.00001110.00110000.00000100.10101000.00000011.01010010.00000000.00000100.00110010.00100000.11000101.10111000.00111000.00100000.00001110.11000100.10111000.11100011.00010101.10100110.11001110.11100110.11110011.11000011.10111001.11111111.10111000.00010000.00001001.10010100.00100000.00001100.11000101.10011000.00110111.00010000.10000100.00000001.10011000.10011101.00000101.01100010.10000000.10000000.00111111.00011110.10100011.11000100.00010001.00110000.00000100.01110000.00000011.01010011.00000000.00000100.00110010.00111101.11000011.11001000.01100000.00100000.00001110.11000100.00011000.00101111.00011100.10100111.11100100.01010010.11000000.00010111.01000100.10100000.10100100.11111111.01101011.10111010.11100110.10100101.00110011.10111111.11001101.00011001.01100111.10100010.01101110.00100000.00010010.01001011.11100001.10010010.00001010.01001010.01101010.01100001.00110011.00010101.01010001.00010000.00000111.10100100.10100011.00101011.11101001.10101110.01101011.01011100.10101101.00011001.11100101.01101011.10100001.01000011.00000000.01010011.10000000.00110011.01000001.00000000.01111110.00100100.11000001.01101000.10100110.00100000.00001101.01000110.00111000.01000011.00011000.10100110.10101010.11001000.01000000.00011001.00000111.00000000.00111110.11110010.11110111.11010111.00000010.01100101.11101111.10101110.00000100.11001100.10101011.00111000.00000001.01110010.01111100.10011110.11001001.01001000.00000010.10101111.10010010.11110100.10010011.10101000.00000001.00101101.10011110.10101011.10110100.10001011.10100011.10000000.01011101.00011011.01100101.01110101.01100101.01011001.00100011.00011001.00010111.10111001.00111010.01001011.01001011.01000110.00000000.10011100.10010001.11110110.00000000.01001000.00010010.01010100.10101111.01100111.10101000.01000101.10111010.01010110.01100001.00000100.00000000.01000001.01001110.00110000.10010000.00000101.01010101.00100101.11101000.01001001.10111110.10000000.00010000.00111001.01110100.00111001.01110100.01100100.00000000.10111100.00100101.01011110.11100101.00110110.00100001.00110110.11110111.00101011.01001010.01001010.01111011.01110010.00010000.00000100.10100101.00010000.10100111.00110111.00001100.11001011.00110111.11000010.10111010.00000000.00010000.11010011.00000011.10000110.00000000.10111110.10000000.00101001.00110111.10100100.00101001.10111101.00100000.11001001.00001100.11001100.01010110.01001110.11000100.00000001.01000100.11101000.00000010.00010011.11100010.10110011.11100010.11110011.11011001.11010110.11011011.00001000.00000011.11110010.10110000.00000101.10100111.00110001.11011100.01101100.01010010.01001010.00010110.10110101.11110001.11000001.11011001.10001110.11100000.00000110.00110011.10010000.00001001.10101011.01000000.00011010.10110000.10000000.00110001.00001000.10100001.00101100.00101010.00000000.00000100.00111010.00010011.01000010.00101000.10101011.00101101.10111101.10100000.00001000.00000001.11101100.00000110.00011011.00001000.00000001.10000000.00000101.01111010.10001000.00000001.01100100.00000100.11011010.00100000.00010010.00000110.10100000.00000110.00111000.00000001.01001110.00000000.01001001.10000000.00011110.11000000.00110000.10011000.00000000.10101100.00100000.00000010.01011011.00100000.00001000.00001100.01100000.00000111.00010000.00000101.10001010.01100000.00000111.00000000.00000011.00001000.00000000.00000110.10100100.11110101.01100110.00100100.01000000.00010010.11000100.10110010.00000100.10101000.00111100.10110010.00110100.11000010.00100100.01000001.00100000.00001000.00001110.01100101.01100010.01000100.01100110.01101001.00000010.01011100.11010100.10000000.00101101.10101001.10100100.01011111.00000000.01000110.01011000.01010010.01111111.11111110.11000111.00100000.10010000.01100000.00001000.11001001.10010110.01110000.00000011.10010111.00110001.10011101.11010111.01110001.00000101.10010110.01100111.01000000.00000100.10111110.01101010.00010110.00010010.10000011.10110000.10011100.10000000.00101011.00001101.00000000.01000110.00111010.11001001.01010110.00111100.00101011.10010110.10011111.00001110.00001000.00000001.10000010.10010111.00101110.10011000.00101111.10010111.00011000.00100000.00001101.00101110.01100011.01111101.10110001.01100110.00000000.00110100.00010010.10010010.11010110.11011000.10100111.11110100.10010110.10000000.00001111.11010100.10010000.10111100.11011100.01101000.10110011.00111001.01110110.10101000.01110010.11100000.00001000.11000101.11111001.11011111.11010101.01100110.01001001.00001010.00010100.
/// 10100000.01010010.11100000
/// .  .  ..                .           
///                         00010011001111110110011101110000000010010010010110100101001111000010100001000111101010001000000100010011100010000000001110100100100110011001110110011010101011000000000100101001010010010111111000011011111111110011011111010010110010010111011111110001000000000110100010000000000100001110100001000111000110000110000011100001100000000011111100011001111000110101010001101110001100000000011001100010010001000011101010000100100000000000000001000010110011111011101010010000000010010001101100100101110100111011110000000000110011001100010000000001100110000110011100001001101000010001000010000000001101010000010100100001011001000001001001010000000010110110011100110110100110010010111101111011110011110001001110011000100011001100110001111110111100110010100011111001001101111110111011111111000111100111100100000000000100110111101001011011111100001110110001111011000110110111100100001010011000110101011010000111001000111110011000111001000101001001100110011101000101101100111000001000000100000000011001000111010000000000110100010100111001011010000010111101001110010110100000101101010010011111111110101100000001100000000010100100100111011111001000100011100000000000111001100100110111110110001010110011001100011000100101011000001001001000000000010000010110010111010011011001011000011011001100000000010011101000000000100001100010010110100010110101110011100101001010010110110001110111100001011011000000111000110000000001001010100000000011010100100000000000000100001100100010000011000101101110000011100000100000000011101100010010111000111000110001010110100110110011101110011011110011110000111011100111111111101110000001000000001001100101000010000000001100110001011001100000110111000100001000010000000001100110001001110100000101011000101000000010000000001111110001111010100011110001000001000100110000000001000111000000000011010100110000000000000100001100100011110111000011110010000110000000100000000011101100010000011000001011110001110010100111111001000101001011000000000101110100010010100000101001001111111101101011101110101110011010100101001100111011111111001101000110010110011110100010011011100010000000010010010010111110000110010010000010100100101001101010011000010011001100010101010100010001000000000111101001001010001100101011111010011010111001101011010111001010110100011001111001010110101110100001010000110000000001010011100000000011001101000001000000000111111000100100110000010110100010100110001000000000110101000110001110000100001100011000101001101010101011001000010000000001100100000111000000000011111011110010111101111101011100000010011001011110111110101110000001001100110010101011001110000000000101110010011111001001111011001001010010000000001010101111100100101111010010010011101010000000000100101101100111101010101110110100100010111010001110000000010111010001101101100101011101010110010101011001001000110001100100010111101110010011101001001011010010110100011000000000100111001001000111110110000000000100100000010010010101001010111101100111101010000100010110111010010101100110000100000100000000000100000101001110001100001001000000000101010101010010010111101000010010011011111010000000000100000011100101110100001110010111010001100100000000001011110000100101010111101110010100110110001000010011011011110111001010110100101001001010011110110111001000010000000001001010010100010000101001110011011100001100110010110011011111000010101110100000000000010000110100110000001110000110000000001011111010000000001010010011011110100100001010011011110100100000110010010000110011001100010101100100111011000100000000010100010011101000000000100001001111100010101100111110001011110011110110011101011011011011000010000000001111110010101100000000010110100111001100011101110001101100010100100100101000010110101101011111000111000001110110011000111011100000000001100011001110010000000010011010101101000000000110101011000010000000001100010000100010100001001011000010101000000000000001000011101000010011010000100010100010101011001011011011110110100000000010000000000111101100000001100001101100001000000000011000000000000101011110101000100000000001011001000000010011011010001000000001001000000110101000000000011000111000000000010100111000000000010010011000000000011110110000000011000010011000000000001010110000100000000000100101101100100000000010000000110001100000000001110001000000000101100010100110000000000111000000000000001100001000000000000000011010100100111101010110011000100100010000000001001011000100101100100000010010101000001111001011001000110100110000100010010001000001001000000000100000001110011001010110001001000100011001100110100100000010010111001101010010000000001011011010100110100100010111110000000001000110010110000101001001111111111111101100011100100000100100000110000000001000110010011001011001110000000000111001011100110001100111011101011101110001000001011001011001100111010000000000010010111110011010100001011000010010100000111011000010011100100000000010101100001101000000000100011000111010110010010101011000111100001010111001011010011111000011100000100000000001100000101001011100101110100110000010111110010111000110000010000000001101001011100110001101111101101100010110011000000000001101000001001010010010110101101101100010100111111101001001011010000000000011111101010010010000101111001101110001101000101100110011100101110110101010000111001011100000000010001100010111111001110111111101010101100110010010010000101000010100
///                         0001001100111111011001110111000000001001001001011010010100111100001010000100011110101000100000010001001110001000000000111010010010011001100111011001101010101100000000010010100101001001011111100001101111111111001101111101001011001001011101111111000100000000011010001000000000010000111010000100011100011000011000001110000110000000001111110001100111100011010101000110111000110000000001100110001001000100001110101000010010000000000000000100001011001111101110101001000000001001000110110010010111010011101111000000000011001100110001000000000110011000011001110000100110100001000100001000000000110101000001010010000101100100000100100101000000001011011001110011011010011001001011110111101111001111000100111001100010001100110011000111111011110011001010001111100100110111111011101111111100011110011110010000000000010011011110100101101111110000111011000111101100011011011110010000101001100011010101101000011100100011111001100011100100010100100110011001110100010110110011100000100000010000000001100100011101000000000011010001010011100101101000001011110100111001011010000010110101001001111111111010110000000110000000001010010010011101111100100010001110000000000011100110010011011111011000101011001100110001100010010101100000100100100000000001000001011001011101001101100101100001101100110000000001001110100000000010000110001001011010001011010111001110010100101001011011000111011110000101101100000011100011000000000100101010000000001101010010000000000000010000110010001000001100010110111000001110000010000000001110110001001011100011100011000101011010011011001110111001101111001111000011101110011111111110111000000100000000100110010100001000000000110011000101100110000011011100010000100001000000000110011000100111010000010101100010100000001000000000111111000111101010001111000100000100010011000000000100011100000000001101010011000000000000010000110010001111011100001111001000011000000010000000001110110001000001100000101111000111001010011111100100010100101100000000010111010001001010000010100100111111110110101110111010111001101010010100110011101111111100110100011001011001111010001001101110001000000001001001001011111000011001001000001010010010100110101001100001001100110001010101010001000100000000011110100100101000110010101111101001101011100110101101011100101011010001100111100101011010111010000101000011000000000101001110000000001100110100000100000000011111100010010011000001011010001010011000100000000011010100011000111000010000110001100010100110101010101100100001000000000110010000011100000000001111101111001011110111110101110000001001100101111011111010111000000100110011001010101100111000000000010111001001111100100111101100100101001000000000101010111110010010111101001001001110101000000000010010110110011110101010111011010010001011101000111000000001011101000110110110010101110101011001010101100100100011000110010001011110111001001110100100101101001011010001100000000010011100100100011111011000000000010010000001001001010100101011110110011110101000010001011011101001010110011000010000010000000000010000010100111000110000100100000000010101010101001001011110100001001001101111101000000000010000001110010111010000111001011101000110010000000000101111000010010101011110111001010011011000100001001101101111011100101011010010100100101001111011011100100001000000000100101001010001000010100111001101110000110011001011001101111100001010111010000000000001000011010011000000111000011000000000101111101000000000101001001101111010010000101001101111010010000011001001000011001100110001010110010011101100010000000001010001001110100000000010000100111110001010110011111000101111001111011001110101101101101100001000000000111111001010110000000001011010011100110001110111000110110001010010010010100001011010110101111100011100000111011001100011101110000000000110001100111001000000001001101010110100000000011010101100001000000000110001000010001010000100101100001010100000000000000100001110100001001101000010001010001010101100101101101111011010000000001000000000011110110000000110000110110000100000000001100000000000010101111010100010000000000101100100000001001101101000100000000100100000011010100000000001100011100000000001010011100000000001001001100000000001111011000000001100001001100000000000101011000010000000000010010110110010000000001000000011000110000000000111000100000000010110001010011000000000011100000000000000110000100000000000000001101010010011110101011001100010010001000000000100101100010010110010000001001010100000111100101100100011010011000010001001000100000100100000000010000000111001100101011000100100010001100110011010010000001001011100110101001000000000101101101010011010010001011111000000000100011001011000010100100111111111111110110001110010000010010000011000000000100011001001100101100111000000000011100101110011000110011101110101110111000100000101100101100110011101000000000001001011111001101010000101100001001010000011101100001001110010000000001010110000110100000000010001100011101011001001010101100011110000101011100101101001111100001110000010000000000110000010100101110010111010011000001011111001011100011000001000000000110100101110011000110111110110110001011001100000000000110100000100101001001011010110110110001010011111110100100101101000000000001111110101001001000010111100110111000110100010110011001110010111011010101000011100101110000000001000110001011111100111011111110101010110011001001001000010
///                         00.01001100.11111101.10011101.11000000.00100100.10010110.10010100.11110000.10100001.00011110.101000.10000001.00010011.10001000.00000011.10100100.10011001.10011101.10011010.10101100.00000001.00101001.01001001.01111110.00011011.11111111.00110111.11010010.11001001.01110111.11110001.00000000.01101000.10000000.00010000.11101000.01000111.00011000.01100000.11100001.10000000.00111111.00011001.11100011.01010100.01101110.00110000.00000110.01100010.01000100.00111010.10000100.10000000.00000000.01000010.11001111.10111010.10010000.00001001.00011011.00100101.11010011.10111100.00000000.11001100.11000100.00000001.10011000.01100111.00001001.10100001.00010000.10000000.00110101.00000101.00100001.01100100.00010010.01010000.00001011.01100111.00110110.10011001.00101111.01111011.11001111.00010011.10011000.10001100.11001100.01111110.11110011.00101000.11111001.00110111.11101110.11111111.00011110.01111001.00000000.00010011.01111010.01011011.11110000.11101100.01111011.00011011.01111001.00001010.01100011.01010110.10000111.00100011.11100110.00111001.00010100.10011001.10011101.00010110.11001110.00001000.00010000.00000110.01000111.01000000.00001101.00010100.11100101.10100000.10111101.00111001.01101000.00101101.01001001.11111111.10101100.00000110.00000000.10100100.10011101.11110010.00100011.10000000.00001110.01100100.11011111.01100010.10110011.00110001.10001001.01011000.00100100.10000000.00010000.01011001.01110100.11011001.01100001.10110011.00000000.01001110.10000000.00100001.10001001.01101000.10110101.11001110.01010010.10010110.11000111.01111000.01011011.00000011.10001100.00000001.00101010.00000000.11010100.10000000.00000001.00001100.10001000.00110001.01101110.00001110.00001000.00000011.10110001.00101110.00111000.11000101.01101001.10110011.10111001.10111100.11110000.11101110.01111111.11101110.00000100.00000010.01100101.00001000.00000011.00110001.01100110.00001101.11000100.00100001.00000000.01100110.00100111.01000001.01011000.10100000.00100000.00001111.11000111.10101000.11110001.00000100.01001100.00000001.00011100.00000000.11010100.11000000.00000001.00001100.10001111.01110000.11110010.00011000.00001000.00000011.10110001.00000110.00001011.11000111.00101001.11111001.00010100.10110000.00000101.11010001.00101000.00101001.00111111.11011010.11101110.10111001.10101001.01001100.11101111.11110011.01000110.01011001.11101000.10011011.10001000.00000100.10010010.11111000.01100100.10000010.10010010.10011010.10011000.01001100.11000101.01010100.01000100.00000001.11101001.00101000.11001010.11111010.01101011.10011010.11010111.00101011.01000110.01111001.01011010.11101000.01010000.11000000.00010100.11100000.00001100.11010000.01000000.00011111.10001001.00110000.01011010.00101001.10001000.00000011.01010001.10001110.00010000.11000110.00101001.10101010.10110010.00010000.00000110.01000001.11000000.00001111.10111100.10111101.11110101.11000000.10011001.01111011.11101011.10000001.00110011.00101010.11001110.00000000.01011100.10011111.00100111.10110010.01010010.00000000.10101011.11100100.10111101.00100100.11101010.00000000.01001011.01100111.10101010.11101101.00100010.11101000.11100000.00010111.01000110.11011001.01011101.01011001.01010110.01001000.11000110.01000101.11101110.01001110.10010010.11010010.11010001.10000000.00100111.00100100.01111101.10000000.00010010.00000100.10010101.00101011.11011001.11101010.00010001.01101110.10010101.10011000.01000001.00000000.00010000.01010011.10001100.00100100.00000001.01010101.01001001.01111010.00010010.01101111.10100000.00000100.00001110.01011101.00001110.01011101.00011001.00000000.00101111.00001001.01010111.10111001.01001101.10001000.01001101.10111101.11001010.11010010.10010010.10011110.11011100.10000100.00000001.00101001.01000100.00101001.11001101.11000011.00110010.11001101.11110000.10101110.10000000.00000100.00110100.11000000.11100001.10000000.00101111.10100000.00001010.01001101.11101001.00001010.01101111.01001000.00110010.01000011.00110011.00010101.10010011.10110001.00000000.01010001.00111010.00000000.10000100.11111000.10101100.11111000.10111100.11110110.01110101.10110110.11000010.00000000.11111100.10101100.00000001.01101001.11001100.01110111.00011011.00010100.10010010.10000101.10101101.01111100.01110000.01110110.01100011.10111000.00000001.10001100.11100100.00000010.01101010.11010000.00000110.10101100.00100000.00001100.01000010.00101000.01001011.00001010.10000000.00000001.00001110.10000100.11010000.10001010.00101010.11001011.01101111.01101000.00000010.00000000.01111011.00000001.10000110.11000010.00000000.01100000.00000001.01011110.10100010.00000000.01011001.00000001.00110110.10001000.00000100.10000001.10101000.00000001.10001110.00000000.01010011.10000000.00010010.01100000.00000111.10110000.00001100.00100110.00000000.00101011.00001000.00000000.10010110.11001000.00000010.00000011.00011000.00000001.11000100.00000001.01100010.10011000.00000001.11000000.00000000.11000010.00000000.00000001.10101001.00111101.01011001.10001001.00010000.00000100.10110001.00101100.10000001.00101010.00001111.00101100.10001101.00110000.10001001.00010000.01001000.00000010.00000011.10011001.01011000.10010001.00011001.10011010.01000000.10010111.00110101.00100000.00001011.01101010.01101001.00010111.11000000.00010001.10010110.00010100.10011111.11111111.10110001.11001000.00100100.00011000.00000010.00110010.01100101.10011100.00000000.11100101.11001100.01100111.01110101.11011100.01000001.01100101.10011001.11010000.00000001.00101111.10011010.10000101.10000100.10100000.11101100.00100111.00100000.00001010.11000011.01000000.00010001.10001110.10110010.01010101.10001111.00001010.11100101.10100111.11000011.10000010.00000000.01100000.10100101.11001011.10100110.00001011.11100101.11000110.00001000.00000011.01001011.10011000.11011111.01101100.01011001.10000000.00001101.00000100.10100100.10110101.10110110.00101001.11111101.00100101.10100000.00000011.11110101.00100100.00101111.00110111.00011010.00101100.11001110.01011101.10101010.00011100.10111000.00000010.00110001.01111110.01110111.11110101.01011001.10010010.01000010.
///
// Sum { version: 5, subs: [
//     Literal { version: 0, value: 2556, }, 5
//     Max { version: 7, subs: [ 12
//             Literal { version: 4, value: 402314, }, 16
//             Literal { version: 0, value: 15, },
//             Literal { version: 2, value: 8, }, 18
//             Literal { version: 0, value: 9, },
//         ], },
//     Product { version: 6, subs: [ 24
//             Literal { version: 4, value: 37741, }, 28
//             Greater { version: 2, 30
//                 left: Literal { version: 2, value: 287506942933, }, 32
//                 right: Literal { version: 4, value: 127, }, 36
//             },
//         ],
//     },
//     Sum { version: 7, subs: [ 43
//             Product { version: 2, subs: [ 45
//                     Literal { version: 6, value: 4, }, 49
//                     Literal { version: 3, value: 12, }, 52
//                     Literal { version: 1, value: 3, }, 53
//                 ], },
//             Product { version: 4, subs: [ 57
//                     Literal { version: 7, value: 12, }, 64
//                     Literal { version: 7, value: 13, }, 71
//                     Literal { version: 2, value: 13, }, 73
//                 ], },
//             Product { version: 6, subs: [ 79
//                     Literal { version: 1, value: 9, }, 80
//                     Literal {version: 0, value: 7, },
//                     Literal { version: 2, value: 4, }, 82
//                 ],},
//         ],
//     },
// ],
// }
#[test]
fn day16_v2() {
    let data: Vec<Datum> = read();
    assert_eq!(666, data.len(), "expect 666 datums");
    let stream = DataStream::new(data);
    let mut version_sum = 0;
    if let Some(tail) = stream.tail() {
        assert_eq!(
            tail.index,
            DataIdx { skip: 0, datum: 0 },
            "expect initial index"
        );
        if let Some((expr, _)) = tail.read_expr() {
            assert!(
                match expr {
                    Sum { .. } => true,
                    _ => false,
                },
                "expect a sum expr"
            );
            version_sum = expr.get_versions().iter().fold(0, |a, v| a + v);
            assert_eq!(539051801941, expr.value(), "expect value");
        }
    }
    assert_eq!(879, version_sum, "expect version sum");
}

#[test]
fn day16_test_38006F45291200() {
    let data = read_datums(&"38006F45291200".to_owned());
    let stream = DataStream::new(data);
    let (expr, tail) = stream.tail().unwrap().read_expr().unwrap();
    println!("expr: {:?}", expr);
    assert_eq!(
        Less {
            version: 1,
            left: Box::new(Literal {
                version: 6,
                value: 10
            }),
            right: Box::new(Literal {
                version: 2,
                value: 20
            }),
        },
        expr,
        "exect expr"
    );
    assert_eq!(1, expr.value(), "expect value");
}

#[test]
fn day16_test_EE00D40C823060() {
    let data = read_datums(&"EE00D40C823060".to_owned());
    let stream = DataStream::new(data);
    let (expr, tail) = stream.tail().unwrap().read_expr().unwrap();
    println!("expr: {:?}", expr);
    assert_eq!(
        Max {
            version: 7,
            subs: vec![
                Literal {
                    version: 2,
                    value: 1
                },
                Literal {
                    version: 4,
                    value: 2
                },
                Literal {
                    version: 1,
                    value: 3
                }
            ]
        },
        expr,
        "exect expr"
    );
    assert_eq!(3, expr.value(), "expect value");
}
#[test]
fn day16_test_read_i_110100010100101001000100100() {
    let value = usize::from_str_radix(&"110100010100101001000100100", 2)
        .ok()
        .unwrap();

    let arb = Arb {
        data: DataHead::usize_to_datums(value, 27),
        width: 27,
    };
    let stream = arb.to_stream();
    println!("data: {}", fslice(&stream.data[0..]));
    let idx = DataIdx { datum: 0, skip: 0 };
    println!("{:012b}", stream.read(&idx, 11).unwrap().0);
    let (expr, tail) = stream.tail().unwrap().read_expr().unwrap();
    assert_eq!(
        Literal {
            version: 6,
            value: 10
        },
        expr,
        "expect expr"
    );
}

///.        01234567.89012345.67890123.45678901.23456789.01234567.89012345.67890123.45678901.23456789.01234567.89012345.67890123
///         01100010.00000000.10000000.00000000.00010110.00010001.01010110.00101100.10001000.00000010.00010001.10001110.00110100
///            /-\|/           \/    \ |/              \ /     |                \/. \/ \|/          \ / \/ \|/   \/ \/  \|/  \
/// sum ------ +  ||           ||.                       / \/ \|/   \/ \/ .\|/  \|.                 |
///               ||           ||.                                              ||
/// len_type ---- +|           ||                                               ||
///                |.--------.-||                                               ||
/// n_subs --------+ = 2.       | ----------------------------------------------+|
/// sub 1 ----------------------+                                                |
/// sub 2 -----------------------------------------------------------------------+
///
/// sum v=3 nsubs=2
/// +- sum: v=0 nbits=22
///    +- lit: 10 v=0
///    +- lit: 11 v=5
/// +- sum: v=1 nsubs=2
///    +- lit: 12 v=0
///    +- lit: 13 v=3
#[test]
fn day16_test_620080001611562C8802118E34() {
    let data = read_datums(&"620080001611562C8802118E34".to_owned());
    let stream = DataStream::new(data);
    let (expr, tail) = stream.tail().unwrap().read_expr().unwrap();
    println!("expr: {:?}", expr);
    assert_eq!(
        12,
        expr.get_versions().iter().fold(0, |a, v| a + v),
        "expect version sum"
    );
    assert_eq!(46, expr.value(), "expect value");
}

///                 .  ..                .
/// full string: 11000000.00000001.01010000.00000000.00000001.01100001.00010101.10100010.11100000.10000000.00101111.00011000.00100011.01000000.
/// outer subs (len 84 bits):            00.00000000.00000001.01100001.00010101.10100010.11100000.10000000.00101111.00011000.00100011.01
///                                          .  ..                .  .   .  ..   .
///                                     .00000000.00000000.01011000.01000101.01101000.10111000.00100000.00001011.11000110.00001000.1101
///                                      00000000.00000000.01011000.01000101.01101000.10111000.00100000.00001011.11000110.00001000.11010000.
#[test]
fn day16_test_C0015000016115A2E0802F182340() {
    let data = read_datums(&"C0015000016115A2E0802F182340".to_owned());
    let stream = DataStream::new(data);
    let (expr, tail) = stream.tail().unwrap().read_expr().unwrap();
    println!("expr: {:?}", expr);
    assert_eq!(
        23,
        expr.get_versions().iter().fold(0, |a, v| a + v),
        "expect version sum"
    );
    assert_eq!(46, expr.value(), "expect value");
}
