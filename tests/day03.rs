mod common;
/// --- Day 3: Binary Diagnostic ---
///
/// The submarine has been making some odd creaking noises, so you ask it to 
/// produce a diagnostic report just in case.
///
/// The diagnostic report (your puzzle input) consists of a list of binary 
/// numbers which, when decoded properly, can tell you many useful things about 
/// the conditions of the submarine. The first parameter to check is the power consumption.
///
/// You need to use the binary numbers in the diagnostic report to generate two 
/// new binary numbers (called the gamma rate and the epsilon rate). The power 
/// consumption can then be found by multiplying the gamma rate by the epsilon rate.
///
/// Each bit in the gamma rate can be determined by finding the most common bit in 
/// the corresponding position of all numbers in the diagnostic report. 
fn read() -> Vec<i16> {
    let values: Vec<i16> = common::read_test_input("data/day-03/input.txt")
        .iter()
        .cloned()
        .flat_map(|line| read_diagnostic(line))
        .collect();
    values
}

/// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, 
/// then multiply them together. What is the power consumption of the submarine? 
/// (Be sure to represent your answer in decimal, not binary.)
#[test]
fn day03part1() {
    let readings: Vec<i16> = read();

    assert_eq!(1000, readings.len(), "expect readings vec length");

    let mut gamma: i16 = 0;
    let mut epsilon: i16 = 0;
    let base: i16 = 2;
    for index in 0..12 {
        let multi: i16 = base.pow(index);
        let ones: Vec<&i16> = readings
            .iter()
            .filter(|reading| (**reading) & multi == multi)
            .collect();
        if ones.len() >= readings.len() / 2 {
            gamma += multi;
        } else {
            epsilon += multi;
        }
    }

    let gamma32: i32 = gamma.into();
    let epsilon32: i32 = epsilon.into();

    assert_ne!(1222, gamma32 * epsilon32, "it's not this");
    assert_ne!(6157242, gamma32 * epsilon32, "it's not this");
    assert_ne!(351162, gamma32 * epsilon32, "it's not this");
    assert_eq!(738234, gamma32 * epsilon32, "expect gamma times epsilon");
}

fn read_diagnostic(line: String) -> Option<i16> {
    return i16::from_str_radix(&line, 2).ok();
}

fn filter_bits(input: Vec<i16>, index: u32) -> (Vec<i16>, Vec<i16>) {
    let base: i16 = 2;
    let multi: i16 = (base).pow(index);
    let ones: Vec<i16> = input
        .iter()
        .cloned()
        .filter(|&reading| reading & multi == multi)
        .collect();
    let zeros: Vec<i16> = input
        .iter()
        .cloned()
        .filter(|&reading| reading & multi != multi)
        .collect();
    assert_eq!(input.len(), ones.len() + zeros.len(), "expect same total");
    assert_ne!(1000, ones.len(), "expect non-1000 ones");
    assert_ne!(1000, zeros.len(), "expect non-1000 ones");
    return (ones, zeros);
}

#[test]
fn day03_match_first() {
    let num: i16 = 0b110110110000;
    let nums: Vec<i16> = [num].iter().map(|&value| value).collect();
    let (ones, zeros) = filter_bits(nums, 11);
    assert_eq!(1, ones.len(), "expect ones");
    assert_eq!(0, zeros.len(), "expect no zeros");
}

#[test]
fn day03_match_third() {
    let num: i16 = 0b110110110000;
    let nums: Vec<i16> = [num].iter().map(|&value| value).collect();
    let (ones, zeros) = filter_bits(nums, 9);
    assert_eq!(0, ones.len(), "expect no ones");
    assert_eq!(1, zeros.len(), "expect zeros");
}

#[test]
fn day03_match_last() {
    let num: i16 = 0b110110110000;
    let nums: Vec<i16> = [num].iter().map(|&value| value).collect();
    let (ones, zeros) = filter_bits(nums, 0);
    assert_eq!(0, ones.len(), "expect no ones");
    assert_eq!(1, zeros.len(), "expect zeros");
}

fn search_diagnostics(
    original: Vec<i16>,
    chooser: fn(usize, Vec<i16>, Vec<i16>) -> Vec<i16>,
) -> i32 {
    let reduced: Vec<i16> = (0..=11).rev().fold(original, |init, index| {
        let init_len = init.len();

        if init_len <= 1 {
            return init;
        }

        let (ones, zeros) = filter_bits(init, index);
        return chooser(init_len, ones, zeros);
    });
    assert_eq!(1, reduced.len(), "expect single element");
    let element: i16 = reduced[0];
    return element.into();
}

/// Next, you should verify the life support rating, which can be determined by multiplying 
/// the oxygen generator rating by the CO2 scrubber rating.
///
/// Both the oxygen generator rating and the CO2 scrubber rating are values that can be found 
/// in your diagnostic report - finding them is the tricky part. Both values are located using 
/// a similar process that involves filtering out values until only one remains. Before searching 
/// for either rating value, start with the full list of binary numbers from your diagnostic 
/// report and consider just the first bit of those numbers. Then:
///
/// * Keep only numbers selected by the bit criteria for the type of rating value for which you 
/// are searching. Discard numbers which do not match the bit criteria.
/// * If you only have one number left, stop; this is the rating value for which you are searching.
/// * Otherwise, repeat the process, considering the next bit to the right.
/// 
/// The bit criteria depends on which type of rating value you want to find:
///
/// * To find oxygen generator rating, determine the most common value (0 or 1) in the current bit 
/// position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, 
/// keep values with a 1 in the position being considered.
/// * To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit 
/// position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, 
/// keep values with a 0 in the position being considered.
/// 
/// Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and 
/// CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? 
/// (Be sure to represent your answer in decimal, not binary.)
#[test]
fn day03part2() {
    let readings: Vec<i16> = read();

    assert_eq!(1000, readings.len(), "expect readings vec length");
    let oxy_chooser: fn(usize, Vec<i16>, Vec<i16>) -> Vec<i16> = |init_len, ones, zeros| {
        if zeros.len() == init_len {
            return zeros;
        } else if ones.len() == init_len || ones.len() * 2 >= init_len {
            return ones;
        } else {
            return zeros;
        }
    };
    let init_oxys: Vec<i16> = readings.iter().map(|&item| item).collect();
    let oxygen: i32 = search_diagnostics(init_oxys, oxy_chooser);

    let scrub_chooser: fn(usize, Vec<i16>, Vec<i16>) -> Vec<i16> = |init_len, ones, zeros| {
        if ones.len() == init_len {
            return ones;
        } else if zeros.len() == init_len || ones.len() * 2 >= init_len {
            return zeros;
        } else {
            return ones;
        }
    };
    let init_scrubs: Vec<i16> = readings.iter().map(|&item| item).collect();
    let scrubber: i32 = search_diagnostics(init_scrubs, scrub_chooser);

    assert_eq!(3969126, oxygen * scrubber, "expect lifesupport rating");
}
