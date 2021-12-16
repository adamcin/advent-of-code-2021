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

use std::ops::Add;
/// Apply 10 steps of pair insertion to the polymer template and find the most and least common
/// elements in the result. What do you get if you take the quantity of the most common element
/// and subtract the quantity of the least common element?
#[test]
fn day14part1() {
    let (template, rules) = read();
    assert_eq!(20, template.len(), "expect template length");
    assert_eq!(100, rules.len(), "expect n insertion rules");
    let codec = Codec::new(&rules);
    let count_diff = codec.solution(&template, 10);
    assert_ne!(1220, count_diff, "too low");
    assert_ne!(2437, count_diff, "too high"); // off by TWO because of obscure match_indices behavior
    assert_eq!(2435, count_diff, "expect max_count - min_count");
}

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

/// The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need
/// to run more steps of the pair insertion process; a total of 40 steps should do it.
///
/// Apply 40 steps of pair insertion to the polymer template and find the most and least
/// common elements in the result. What do you get if you take the quantity of the most common
/// element and subtract the quantity of the least common element?
#[test]
fn day15part2() {
    let (template, rules) = read();
    let codec = Codec::new(&rules);
    let count_diff = codec.solution(&template, 40);
    assert_eq!(2587447599164, count_diff, "expect max_count - min_count");
}

struct Codec<'m> {
    pairs: Vec<&'m String>,
    chars: HashMap<String, Letter>,
}

use std::collections::HashSet;
impl<'m> Codec<'m> {
    fn new_memos() -> HashMap<&'m str, HashMap<char, usize>> {
        return HashMap::new();
    }

    fn new(rules: &'m Vec<(String, char)>) -> Codec<'m> {
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

        let mut letters: HashMap<String, Letter> = HashMap::new();
        for all_char in all_chars {
            let all_char_c = all_char.chars().nth(0).expect("Codec::new from_left");

            let from_left: HashMap<char, char> = rules
                .iter()
                .filter(|(twochar, _)| &twochar[0..1] == all_char.as_str())
                .map(|(twochar, ch)| {
                    (
                        *(&twochar.chars().nth(1).expect("Codec::new from_left")),
                        *ch,
                    )
                })
                .collect();
            let letter: Letter = Letter {
                code_c: all_char_c,
                from_left: from_left,
            };
            letters.insert(Self::copy_string(&all_char), letter);
        }
        return Codec {
            pairs: rules.iter().map(|(k, _)| k).collect(),
            chars: letters,
        };
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
            self.pair_insertions(memo_depth, memos, &mut hist, &input[i..i + 2], n_iter);
        }
        return hist;
    }

    fn solution(&self, input: &String, iterations: usize) -> usize {
        let final_memos = if iterations > 1 {
            (0..(iterations - 1)).fold(Codec::new_memos(), |last_memos, memo_depth| {
                return self
                    .pairs
                    .iter()
                    .fold(Self::new_memos(), |mut memos, pair| {
                        let hist =
                            self.measure_insertions(memo_depth, &last_memos, pair, memo_depth + 1);
                        memos.insert(pair.as_str(), hist);
                        return memos;
                    });
            })
        } else {
            Codec::new_memos()
        };
        let mut hist = self.measure_insertions(iterations - 1, &final_memos, input, iterations);
        let last_char = input.chars().last().expect("add 1 to last char");
        Codec::inc_hist_char(&mut hist, &last_char, 1);
        let (max, min) = hist_bounds(&hist);
        return max - min;
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
    code_c: char,
    from_left: HashMap<char, char>,
}

impl Letter {
    fn char_with_right(&self, right: &Letter) -> char {
        return *self.from_left.get(&right.code_c).expect("char_with_right");
    }
}
