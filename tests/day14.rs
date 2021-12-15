mod common;
/// --- Day 14: Extended Polymerization ---
///
/// The incredible pressures at this depth are starting to put a strain on your submarine.
/// The submarine has polymerization equipment that would produce suitable materials to
/// reinforce the submarine, and the nearby volcanically-active caves should even have the
/// necessary input elements in sufficient quantities.
///
/// The submarine manual contains instructions for finding the optimal polymer formula;
/// specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input).
/// You just need to work out what polymer would result after repeating the pair insertion process
/// a few times.
fn read() -> (String, Vec<(String, char)>) {
    return parse_input(common::read_test_input("data/day-14/input.txt"));
}

fn parse_input(lines: Vec<String>) -> (String, Vec<(String, char)>) {
    let mut template: String = String::from("");
    let mut rules: Vec<(String, char)> = Vec::new();
    for input_line in lines {
        if template.is_empty() {
            template = String::from(input_line);
            continue;
        }

        if !input_line.is_empty() {
            let parts: Vec<&str> = input_line.split(" -> ").collect();
            rules.push((
                String::from(parts[0]),
                parts[1].chars().last().expect("expect at least one"),
            ));
        }
    }
    return (template, rules);
}

#[test]
fn day14part0() {
    let src = "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
    let input_lines: Vec<String> = src.split("\n").map(|slice| slice.to_owned()).collect();
    let (template, rules) = parse_input(input_lines);
    let mut poly: String = template.clone();
    for i in 0..5 {
        println!("poly: {}", poly);
        do_insertions(&mut poly, &rules);
        match i {
            0 => assert_eq!("NCNBCHB", poly.as_str(), "expect result after {}", i + 1),
            1 => assert_eq!(
                "NBCCNBBBCBHCB",
                poly.as_str(),
                "expect result after {}",
                i + 1
            ),
            2 => assert_eq!(
                "NBBBCNCCNBBNBNBBCHBHHBCHB",
                poly.as_str(),
                "expect result after {}",
                i + 1
            ),
            3 => assert_eq!(
                "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
                poly.as_str(),
                "expect result after {}",
                i + 1
            ),
            _ => {}
        }
    }
    //println!("poly: {}, len: {}", poly, poly.len());
    let (hist, max, min) = histogram(&poly);
    println!("{:?} max: {}, min: {}", hist, max, min);
}

use std::ops::Add;
/// Apply 10 steps of pair insertion to the polymer template and find the most and least common
/// elements in the result. What do you get if you take the quantity of the most common element
/// and subtract the quantity of the least common element?
#[test]
fn day14part1() {
    let (template, rules) = read();
    assert_eq!(20, template.len(), "expect template length");
    assert_eq!(100, rules.len(), "expect n insertion rules");

    let n_passes = 10;
    let mut init_poly: String = template.clone();
    for _ in 0..n_passes {
        do_insertions(&mut init_poly, &rules);
    }
    let (hist, max_count, min_count) = histogram(&init_poly);
    println!("{:?} max: {}, min: {}", hist, max_count, min_count);
    let checksum = hist.iter().fold(0, |acc, (_, next)| acc + next);
    assert_eq!(checksum, init_poly.len(), "expect equal length");
    let count_diff = max_count - min_count;
    assert_ne!(1220, count_diff, "too low");
    assert_ne!(2437, count_diff, "too high"); // off by TWO because of obscure match_indices behavior
    assert_eq!(2435, count_diff, "expect max_count - min_count");
}

/// The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need
/// to run more steps of the pair insertion process; a total of 40 steps should do it.
///
/// Apply 40 steps of pair insertion to the polymer template and find the most and least
/// common elements in the result. What do you get if you take the quantity of the most common
/// element and subtract the quantity of the least common element?

use std::collections::HashMap;
fn hist_bounds(hist: &HashMap<char, usize>) -> (usize, usize) {
    let max_count = hist.iter().fold(0, |prev, (_, next)| {
        if *next > prev {
            return *next;
        } else {
            return prev;
        }
    });
    let min_count = hist.iter().fold(max_count, |prev, (_, next)| {
        if *next < prev {
            return *next;
        } else {
            return prev;
        }
    });
    (max_count, min_count)
}
fn histogram(poly: &String) -> (HashMap<char, usize>, usize, usize) {
    let hist = poly.chars().fold(HashMap::new(), |mut acc, ch| {
        let mut new_count = 1;
        if let Some(count) = acc.get(&ch) {
            new_count = *count + 1;
        }
        acc.insert(ch, new_count);
        return acc;
    });
    let max_count = hist.iter().fold(0, |prev, (_, next)| {
        if *next > prev {
            return *next;
        } else {
            return prev;
        }
    });
    let min_count = hist.iter().fold(max_count, |prev, (_, next)| {
        if *next < prev {
            return *next;
        } else {
            return prev;
        }
    });
    return (hist, max_count, min_count);
}

fn do_insertions(poly: &mut String, rules: &Vec<(String, char)>) {
    let mut founds: Vec<(usize, &(String, char))> = Vec::new();
    for rule in rules {
        let (needle, _) = rule;
        for (index, _) in poly.match_indices(needle) {
            founds.push((index, rule));
            // in case of overlap, we need to check for index + 1
            if index + 3 <= poly.len() && needle == &poly[index + 1..index + 3] {
                founds.push((index + 1, rule));
            }
        }
    }
    //println!("{:?}", founds);
    founds.sort_by_key(|(index, _)| *index);
    founds.reverse();
    for (index, &(_, insert)) in founds {
        poly.insert(index + 1, insert);
    }
}
/// 2 -> 3 -> 5 -> 9 -> 17 -> 33 ->
fn new_size(current_size: usize, iterations: usize) -> usize {
    let mut size = current_size;
    for i in 0..iterations {
        size = size + size - 1;
    }
    return size;
}


#[test]
fn day14part2() {
    let (template, rules) = read();
    let codec = Codec::new(&rules);
    let memo_depth_5 = 10;
    let mut memos_5 = Codec::new_memos();

    let empty_memos = Codec::new_memos();
    for (i, pair) in rules.iter().map(|(k, v)| k).enumerate() {
        println!("memoizing for depth {} {}/100", memo_depth_5, i);
        let memo_hist = codec.measure_insertions(0, &empty_memos, pair, memo_depth_5);
        memos_5.insert(pair.as_str(), memo_hist);
    }

    let memo_depth_20 = 20;
    let mut memos_20 = Codec::new_memos();
    for (i, pair) in rules.iter().map(|(k, v)| k).enumerate() {
        println!("memoizing for depth {} {}/100", memo_depth_20, i);
        let memo_hist = codec.measure_insertions(memo_depth_5, &memos_5, pair, memo_depth_20);
        memos_20.insert(pair.as_str(), memo_hist);
    }

    let memo_depth_30 = 30;
    let mut memos_30 = Codec::new_memos();
    for (i, pair) in rules.iter().map(|(k, v)| k).enumerate() {
        println!("memoizing for depth {} {}/100", memo_depth_30, i);
        let memo_hist = codec.measure_insertions(memo_depth_20, &memos_20, pair, memo_depth_30);
        memos_30.insert(pair.as_str(), memo_hist);
    }

    let mut hist = codec.measure_insertions(memo_depth_30, &memos_30, &template, 40);
    let last_char = template.chars().last().expect("add 1 to last char");
    Codec::inc_hist_char(&mut hist, &last_char, 1);
    let (max, min) = hist_bounds(&hist);
    let count_diff = max - min;
    assert_eq!(2587447599164, count_diff, "expect max_count - min_count");
}
///
fn n_insertion(pair: &str, rules: &Vec<(String, char)>, n: usize) -> String {
    "".to_owned()
}

struct Codec {
    chars: HashMap<String, Letter>,
}

use std::collections::HashSet;
impl<'m> Codec {
    fn new_memos() -> HashMap<&'m str, HashMap<char, usize>> {
        return HashMap::new();
    }

    fn new(rules: &Vec<(String, char)>) -> Codec {
        let mut all_chars: HashSet<String> = HashSet::new();
        for (twochar, _) in rules.iter() {
            for onechar in twochar
                .split("")
                .map(|c| c.to_owned())
                .filter(|c| !c.is_empty())
            {
                all_chars.insert(onechar);
            }
        }
        //println!("all_chars: {:?}, rules: {:?}", all_chars, rules);
        let mut letters: HashMap<String, Letter> = HashMap::new();
        for all_char in all_chars {
            let all_char_c = all_char.chars().nth(0).expect("Codec::new from_left");

            let from_left: HashMap<char, char> = rules
                .iter()
                .filter(|(twochar, _)| &twochar[0..1] == all_char.as_str())
                .map(|(twochar, ch)| {
                    (
                        *(&twochar[1..2].chars().last().expect("Codec::new from_left")),
                        *ch,
                    )
                })
                .collect();
            let from_right: HashMap<char, char> = rules
                .iter()
                .filter(|(twochar, _)| &twochar[1..2] == all_char.as_str())
                .map(|(twochar, ch)| {
                    (
                        *(&twochar[0..1].chars().last().expect("Codec::new from_right")),
                        *ch,
                    )
                })
                .collect();
            let letter: Letter = Letter {
                code: Self::copy_string(&all_char),
                code_c: all_char_c,
                from_left: from_left,
                from_right: from_right,
            };
            letters.insert(Self::copy_string(&all_char), letter);
        }
        return Codec { chars: letters };
    }

    fn copy_string(src: &str) -> String {
        return String::from(src);
    }

    fn measure_insertions(
        &self,
        memo_depth: usize,
        memos: &HashMap<&str, HashMap<char, usize>>,
        input: &String,
        n_iter: usize,
    ) -> HashMap<char, usize> {
        let mut hist: HashMap<char, usize> = HashMap::new();
        for i in 0..(input.len() - 1) {
            println!("measuring insertions for pair: {}", &input[i..i + 2]);
            self.pair_insertions(memo_depth, memos, &mut hist, &input[i..i + 2], n_iter);
        }
        return hist;
    }

    fn pairs_for_pair(&self, pair: &str) -> (String, String) {
        let left_letter: &Letter = self.chars.get(&pair[0..1]).expect("pairs_for_pair");
        let right_letter: &Letter = self.chars.get(&pair[1..2]).expect("pairs_for_pair");
        let inserted = String::from(left_letter.char_with_right(right_letter));
        let left_pair = String::from(&pair[0..1]).add(inserted.as_str());
        let right_pair = String::from(inserted).add(&pair[1..2]);
        (left_pair, right_pair)
    }

    // be sure to count left char, but not right char.
    fn pair_insertions(
        &self,
        memo_depth: usize,
        memos: &HashMap<&str, HashMap<char, usize>>,
        hist: &mut HashMap<char, usize>,
        pair: &str,
        n_iter: usize,
    ) {
        if n_iter == 0 {
            let ch = pair[0..1].chars().last().expect("pair_insertions");
            Self::inc_hist_char(hist, &ch, 1);
        } else if memo_depth > 0 && memo_depth == n_iter {
            if let Some(memo) = memos.get(&pair) {
                for (ch, count) in memo {
                    Self::inc_hist_char(hist, ch, *count);
                }
            }
        } else {
            let (left_pair, right_pair) = self.pairs_for_pair(pair);
            self.pair_insertions(memo_depth, memos, hist, left_pair.as_str(), n_iter - 1);
            self.pair_insertions(memo_depth, memos, hist, right_pair.as_str(), n_iter - 1);
        }
    }

    fn inc_hist_char(hist: &mut HashMap<char, usize>, ch: &char, value: usize) {
        let char_count: usize = if let Some(from_hist) = hist.get(ch) {
            *from_hist
        } else {
            0
        };
        hist.insert(*ch, char_count + value);
    }
}

struct Letter {
    code: String,
    code_c: char,
    from_left: HashMap<char, char>,
    from_right: HashMap<char, char>,
}

impl Letter {
    fn char_with_right(&self, right: &Letter) -> char {
        return *self.from_left.get(&right.code_c).expect("char_with_right");
    }

    fn build_from_right(&self, codec: &Codec, left: &Letter, n_iter: usize) -> String {
        let mut buffer: Vec<char> = Vec::with_capacity(new_size(2, n_iter));

        return buffer.iter().collect();
    }

    // 0..=39

    fn append_self(&self, codec: &Codec, right: &Letter, n_iter: usize, buffer: &mut Vec<char>) {}
}
