#[cfg(test)]
mod day10 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;

    enum SyntaxCheckResult {
        Incomplete { score: usize },
        Corrupt { score: usize },
    }

    impl SyntaxCheckResult {
        fn corrupt_score(&self) -> usize {
            return match *self {
                SyntaxCheckResult::Corrupt { score } => score,
                _ => 0,
            };
        }
        fn complete_score(&self) -> usize {
            return match *self {
                SyntaxCheckResult::Incomplete { score } => score,
                _ => 0,
            };
        }
    }
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Delim {
        LRound,
        LSquare,
        LCurly,
        LAngle,
        RRound,
        RSquare,
        RCurly,
        RAngle,
    }

    use Delim::*;
    impl FromStr for Delim {
        type Err = ();

        fn from_str(input: &str) -> Result<Delim, Self::Err> {
            match input {
                "(" => Ok(LRound),
                "[" => Ok(LSquare),
                "{" => Ok(LCurly),
                "<" => Ok(LAngle),
                ")" => Ok(RRound),
                "]" => Ok(RSquare),
                "}" => Ok(RCurly),
                ">" => Ok(RAngle),
                _ => Err(()),
            }
        }
    }

    impl Delim {
        fn is_open(&self) -> bool {
            return *self == LRound || *self == LSquare || *self == LCurly || *self == LAngle;
        }

        fn is_close(&self) -> bool {
            return !self.is_open();
        }

        fn matches(&self, other: &Delim) -> bool {
            return (self.is_close() && other.matches(self))
                || *self == LRound && *other == RRound
                || *self == LSquare && *other == RSquare
                || *self == LCurly && *other == RCurly
                || *self == LAngle && *other == RAngle;
        }

        fn to_closing(&self) -> Option<Delim> {
            return match *self {
                LRound => Some(RRound),
                LSquare => Some(RSquare),
                LCurly => Some(RCurly),
                LAngle => Some(RAngle),
                _ => None,
            };
        }

        fn corrupt_score(&self) -> usize {
            return match self {
                RRound => 3,
                RSquare => 57,
                RCurly => 1197,
                RAngle => 25137,
                _ => 0,
            };
        }

        fn close_score(&self) -> usize {
            return match self {
                RRound => 1,
                RSquare => 2,
                RCurly => 3,
                RAngle => 4,
                _ => 0,
            };
        }
    }

    fn check_syntax(line: &Vec<Delim>) -> SyntaxCheckResult {
        let mut stack: Vec<Delim> = Vec::new();
        for delim in line {
            if delim.is_open() {
                stack.push(*delim);
            } else {
                if let Some(last) = stack.last() {
                    if last.matches(delim) {
                        stack.pop();
                    } else {
                        return SyntaxCheckResult::Corrupt {
                            score: delim.corrupt_score(),
                        };
                    }
                }
            }
        }
        let to_close: Vec<Delim> = stack.iter().rev().flat_map(|d| d.to_closing()).collect();
        let close_score = to_close.iter().fold(0, |a, v| (5 * a) + v.close_score());
        SyntaxCheckResult::Incomplete { score: close_score }
    }

    fn read_input(filename: &str) -> Vec<Vec<Delim>> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().flat_map(|line_r| line_r.ok()).collect();
        let parsed: Vec<Vec<Delim>> = lines
            .iter()
            .map(|line_s: &String| -> Vec<Delim> {
                line_s.split("").flat_map(|c| c.parse().ok()).collect()
            })
            .collect();
        return parsed;
    }

    #[test]
    fn part1() {
        let input = read_input("data/day-10/input.txt");

        assert_eq!(90, input.len(), "expect number of lines");

        let results: Vec<SyntaxCheckResult> = input.iter().map(|line| check_syntax(line)).collect();

        assert_eq!(90, results.len(), "expect number of results");

        let corrupt_score: usize = results
            .iter()
            .map(|r| r.corrupt_score())
            .fold(0, |a, v| a + v);

        assert_eq!(339411, corrupt_score, "expect corrupt score");
    }

    #[test]
    fn part2() {
        let input = read_input("data/day-10/input.txt");

        assert_eq!(90, input.len(), "expect number of lines");

        let results: Vec<SyntaxCheckResult> = input.iter().map(|line| check_syntax(line)).collect();

        assert_eq!(90, results.len(), "expect number of results");

        let mut complete_scores: Vec<usize> = results
            .iter()
            .map(|r| r.complete_score())
            .filter(|&s| s > 0)
            .collect();

        complete_scores.sort();

        let complete_score = complete_scores[complete_scores.len() / 2];

        assert_ne!(741714, complete_score, "too low");
        assert_ne!(316965209766, complete_score, "too high");
        assert_eq!(2289754624, complete_score, "expect complete score");
    }
}
