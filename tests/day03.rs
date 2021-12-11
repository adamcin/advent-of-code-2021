mod common;

fn read() -> Vec<i16> {
    let values: Vec<i16> = common::read_test_input("data/day-03/input.txt")
        .iter()
        .cloned()
        .flat_map(|line| read_diagnostic(line))
        .collect();
    values
}

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
