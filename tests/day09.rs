mod common;

use std::collections::HashSet;

fn read() -> CaveMap {
    let raw_heights: Vec<Vec<usize>> = common::read_test_input("data/day-09/input.txt")
        .iter()
        .map(|line| -> Vec<usize> { line.split("").flat_map(|c| c.parse().ok()).collect() })
        .clone()
        .collect();

    assert_eq!(100, raw_heights.len(), "expect n lines of input");

    for (index, row) in raw_heights.iter().enumerate() {
        assert_eq!(100, row.len(), "expect n cols in row #{}", index);
    }

    let mut rows: [[usize; CAVE_SIZE]; CAVE_SIZE] = [[0; CAVE_SIZE]; CAVE_SIZE];
    for (ri, row) in raw_heights.iter().enumerate() {
        for (ci, &col) in row.iter().enumerate() {
            rows[ri][ci] = col;
        }
    }

    return CaveMap { rows: rows };
}

const CAVE_SIZE: usize = 100;

type Point = (usize, usize);

struct CaveMap {
    rows: [[usize; CAVE_SIZE]; CAVE_SIZE],
}

impl CaveMap {
    pub fn height(&self, point: Point) -> usize {
        let (row, col) = point;
        return self.rows[row][col];
    }

    pub fn risk_level(&self, point: Point) -> usize {
        return self.height(point) + 1;
    }

    pub fn adjacents(&self, point: Point) -> [Option<Point>; 4] {
        let mut adjs: [Option<Point>; 4] = [None; 4]; // T R B L
        let (row, col) = point;
        if row <= CAVE_SIZE && col <= CAVE_SIZE {
            if row > 0 {
                adjs[0] = Some((row - 1, col));
            }
            if col < CAVE_SIZE - 1 {
                adjs[1] = Some((row, col + 1));
            }
            if row < CAVE_SIZE - 1 {
                adjs[2] = Some((row + 1, col));
            }
            if col > 0 {
                adjs[3] = Some((row, col - 1));
            }
        }
        adjs
    }

    pub fn all_points(&self) -> Vec<Point> {
        let mut all: Vec<Point> = Vec::new();
        for row in 0..CAVE_SIZE {
            for col in 0..CAVE_SIZE {
                all.push((row, col));
            }
        }
        all
    }

    pub fn low_points(&self) -> Vec<Point> {
        return self
            .all_points()
            .iter()
            .filter(|&c| {
                let height = self.height(*c);
                let lower: Vec<Point> = self
                    .adjacents(*c)
                    .iter()
                    .cloned()
                    .filter(|&a| a.is_some())
                    .filter(|&a| {
                        if let Some(adj) = a {
                            return self.height(adj) <= height;
                        } else {
                            return false;
                        }
                    })
                    .flat_map(|a| a)
                    .collect();
                return lower.is_empty();
            })
            .cloned()
            .collect();
    }

    fn _expand_basin(&self, basin_points: &mut HashSet<Point>, points: Vec<Point>) -> Vec<Point> {
        let to_insert: Vec<Point> = points
            .iter()
            .filter(|&p| self.height(*p) < 9 && !basin_points.contains(p))
            .cloned()
            .collect();
        for &point in to_insert.iter() {
            basin_points.insert(point);
        }
        return to_insert
            .iter()
            .flat_map(|&p| -> Vec<Point> {
                self.adjacents(p)
                    .iter()
                    .flat_map(|&a| {
                        if let Some(adj) = a {
                            return vec![adj];
                        } else {
                            return Vec::new();
                        }
                    })
                    .collect()
            })
            .collect();
    }
    pub fn expand_basin(&self, low_point: Point) -> Vec<Point> {
        let mut basin_points: HashSet<Point> = HashSet::new();
        let initial_input: Vec<Point> = vec![low_point];
        let mut expansion: Vec<Point> = self._expand_basin(&mut basin_points, initial_input);
        let mut expanded_basin = expansion.len() > 0;
        while expanded_basin {
            expansion = self._expand_basin(&mut basin_points, expansion);
            expanded_basin = expansion.len() > 0;
        }
        return basin_points.iter().cloned().collect();
    }
}

#[test]
fn day09part1() {
    let input: CaveMap = read();

    let low_points = input.low_points();
    assert_eq!(238, low_points.len(), "expect number of low points");

    let sum_risk_levels: usize = low_points
        .iter()
        .map(|&p| input.risk_level(p))
        .fold(0, |a, v| a + v);
    assert_eq!(570, sum_risk_levels, "expect sum of risk levels");
}

#[test]
fn day09part2() {
    let input: CaveMap = read();

    let low_points = input.low_points();
    assert_eq!(238, low_points.len(), "expect number of low points");

    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|&p| input.expand_basin(p).len())
        .collect();

    basin_sizes.sort();
    let solution: usize = basin_sizes.iter().rev().take(3).fold(1, |a, v| a * v);

    assert_eq!(
        899392, solution,
        "expect solution for triple basin size product"
    );
}
