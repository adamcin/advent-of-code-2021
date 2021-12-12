mod common;

/// --- Day 11: Dumbo Octopus ---
///
/// You enter a large cavern full of rare bioluminescent dumbo octopuses!
/// They seem to not like the Christmas lights on your submarine, so you turn them off for now.
///
/// There are 100 octopuses arranged neatly in a 10 by 10 grid.
/// Each octopus slowly gains energy over time and flashes brightly for a moment when its energy is full.
/// Although your lights are off, maybe you could navigate through the cave without disturbing the
/// octopuses if you could predict when the flashes of light will happen.
///
/// Each octopus has an energy level - your submarine can remotely measure the energy level of each
/// octopus (your puzzle input).
fn read() -> Octopi {
    let mut grid: Grid = Octopi::new_grid(0);
    let rows: Vec<Vec<usize>> = common::read_test_input("data/day-11/input.txt")
        .iter()
        .map(|line| line.split("").flat_map(|c| c.parse().ok()).collect())
        .collect();

    assert_eq!(GRID_SIZE, rows.len(), "expect n rows");
    for (row_i, row) in rows.iter().enumerate() {
        assert_eq!(GRID_SIZE, row.len(), "expect n cols in row #{}", row_i);
        for (col_i, &col) in row.iter().enumerate() {
            assert!(
                col < 10,
                "expect uint value at ({},{}) to be between 0 and 9",
                row_i,
                col_i
            );
            grid[row_i][col_i] = col;
        }
    }

    return Octopi {
        current_step: 0,
        flashes: 0,
        grid: grid,
    };
}

const GRID_SIZE: usize = 10;
type Grid = [[usize; GRID_SIZE]; GRID_SIZE];

#[derive(Copy, Clone, Debug)]
enum AdjDir {
    N = 0,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
}

use AdjDir::{Ne, Nw, Se, Sw, E, N, S, W};
impl AdjDir {
    fn values() -> [AdjDir; 8] {
        return [N, Ne, E, Se, S, Sw, W, Nw];
    }
}

/// (row, col)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn adjacent(&self, dir: AdjDir) -> Option<Position> {
        match dir {
            N => {
                if self.row > 0 {
                    return Some(Position {
                        row: self.row - 1,
                        col: self.col,
                    });
                }
            }
            Ne => {
                if self.row > 0 && self.col + 1 < GRID_SIZE {
                    return Some(Position {
                        row: self.row - 1,
                        col: self.col + 1,
                    });
                }
            }
            E => {
                if self.col + 1 < GRID_SIZE {
                    return Some(Position {
                        row: self.row,
                        col: self.col + 1,
                    });
                }
            }
            Se => {
                if self.row + 1 < GRID_SIZE && self.col + 1 < GRID_SIZE {
                    return Some(Position {
                        row: self.row + 1,
                        col: self.col + 1,
                    });
                }
            }
            S => {
                if self.row + 1 < GRID_SIZE {
                    return Some(Position {
                        row: self.row + 1,
                        col: self.col,
                    });
                }
            }
            Sw => {
                if self.row + 1 < GRID_SIZE && self.col > 0 {
                    return Some(Position {
                        row: self.row + 1,
                        col: self.col - 1,
                    });
                }
            }
            W => {
                if self.col > 0 {
                    return Some(Position {
                        row: self.row,
                        col: self.col - 1,
                    });
                }
            }
            Nw => {
                if self.row > 0 && self.col > 0 {
                    return Some(Position {
                        row: self.row - 1,
                        col: self.col - 1,
                    });
                }
            }
        }
        None
    }

    fn adjacents(&self) -> [Option<Position>; 8] {
        let mut items: [Option<Position>; 8] = [None; 8];
        for dir in AdjDir::values() {
            items[dir as usize] = self.adjacent(dir);
        }
        return items;
    }
}

use std::iter::IntoIterator;
impl IntoIterator for Position {
    type Item = Position;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let positions: Vec<Position> = self.adjacents().iter().flat_map(|&adj| adj).collect();
        return positions.into_iter();
    }
}

struct Octopi {
    current_step: usize,
    flashes: usize,
    grid: Grid,
}

impl Octopi {
    /// You can model the energy levels and flashes of light in steps.
    /// During a single step, the following occurs:
    ///
    /// First, the energy level of each octopus increases by 1.
    ///
    /// Then, any octopus with an energy level greater than 9 flashes.
    /// * This increases the energy level of all adjacent octopuses by 1,
    ///   including octopuses that are diagonally adjacent.
    /// * If this causes an octopus to have an energy level greater than 9, it also flashes.
    ///   This process continues as long as new octopuses keep having their energy level
    ///   increased beyond 9. (An octopus can only flash at most once per step.)
    ///
    /// Finally, any octopus that flashed during this step has its energy level set to 0,
    /// as it used all of its energy to flash.
    ///
    /// Adjacent flashes can cause an octopus to flash on a step even if it begins that
    /// step with very little energy.
    fn step(&mut self) -> usize {
        self.current_step += 1;
        self.apply_overlay(&Self::new_grid(1));
        let mut to_mark = self.all_gonna_flash(&vec![]);
        if to_mark.is_empty() {
            return 0;
        }
        let mut overlay = Self::new_grid(0);
        let mut marked = to_mark.len();
        while marked > 0 {
            marked = Self::mark_for_flash(&mut overlay, &to_mark);
            to_mark = self.all_gonna_flash(&vec![overlay]);
        }
        self.apply_overlay(&overlay);
        return self.flash(&to_mark);
    }

    fn new_grid(start: usize) -> Grid {
        return [[start; GRID_SIZE]; GRID_SIZE];
    }

    fn apply_overlay(&mut self, overlay: &Grid) {
        for row_i in 0..GRID_SIZE {
            for col_i in 0..GRID_SIZE {
                self.grid[row_i][col_i] += overlay[row_i][col_i];
            }
        }
    }

    fn flash(&mut self, octs: &Vec<Position>) -> usize {
        for &Position { row, col } in octs.iter() {
            self.grid[row][col] = 0;
        }
        self.flashes += octs.len();
        return octs.len();
    }

    fn mark_for_flash(grid: &mut Grid, octs: &Vec<Position>) -> usize {
        let mut count_marked = 0;
        for oct in octs {
            if !Self::is_marked(oct, grid) {
                count_marked += 1;
                grid[oct.row][oct.col] = 999;
                for adj in oct.adjacents().iter() {
                    if let &Some(Position { row, col }) = adj {
                        grid[row][col] += 1;
                    }
                }
            }
        }
        count_marked
    }

    fn is_marked(oct: &Position, grid: &Grid) -> bool {
        return grid[oct.row][oct.col] > 99;
    }

    fn all_gonna_flash(&self, overlays: &Vec<Grid>) -> Vec<Position> {
        let mut poss: Vec<Position> = Vec::new();
        for row_i in 0..GRID_SIZE {
            for col_i in 0..GRID_SIZE {
                let val: usize =
                    self.grid[row_i][col_i] + overlays.iter().fold(0, |a, g| a + g[row_i][col_i]);
                if val > 9 {
                    poss.push(Position {
                        row: row_i,
                        col: col_i,
                    });
                }
            }
        }
        return poss;
    }

    fn count_flashes(&self) -> usize {
        self.flashes
    }
}

#[test]
fn day11_single() {
    let mut input = Octopi {
        current_step: 0,
        flashes: 0,
        grid: Octopi::new_grid(0),
    };
    let center = Position { row: 5, col: 5 };
    input.grid[center.row][center.col] = 9;
    input.step();
    assert_eq!(1, input.count_flashes(), "expect 1 flash");
}

/// Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100 steps.
/// How many total flashes are there after 100 steps?
#[test]
fn day11part1() {
    let mut input = read();
    for _ in 0..100 {
        input.step();
    }

    assert_ne!(1000, input.count_flashes(), "too low");
    assert_eq!(1700, input.count_flashes(), "expect n flashes");
}

/// It seems like the individual flashes aren't bright enough to navigate.
/// However, you might have a better option: the flashes seem to be synchronizing!
/// If you can calculate the exact moments when the octopuses will all flash
/// simultaneously, you should be able to navigate through the cavern.
/// What is the first step during which all octopuses flash?
#[test]
fn day11part2() {
    let mut input = read();

    while input.step() != GRID_SIZE * GRID_SIZE && input.current_step < 10000 {}
    assert_ne!(10000, input.current_step, "not synchronized");
    assert_eq!(273, input.current_step, "expect step number");
}
