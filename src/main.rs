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

    struct Board {
        // rows by cols
        values: [[usize; 5]; 5],
        marks: [[bool; 5]; 5],
        last_mark: usize,
    }

    impl Board {
        fn mark(&mut self, value: usize) -> bool {
            if let Some((row, col)) = self.find(&value) {
                if !self.marks[row][col] {
                    self.marks[row][col] = true;
                    self.last_mark = value;
                    return true;
                }
            }
            false
        }

        fn find(&self, value: &usize) -> Option<(usize, usize)> {
            for row in 0..=4 {
                for col in 0..=4 {
                    if self.values[row][col] == *value {
                        return Some((row, col));
                    }
                }
            }
            None
        }

        fn score(&self) -> usize {
            let mut total = 0;
            for row in 0..=4 {
                for col in 0..=4 {
                    if !self.marks[row][col] {
                        total += self.values[row][col];
                    }
                }
            }
            return total * self.last_mark
        }

        fn check_bingo(&self) -> bool {
            for row in 0..=4 {
                if self.marks[row] == [true, true, true, true, true] {
                    return true;
                }
            }
            for col in 0..=4 {
                if self.marks[0][col]
                    && self.marks[1][col]
                    && self.marks[2][col]
                    && self.marks[3][col]
                    && self.marks[4][col]
                {
                    return true;
                }
            }
            false
        }
    }

    fn day04read(filename: &str) -> (Vec<usize>, Vec<Board>) {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut calls: Vec<usize> = Vec::new();
        let mut board_srcs: Vec<Vec<Vec<usize>>> = Vec::new();
        board_srcs.push(Vec::new());
        for input_line in reader.lines().flat_map(|line_r| line_r.ok()) {
            if calls.is_empty() {
                let calls_s: Vec<usize> = input_line
                    .split(',')
                    .filter_map(|call_s| call_s.parse().ok())
                    .collect();
                calls = calls_s;
            }

            if input_line.is_empty() {
                if board_srcs.last().filter(|b| !b.is_empty()).is_some() {
                    board_srcs.push(Vec::new());
                }
            } else {
                if let Some(last_board) = board_srcs.last_mut() {
                    last_board.push(
                        input_line
                            .split_ascii_whitespace()
                            .flat_map(|value_s| value_s.parse())
                            .collect(),
                    );
                }
            }
        }
        let boards: Vec<Board> = board_srcs
        .iter()
        .flat_map(|b_src| {
            let mut values = [[0; 5]; 5];
            let mut five_rows = false;
            for (row_i, row_v) in b_src.iter().enumerate().take(5) {
                let mut five_cols = false;
                for (col_i, col_v) in row_v.iter().enumerate().take(5) {
                    values[row_i][col_i] = *col_v;
                    if col_i == 4 {
                        five_cols = true;
                    }
                }
                if row_i == 4 && five_cols {
                    five_rows = true;
                }
            }

            let mut to_boards: Vec<Board> = Vec::new();
            if five_rows {
                to_boards.push(Board {
                    values: values,
                    marks: [[false; 5]; 5],
                    last_mark: 0,
                });
            }
            return to_boards;
        })
        .collect();
        (calls, boards)
    }

    #[test]
    fn day04part1() {
        let filename = "data/day-04/input.txt";
        let (calls, mut boards) = day04read(&filename);
        
        assert_eq!(100, boards.len(), "expect number of boards");

        'outer: for call in calls.iter() {
            for (index, board) in boards.iter_mut().enumerate() {
                if board.mark(*call) {
                    if board.check_bingo() {
                        assert_eq!(39984, board.score(), "expect winning score {}", index);
                        break 'outer;
                    }
                }
            }
        }
    }

    #[test]
    fn day04part2() {
        let filename = "data/day-04/input.txt";
        let (calls, mut boards) = day04read(&filename);
        
        assert_eq!(100, boards.len(), "expect number of boards");

        let mut scores: Vec<usize> = Vec::new();

        for call in calls.iter() {
            let mut index_rem: Vec<usize> = Vec::new();
            for (index, board) in boards.iter_mut().enumerate() {
                if board.mark(*call) {
                    if board.check_bingo() {
                        scores.push(board.score());
                        index_rem.push(index);
                    }
                }
            }
            index_rem.sort();
            for index in index_rem.iter().rev() {
                boards.remove(*index);
            }
        }
        
        if let Some(&score) = scores.last() {
            assert_eq!(8468, score, "expect last winning score");
        }
    }
}
