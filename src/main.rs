fn main() {
    println!("Hello, world!");
}

#[cfg(test)] // The module is only compiled when testing.
mod test {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    // This function is a test function. It will be executed and
    // the test will succeed if the function exits cleanly.
    #[test]
    fn day01part1() {
        let filename = "data/day-01/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let values: Vec<i64> = reader
            .lines()
            .flat_map(|line| line.map(|str| str.parse()))
            .filter_map(|value| value.ok())
            .collect();

        assert_eq!(2000, values.len(), "expect vec len");

        let increases: Vec<i64> = values[1..]
            .iter()
            .zip(values.iter())
            .map(|(next, value)| next - value)
            .filter(|diff| diff > &0)
            .collect();

        assert_eq!(1557, increases.len(), "expect increases len");
    }

    #[test]
    fn day01part2() {
        let filename = "data/day-01/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let firsts: Vec<i64> = reader
            .lines()
            .flat_map(|line| line.map(|str| str.parse()))
            .filter_map(|value| value.ok())
            .collect();

        assert_eq!(2000, firsts.len(), "expect vec len");

        // zip3 iterating from thirds vector, then sum
        let sums: Vec<i64> = firsts[2..] // thirds
            .iter()
            .zip(firsts[1..].iter()) // seconds
            .zip(firsts.iter()) // firsts
            .map(|((third, second), first)| third + second + first)
            .collect();

        let increases: Vec<i64> = sums[1..]
            .iter()
            .zip(sums.iter())
            .map(|(next, value)| next - value)
            .filter(|diff| diff > &0)
            .collect();

        assert_eq!(1608, increases.len(), "expect increases len");
    }

    enum SubCommand {
        Up { scale: i64 },
        Down { scale: i64 },
        Forward { scale: i64 },
    }

    fn parse_command_name(name: &str, scale: i64) -> Option<SubCommand> {
        if "up" == name {
            return Some(SubCommand::Up { scale: scale });
        } else if "down" == name {
            return Some(SubCommand::Down { scale: scale });
        } else if "forward" == name {
            return Some(SubCommand::Forward { scale: scale });
        } else {
            return None;
        }
    }

    fn read_command(line: String) -> Option<SubCommand> {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 2 {
            let cmd = parts[0];
            let sca = parts[1];
            let opt_value: Option<i64> = sca.parse::<i64>().ok();
            return opt_value.and_then(|value| parse_command_name(cmd, value));
        } else {
            return None;
        }
    }

    #[test]
    fn day02part1() {
        let filename = "data/day-02/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let commands: Vec<SubCommand> = reader
            .lines()
            .flat_map(|line_res| line_res.map(|line| read_command(line)))
            .flatten()
            .collect();

        assert_eq!(1000, commands.len(), "expect command vec length");

        let initial_loc: (i64, i64) = (0, 0);

        let (final_x, final_y): (i64, i64) = commands.iter().fold(initial_loc, |acc, cmd| {
            let (cur_x, cur_y) = acc;
            return match cmd {
                SubCommand::Up { scale } => (cur_x, cur_y - scale),
                SubCommand::Down { scale } => (cur_x, cur_y + scale),
                SubCommand::Forward { scale } => (cur_x + scale, cur_y),
            };
        });

        assert_eq!(1480518, final_x * final_y, "expect command vec length");
    }

    #[test]
    fn day02part2() {
        let filename = "data/day-02/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let commands: Vec<SubCommand> = reader
            .lines()
            .flat_map(|line_res| line_res.map(|line| read_command(line)))
            .flatten()
            .collect();

        assert_eq!(1000, commands.len(), "expect command vec length");

        let initial_loc: (i64, i64, i64) = (0, 0, 0);

        let (final_x, final_y, _): (i64, i64, i64) =
            commands.iter().fold(initial_loc, |acc, cmd| {
                let (cur_x, cur_y, cur_aim) = acc;
                return match cmd {
                    SubCommand::Up { scale } => (cur_x, cur_y, cur_aim - scale),
                    SubCommand::Down { scale } => (cur_x, cur_y, cur_aim + scale),
                    SubCommand::Forward { scale } => {
                        (cur_x + scale, cur_y + (cur_aim * scale), cur_aim)
                    }
                };
            });

        assert_eq!(1282809906, final_x * final_y, "expect command vec length");
    }

    #[test]
    fn day03part1() {
        let filename = "data/day-03/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let readings: Vec<i16> = reader
            .lines()
            .flat_map(|line_res| line_res.map(|line| read_diagnostic(line)))
            .flatten()
            .collect();

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
            .map(|&reading| reading)
            .filter(|reading| reading & multi == multi)
            .collect();
        let zeros: Vec<i16> = input
            .iter()
            .map(|&reading| reading)
            .filter(|reading| reading & multi != multi)
            .collect();
        assert_eq!(input.len(), ones.len() + zeros.len(), "expect same total");
        assert_ne!(1000, ones.len(), "expect non-1000 ones");
        assert_ne!(1000, zeros.len(), "expect non-1000 ones");
        return (ones, zeros);
    }

    #[test]
    fn day03_match_first() {
        let num: i16 = 0b110110110000;
        let nums: Vec<i16> = [num].iter().map(|&item| item).collect();
        let (ones, zeros) = filter_bits(nums, 11);
        assert_eq!(1, ones.len(), "expect ones");
        assert_eq!(0, zeros.len(), "expect no zeros");
    }

    #[test]
    fn day03_match_third() {
        let num: i16 = 0b110110110000;
        let nums: Vec<i16> = [num].iter().map(|&item| item).collect();
        let (ones, zeros) = filter_bits(nums, 9);
        assert_eq!(0, ones.len(), "expect no ones");
        assert_eq!(1, zeros.len(), "expect zeros");
    }

    #[test]
    fn day03_match_last() {
        let num: i16 = 0b110110110000;
        let nums: Vec<i16> = [num].iter().map(|&item| item).collect();
        let (ones, zeros) = filter_bits(nums, 0);
        assert_eq!(0, ones.len(), "expect no ones");
        assert_eq!(1, zeros.len(), "expect zeros");
    }

    #[test]
    fn day03part2() {
        let filename = "data/day-03/input.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let readings: Vec<i16> = reader
            .lines()
            .flat_map(|line_res| line_res.map(|line| read_diagnostic(line)))
            .flatten()
            .collect();

        assert_eq!(1000, readings.len(), "expect readings vec length");

        let init_oxys: Vec<i16> = readings.iter().map(|&item| item).collect();
        let init_scrubs: Vec<i16> = readings.iter().map(|&item| item).collect();
        let mut range: Vec<u32> = (0..=11).collect();
        range.reverse();
        assert_eq!(12, range.len(), "fail!");
        let (oxygens, scrubbers): (Vec<i16>, Vec<i16>) =
            range
                .iter()
                .fold((init_oxys, init_scrubs), |(oxys, scrubs), &index| {
                    let oxys_len = oxys.len();
                    let scrubs_len = scrubs.len();
                    let scrubs_copy: Vec<i16> = scrubs.iter().map(|&item| item).collect();
                    if oxys_len <= 1 && scrubs_len <= 1 {
                        return (oxys, scrubs);
                    }
                    let (oxy_ones, oxy_zeros) = filter_bits(oxys, index);
                    let fewer_oxys = if oxy_zeros.len() == oxys_len {
                        oxy_zeros
                    } else if oxy_ones.len() == oxys_len || oxy_ones.len() * 2 >= oxys_len {
                        oxy_ones
                    } else {
                        oxy_zeros
                    };

                    let (scrub_ones, scrub_zeros) = filter_bits(scrubs, index);
                    let fewer_scrubs = if scrub_ones.len() == scrubs_len {
                        scrub_ones
                    } else if scrub_zeros.len() == scrubs_len || scrub_ones.len() * 2 >= scrubs_len {
                        scrub_zeros
                    } else {
                        scrub_ones
                    };

                    if fewer_scrubs.len() == 0 {
                        let mut filtered: String = "\n".to_owned();
                        for scrub in scrubs_copy {
                            let scrub_string: String = format!("{:b}\n", scrub);
                            filtered.push_str(&scrub_string);
                        }
                        assert_ne!(
                            0,
                            fewer_scrubs.len(),
                            "remaining scrubs is zero after index: {}, {}",
                            index,
                            filtered
                        );
                    }
                    return (fewer_oxys, fewer_scrubs);
                });

        assert_eq!(1, oxygens.len(), "expect one oxygen");
        assert_eq!(1, scrubbers.len(), "expect one scrubber");

        let oxygen: i16 = oxygens[0];
        let oxygen32: i32 = oxygen.into();
        let scrubber: i16 = scrubbers[0];
        let scrubber32: i32 = scrubber.into();
        assert_eq!(0, oxygen32 * scrubber32, "expect lifesupport rating");
    }
}
