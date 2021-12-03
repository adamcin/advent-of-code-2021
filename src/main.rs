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
            .filter(|&reading| reading & multi == multi)
            .collect();
        let zeros: Vec<i16> = input
            .iter()
            .map(|&reading| reading)
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
}
