mod common;

fn read() -> Seafloor {
    Seafloor::new(
        common::read_test_input("data/day-25/input.txt")
            .iter()
            .map(|line| line.chars().collect())
            .collect(),
    )
}

fn read_test() -> Seafloor {
    Seafloor::new(
        common::read_test_input("data/day-25/input_test.txt")
            .iter()
            .map(|line| line.chars().collect())
            .collect(),
    )
}

struct Seafloor {
    rows: usize,
    cols: usize,
    data: Vec<Vec<char>>,
}

type Coord = (usize, usize);

const CHAR_EMPTY: char = '.';
const CHAR_EAST: char = '>';
const CHAR_SOUTH: char = 'v';

impl Seafloor {
    fn new(data: Vec<Vec<char>>) -> Self {
        Self {
            rows: data.len(),
            cols: data.first().map(|row| row.len()).unwrap_or(0),
            data: data,
        }
    }

    fn scan_to_west(&self) -> Vec<Coord> {
        let mut empties: Vec<Coord> = Vec::new();
        for row in 0..self.rows {
            if let (Some(first), Some(last)) = (self.data[row].first(), self.data[row].last()) {
                if *first == CHAR_EMPTY && *last == CHAR_EAST {
                    empties.push((row, 0));
                }

                let row_vec = &self.data[row];
                empties.extend(
                    row_vec
                        .iter()
                        .zip(row_vec.iter().skip(1))
                        .enumerate()
                        .filter_map(|(col, (west, east))| {
                            if *east == CHAR_EMPTY && *west == CHAR_EAST {
                                Some((row, col + 1))
                            } else {
                                None
                            }
                        }),
                );
            }
        }
        empties
    }

    fn scan_to_north(&self) -> Vec<Coord> {
        let mut empties: Vec<Coord> = Vec::new();
        if let (Some(first_vec), Some(last_vec)) = (self.data.first(), self.data.last()) {
            empties.extend(
                last_vec
                    .iter()
                    .zip(first_vec.iter())
                    .enumerate()
                    .filter_map(|(col, (north, south))| {
                        if *south == CHAR_EMPTY && *north == CHAR_SOUTH {
                            Some((0, col))
                        } else {
                            None
                        }
                    }),
            );
        }
        for row in 0..(self.rows - 1) {
            empties.extend(
                self.data[row]
                    .iter()
                    .zip(self.data[row + 1].iter())
                    .enumerate()
                    .filter_map(|(col, (north, south))| {
                        if *south == CHAR_EMPTY && *north == CHAR_SOUTH {
                            Some((row + 1, col))
                        } else {
                            None
                        }
                    }),
            )
        }
        empties
    }

    fn swap_east(&mut self, empty: &Coord) {
        let (row, ecol) = *empty;
        let hcol = if ecol == 0 { self.cols - 1 } else { ecol - 1 };
        self.data[row][ecol] = CHAR_EAST;
        self.data[row][hcol] = CHAR_EMPTY;
    }

    fn swap_south(&mut self, empty: &Coord) {
        let (erow, col) = *empty;
        let hrow = if erow == 0 { self.rows - 1 } else { erow - 1 };
        self.data[erow][col] = CHAR_SOUTH;
        self.data[hrow][col] = CHAR_EMPTY;
    }

    fn step(&mut self) -> bool {
        let east_swaps = self.scan_to_west();
        let n_east_swaps = east_swaps.len();
        for swap in east_swaps {
            self.swap_east(&swap);
        }

        let south_swaps = self.scan_to_north();
        let n_south_swaps = south_swaps.len();
        for swap in south_swaps {
            self.swap_south(&swap);
        }
        n_east_swaps + n_south_swaps > 0
    }
}

#[test]
fn day25_test_read() {
    let seafloor = read();
    assert_eq!(139, seafloor.cols);
    assert_eq!(137, seafloor.rows);
}

#[test]
fn day25pre_part1() {
    let mut seafloor = read_test();
    let mut step_num: usize = 1;

    loop {
        if seafloor.step() {
            step_num = step_num + 1;
        } else {
            break;
        }
    }

    assert_eq!(58, step_num, "expect step number");
}

#[test]
fn day25part1() {
    let mut seafloor = read();
    let mut step_num: usize = 1;

    loop {
        if seafloor.step() {
            step_num = step_num + 1;
        } else {
            break;
        }
    }

    assert_eq!(353, step_num, "expect step number");
}