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
            return total * self.last_mark;
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

    fn day05read(filename: &str) -> Vec<Segment> {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut segments: Vec<Segment> = Vec::new();

        for line in reader.lines().flat_map(|line_r| line_r.ok()) {
            segments.push(read_segment(&line));
        }
        return segments;
    }

    fn read_segment(line: &str) -> Segment {
        let scalars: Vec<usize> = (*line)
            .split(" -> ")
            .take(2)
            .flat_map(|point| point.split(','))
            .flat_map(|scalar_s| scalar_s.parse().ok())
            .collect();
        let segment = Segment {
            from: (scalars[0], scalars[1]),
            to: (scalars[2], scalars[3]),
        };
        return segment.canonical();
    }

    #[test]
    fn day05_subline_coincidence() {
        let short_h: Segment = Segment {
            from: (0, 9),
            to: (2, 9),
        };
        let long_h: Segment = Segment {
            from: (0, 9),
            to: (5, 9),
        };

        assert_eq!(
            true,
            short_h.is_coincident_with(&long_h),
            "expect horiz coincidence"
        );

        let interxs_h: Vec<Point> = short_h.intersections(&long_h);
        assert_eq!(3, interxs_h.len(), "expect size of horiz intersection");

        let short_v: Segment = Segment {
            from: (9, 0),
            to: (9, 2),
        };
        let long_v: Segment = Segment {
            from: (9, 0),
            to: (9, 5),
        };

        assert_eq!(
            true,
            short_v.is_coincident_with(&long_v),
            "expect vert coincidence"
        );

        let interxs_v: Vec<Point> = short_v.intersections(&long_v);
        assert_eq!(3, interxs_v.len(), "expect size of vert intersection");
    }

    #[test]
    fn day05_headtail_coincidence() {
        let head_l: Segment = Segment {
            from: (9, 4),
            to: (3, 4),
        };
        let tail_l: Segment = Segment {
            from: (3, 4),
            to: (1, 4),
        };

        assert_eq!(
            true,
            head_l.is_coincident_with(&tail_l),
            "expect left coincidence"
        );

        let interxs_l: Vec<Point> = head_l.intersections(&tail_l);
        assert_eq!(1, interxs_l.len(), "expect size of left intersection");

        let head_u: Segment = Segment {
            from: (4, 9),
            to: (4, 3),
        };
        let tail_u: Segment = Segment {
            from: (4, 3),
            to: (4, 1),
        };

        assert_eq!(
            true,
            head_u.is_coincident_with(&tail_u),
            "expect up coincidence"
        );

        let interxs_u: Vec<Point> = head_u.intersections(&tail_u);
        assert_eq!(1, interxs_u.len(), "expect size of up intersection");

        let head_r: Segment = Segment {
            from: (1, 4),
            to: (3, 4),
        };
        let tail_r: Segment = Segment {
            from: (3, 4),
            to: (9, 4),
        };

        assert_eq!(
            true,
            head_r.is_coincident_with(&tail_r),
            "expect right coincidence"
        );

        let interxs_r: Vec<Point> = head_r.intersections(&tail_r);
        assert_eq!(1, interxs_r.len(), "expect size of right intersection");

        let head_d: Segment = Segment {
            from: (4, 1),
            to: (4, 3),
        };
        let tail_d: Segment = Segment {
            from: (4, 3),
            to: (4, 9),
        };

        assert_eq!(
            true,
            head_d.is_coincident_with(&tail_d),
            "expect down coincidence"
        );

        let interxs_d: Vec<Point> = head_d.intersections(&tail_d);
        assert_eq!(1, interxs_d.len(), "expect size of down intersection");
    }

    #[test]
    fn day05part0() {
        let src = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let straights: Vec<Segment> = src
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| read_segment(line))
            .filter(|segment| segment.is_horizontal() || segment.is_vertical())
            .collect();
        assert_eq!(6, straights.len(), "expect number of straight segments");

        let comparisons: Vec<Segment> = straights.iter().map(|&e| e).collect();
        use std::collections::HashSet;
        let interxs: HashSet<Point> = straights
            .iter()
            .flat_map(|segment| {
                comparisons
                    .iter()
                    .flat_map(|other| segment.intersections(other))
            })
            .collect();

        let mut sorted: Vec<Point> = interxs.iter().map(|&e| e).collect();
        sorted.sort();

        let mut display: String = "".to_owned();
        for (x, y) in sorted.iter() {
            let formatted = format!("{}, {}\n", x, y);
            display.push_str(&formatted);
        }

        assert_eq!(
            5,
            interxs.len(),
            "expect number of intersections: {}",
            display
        );
    }

    #[test]
    fn day05part1() {
        use std::collections::HashSet;

        let filename = "data/day-05/input.txt";
        let segments = day05read(&filename);
        assert_eq!(500, segments.len(), "expect number of segments");

        let straights: HashSet<Segment> = segments
            .iter()
            .map(|&e| e.canonical())
            .filter(|segment| segment.is_horizontal() || segment.is_vertical())
            .collect();
        assert_eq!(357, straights.len(), "expect number of straight segments");

        let comparisons: Vec<Segment> = straights.iter().map(|&e| e).collect();

        let interxs: HashSet<Point> = straights
            .iter()
            .flat_map(|segment| {
                comparisons
                    .iter()
                    .flat_map(|other| segment.intersections(other))
            })
            .collect();

        let mut sorted: Vec<Point> = interxs.iter().map(|&e| e).collect();
        sorted.sort();
        assert_ne!(4022, interxs.len(), "4022 is too low");
        assert_ne!(8508, interxs.len(), "8508 is too high");
        assert_eq!(7142, interxs.len(), "expect number of intersections");
    }

    #[test]
    fn day05part2() {
        use std::collections::HashSet;

        let filename = "data/day-05/input.txt";
        let segments = day05read(&filename);
        assert_eq!(500, segments.len(), "expect number of segments");

        let comparisons: Vec<Segment> = segments.iter().cloned().collect();

        let interxs: HashSet<Point> = segments
            .iter()
            .flat_map(|segment| {
                comparisons
                    .iter()
                    .flat_map(|other| segment.intersections(other))
            })
            .collect();

        assert_ne!(10198, interxs.len(), "10198 is too low");
        assert_eq!(20012, interxs.len(), "expect number of intersections");
    }

    type Point = (usize, usize);

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    struct Segment {
        from: Point,
        to: Point,
    }

    fn between(num: usize, lower: usize, upper: usize) -> bool {
        return (num >= lower && num <= upper) || (num >= upper && num <= lower);
    }

    fn urange(from: usize, to: usize) -> std::ops::RangeInclusive<usize> {
        if from > to {
            return to..=from;
        } else {
            return from..=to;
        }
    }

    impl Segment {
        fn is_vertical(&self) -> bool {
            return self.map_points1(|(x1, _), (x2, _)| -> bool {
                return x1 == x2;
            });
        }

        fn is_horizontal(&self) -> bool {
            return self.map_points1(|(_, y1), (_, y2)| -> bool {
                return y1 == y2;
            });
        }

        fn is_rise_right(&self) -> bool {
            return self.canonical().map_points1(|(x1, y1), (x2, y2)| -> bool {
                return x2 > x1 && y2 < y1;
            });
        }

        fn is_rise_left(&self) -> bool {
            return self.canonical().map_points1(|(x1, y1), (x2, y2)| -> bool {
                return x2 > x1 && y2 > y1;
            });
        }

        fn map_points1<T>(&self, f: fn(Point, Point) -> T) -> T {
            return f(self.from, self.to);
        }

        fn map_points2<T>(&self, other: &Segment, f: fn(Point, Point, Point, Point) -> T) -> T {
            return f(self.from, self.to, other.from, other.to);
        }

        fn is_coincident_with(&self, other: &Segment) -> bool {
            let left: Segment = self.canonical();
            let right: Segment = other.canonical();
            if left.from == right.from && left.to == right.to {
                return false;
            }
            return left.from == right.from
                || left.from == right.to
                || left.to == right.from
                || left.to == right.to;
        }

        fn is_colinear_with(&self, other: &Segment) -> bool {
            let left: Segment = self.canonical();
            let right: Segment = other.canonical();
            if left.from == right.from && left.to == right.to {
                return false;
            }

            if left.is_parallel_to(&right) {
                if left.is_vertical() {
                    return left.map_points2(
                        &right,
                        |(sx1, _), (_, _), (ox1, _), (_, _)| -> bool {
                            return sx1 == ox1;
                        },
                    );
                } else if left.is_horizontal() {
                    return left.map_points2(
                        &right,
                        |(_, sy1), (_, _), (_, oy1), (_, _)| -> bool {
                            return sy1 == oy1;
                        },
                    );
                } else if left.is_rise_left() {
                    return left.map_points2(
                        &right,
                        |(sx1, sy1), (_, _), (ox1, oy1), (_, _)| -> bool {
                            let isy1: i32 = sy1 as i32;
                            let isx1: i32 = sx1 as i32;
                            let ioy1: i32 = oy1 as i32;
                            let iox1: i32 = ox1 as i32;
                            return isy1 - isx1 == ioy1 - iox1;
                        },
                    );
                } else if left.is_rise_right() {
                    return left.map_points2(
                        &right,
                        |(sx1, sy1), (_, _), (ox1, oy1), (_, _)| -> bool {
                            let isy1: i32 = sy1 as i32;
                            let isx1: i32 = sx1 as i32;
                            let ioy1: i32 = oy1 as i32;
                            let iox1: i32 = ox1 as i32;
                            return isy1 + isx1 == ioy1 + iox1;
                        },
                    );
                }
            }
            return false;
        }

        fn canonical(&self) -> Segment {
            let (x1, y1) = self.from;
            let (x2, y2) = self.to;
            if x2 < x1 || x2 == x1 && y2 < y1 {
                return Segment {
                    from: self.to,
                    to: self.from,
                };
            } else {
                return *self;
            }
        }

        fn intersections(&self, other_segment: &Segment) -> Vec<Point> {
            let left: Segment = self.canonical();
            let right: Segment = other_segment.canonical();
            if left.from == right.from && left.to == right.to {
                return Vec::new();
            }
            if let Some(point) = left.linear_intersection(&right) {
                let mut interxs = Vec::new();
                interxs.push(point);
                return interxs;
            } else if left.is_colinear_with(&right) {
                use std::collections::HashSet;
                let left_points: HashSet<Point> = left.points().iter().cloned().collect();
                let right_points: HashSet<Point> = right.points().iter().cloned().collect();
                return left_points.intersection(&right_points).cloned().collect();
            }
            return Vec::new();
        }

        fn is_parallel_to(&self, other: &Segment) -> bool {
            return (self.is_horizontal() && other.is_horizontal())
                || (self.is_vertical() && other.is_vertical())
                || (self.is_rise_left() && other.is_rise_left())
                || (self.is_rise_right() && other.is_rise_right());
        }

        fn _solve_for_x(&self, other: &Segment) -> Option<usize> {
            if other.is_horizontal() {
                return other._solve_for_x(self);
            } else if self.is_horizontal() {
                let (_, sy1) = self.from;
                if other.is_rise_left() {
                    let (ox1, oy1) = other.from;
                    let z = (oy1 as i32) - (ox1 as i32);
                    let ix = (sy1 as i32) - z;
                    if ix >= 0 {
                        return Some(ix as usize);
                    }
                } else if other.is_rise_right() {
                    let (ox1, oy1) = other.from;
                    let z = (oy1 as i32) + (ox1 as i32);
                    let ix = -1 * ((sy1 as i32) - z);
                    if ix >= 0 {
                        return Some(ix as usize);
                    }
                }
            } else if self.is_rise_left() {
                let (sx1, sy1) = self.from;
                let sz = (sy1 as i32) - (sx1 as i32);
                if other.is_rise_right() {
                    let (ox1, oy1) = other.from;
                    let oz = (oy1 as i32) + (ox1 as i32);
                    if (oz - sz) % 2 != 0 {
                        return None;
                    }
                    let ix = (oz - sz) / 2;
                    if ix >= 0 {
                        return Some(ix as usize);
                    }
                }
            }
            None
        }

        fn linear_intersection(&self, other: &Segment) -> Option<Point> {
            if !self.is_parallel_to(&other) {
                if self.is_vertical() || other.is_vertical() {
                    return self._vertical_intersection(other);
                }
                let (sx1, sy1) = self.from;
                let (sx2, sy2) = self.to;
                let (ox1, oy1) = other.from;
                let (ox2, oy2) = other.to;
                return self
                    ._solve_for_x(other)
                    .filter(|x| between(*x, sx1, sx2) && between(*x, ox1, ox2))
                    .and_then(|x| self._y_for_x(x))
                    .filter(|(_, y1)| between(*y1, sy1, sy2) && between(*y1, oy1, oy2));
            }
            return None;
        }

        fn _y_for_x(&self, x: usize) -> Option<Point> {
            if self.is_horizontal() {
                let (_, y1) = self.from;
                return Some((x, y1));
            } else if self.is_rise_right() {
                let (x1, y1) = self.from;
                let (_, y2) = self.to;
                let iz = (y1 as i32) + (x1 as i32);
                let y = iz - (x as i32);
                if y >= 0 && between(y as usize, y1, y2) {
                    return Some((x, (y as usize)));
                }
            } else if self.is_rise_left() {
                let (x1, y1) = self.from;
                let (_, y2) = self.to;
                let iz = (y1 as i32) - (x1 as i32);
                let y = iz + (x as i32);
                if y >= 0 && between(y as usize, y1, y2) {
                    return Some((x, (y as usize)));
                }
            }
            None
        }

        fn _vertical_intersection(&self, other: &Segment) -> Option<Point> {
            if self.is_vertical() {
                if other.is_vertical() {
                    return None;
                }
                let (sx1, sy1) = self.from;
                let (_, sy2) = self.to;
                let (ox1, oy1) = other.from;
                let (ox2, oy2) = other.to;
                return other._y_for_x(sx1).filter(|(x1, y1)| {
                    sx1 == *x1
                        && between(*x1, ox1, ox2)
                        && between(*y1, sy1, sy2)
                        && between(*y1, oy1, oy2)
                });
            } else if other.is_vertical() {
                return other._vertical_intersection(self);
            }
            return None;
        }

        fn points(&self) -> Vec<Point> {
            let right = self.canonical();
            if right.is_horizontal() {
                return right.map_points1(|(x1, y1), (x2, _)| -> Vec<Point> {
                    let mut points: Vec<Point> = Vec::new();
                    for x in urange(x1, x2) {
                        points.push((x, y1))
                    }
                    return points;
                });
            }
            if right.is_vertical() {
                return right.map_points1(|(x1, y1), (_, y2)| -> Vec<Point> {
                    let mut points: Vec<Point> = Vec::new();
                    for y in urange(y1, y2) {
                        points.push((x1, y))
                    }
                    return points;
                });
            }
            if right.is_rise_left() {
                return right.map_points1(|(x1, y1), (x2, y2)| -> Vec<Point> {
                    let mut points: Vec<Point> = Vec::new();
                    for y in urange(y1, y2) {
                        points.push((std::cmp::min(x1, x2) + (y - std::cmp::min(y1, y2)), y))
                    }
                    return points;
                });
            }
            if right.is_rise_right() {
                return right.map_points1(|(x1, y1), (x2, y2)| -> Vec<Point> {
                    let mut points: Vec<Point> = Vec::new();
                    for y in urange(y1, y2) {
                        points.push((std::cmp::max(x1, x2) - (y - std::cmp::min(y1, y2)), y))
                    }
                    return points;
                });
            }
            return Vec::new();
        }
    }

    #[test]
    fn day05_crossing() {
        let short_h: Segment = Segment {
            from: (0, 1),
            to: (2, 1),
        };
        let short_v: Segment = Segment {
            from: (1, 0),
            to: (1, 2),
        };

        assert_eq!(
            false,
            short_h.is_coincident_with(&short_v),
            "expect no coincidence"
        );

        let interxs: Vec<Point> = short_h.intersections(&short_v);
        assert_eq!(1, interxs.len(), "expect size of intersection");
        assert_eq!((1, 1), interxs[0], "expect intersection");
    }
    #[test]
    fn day05_right_t() {
        let short_h: Segment = Segment {
            from: (0, 1),
            to: (2, 1),
        };
        let short_v: Segment = Segment {
            from: (0, 0),
            to: (0, 2),
        };

        assert_eq!(
            false,
            short_h.is_coincident_with(&short_v),
            "expect no coincidence"
        );

        let interxs: Vec<Point> = short_h.intersections(&short_v);
        assert_eq!(1, interxs.len(), "expect size of intersection");
        assert_eq!((0, 1), interxs[0], "expect intersection");
    }

    #[test]
    fn day05_left_t() {
        let short_h: Segment = Segment {
            from: (0, 1),
            to: (2, 1),
        };
        let short_v: Segment = Segment {
            from: (2, 0),
            to: (2, 2),
        };

        assert_eq!(
            false,
            short_h.is_coincident_with(&short_v),
            "expect no coincidence"
        );

        let interxs: Vec<Point> = short_h.intersections(&short_v);
        assert_eq!(1, interxs.len(), "expect size of intersection");
        assert_eq!((2, 1), interxs[0], "expect intersection");
    }

    #[test]
    fn day05_up_t() {
        let short_h: Segment = Segment {
            from: (0, 2),
            to: (2, 2),
        };
        let short_v: Segment = Segment {
            from: (1, 0),
            to: (1, 2),
        };

        assert_eq!(
            false,
            short_h.is_coincident_with(&short_v),
            "expect no coincidence"
        );

        let interxs: Vec<Point> = short_h.intersections(&short_v);
        assert_eq!(1, interxs.len(), "expect size of intersection");
        assert_eq!((1, 2), interxs[0], "expect intersection");
    }

    #[test]
    fn day05_down_t() {
        let short_h: Segment = Segment {
            from: (0, 0),
            to: (2, 0),
        };
        let short_v: Segment = Segment {
            from: (1, 0),
            to: (1, 2),
        };

        assert_eq!(
            false,
            short_h.is_coincident_with(&short_v),
            "expect no coincidence"
        );

        let interxs: Vec<Point> = short_h.intersections(&short_v);
        assert_eq!(1, interxs.len(), "expect size of intersection");
        assert_eq!((1, 0), interxs[0], "expect intersection");
    }
}
