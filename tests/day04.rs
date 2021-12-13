mod common;
/// --- Day 4: Giant Squid ---
///
/// You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that 
/// you can't see any sunlight. What you can see, however, is a giant squid that has attached itself 
/// to the outside of your submarine.
///
/// Maybe it wants to play bingo?
///
/// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen 
/// at random, and the chosen number is marked on all boards on which it appears. (Numbers may not 
/// appear on all boards.) If all numbers in any row or any column of a board are marked, that board 
/// wins. (Diagonals don't count.)
///
/// The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass 
/// the time. It automatically generates a random order in which to draw numbers and a random set of 
/// boards (your puzzle input).
fn read() -> (Vec<usize>, Vec<Board>) {
    let mut calls: Vec<usize> = Vec::new();
    let mut board_srcs: Vec<Vec<Vec<usize>>> = Vec::new();
    board_srcs.push(Vec::new());
    for input_line in common::read_test_input("data/day-04/input.txt").iter() {
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

/// To guarantee victory against the giant squid, figure out which board will win first. 
/// What will your final score be if you choose that board?
#[test]
fn day04part1() {
    let (calls, mut boards) = read();
    assert_eq!(100, calls.len(), "expect number of calls");
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

/// On the other hand, it might be wise to try a different strategy: let the giant squid win.
///
/// You aren't sure how many bingo boards a giant squid could play at once, so rather than waste 
/// time counting its arms, the safe thing to do is to figure out which board will win last and 
/// choose that one. That way, no matter which boards it picks, it will win for sure.
/// 
/// Figure out which board will win last. Once it wins, what would its final score be?
#[test]
fn day04part2() {
    let (calls, mut boards) = read();
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
