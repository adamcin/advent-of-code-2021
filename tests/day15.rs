mod common;

/// --- Day 15: Chiton ---
///
/// You've almost reached the exit of the cave, but the walls are getting closer together. Your 
/// submarine can barely still fit, though; the main problem is that the walls of the cave are 
/// covered in chitons, and it would be best not to bump any of them.
///
/// The cavern is large, but has a very low ceiling, restricting your motion to two dimensions. 
/// The shape of the cavern resembles a square; a quick scan of chiton density produces a map of 
/// risk level throughout the cave (your puzzle input).
/// 
/// personal note:
/// 1. google "path finding algorithms"
/// 2. find wiki for Dijkstra's algorithm. begin implementing.
/// 3. reach step where a priority queue is called for
/// 4. google "rust priority queue"
/// 5. land on docs.rust-lang.org page for BinaryHeap
/// 6. copy-paste entire example after reading: 
///    "This is a larger example that implements Dijkstra’s algorithm to solve the shortest
///     path problem on a directed graph. It shows how to use BinaryHeap with custom types."
/// 
/// P.S. Rust arrays are allocated on stack, not on heap. 500x500 usize table doesn't fit.
const CAVE_SIZE: usize = 100;
const META_SIZE: usize = 5;
const FULL_SIZE: usize = CAVE_SIZE * META_SIZE;
type CaveRaw = [[usize; CAVE_SIZE]; CAVE_SIZE];
fn read() -> CaveMap {
    let mut grid: CaveRaw = [[0; CAVE_SIZE]; CAVE_SIZE];
    let collected: Vec<Vec<usize>> = common::read_test_input("data/day-15/input.txt")
        .iter()
        .map(|line| {
            line.split("")
                .flat_map(|e| e.parse().ok())
                .take(CAVE_SIZE)
                .collect()
        })
        .take(CAVE_SIZE)
        .collect();
    for (row, coll_vec) in collected.iter().enumerate() {
        for (col, value) in coll_vec.iter().enumerate() {
            grid[row][col] = *value;
        }
    }

    for row in 0..CAVE_SIZE {
        for col in 0..CAVE_SIZE {
            assert_ne!(0, grid[row][col], "expect no remaining zeros");
        }
    }
    return CaveMap::new(grid);
}
use std::collections::BinaryHeap;

/// You start in the top left position, your destination is the bottom right position, and you 
/// cannot move diagonally. The number at each position is its risk level; to determine the total 
/// risk of an entire path, add up the risk levels of each position you enter (that is, don't 
/// count the risk level of your starting position unless you enter it; leaving it adds no risk 
/// to your total).
///
/// Your goal is to find a path with the lowest total risk.
#[test]
fn day15part1() {
    let cave = read();

    let paths = cave.find_paths(CAVE_SIZE, &ORIGIN);
    let path = CaveMap::get_dist_cost(&paths, &EXIT);

    assert_eq!(472, path, "expect total risk");
}

/// Now that you know how to find low-risk paths in the cave, you can try to find your way out.
///
/// The entire cave is actually five times larger in both dimensions than you thought; the area 
/// you originally scanned is just one tile in a 5x5 tile area that forms the full map. Your 
/// original map tile repeats to the right and downward; each time the tile repeats to the right 
/// or downward, all of its risk levels are 1 higher than the tile immediately up or left of it. 
/// However, risk levels above 9 wrap back around to 1. 
/// 
/// Using the full map, what is the lowest total risk of any path from the top left to the bottom 
/// right?
#[test]
fn day15part2() {
    let cave = read();

    let paths = cave.find_paths(FULL_SIZE, &ORIGIN);
    let path = CaveMap::get_dist_cost(&paths, &FULL_EXIT);

    assert_eq!(2851, path, "expect total risk");
}

type Point = (usize, usize);

const ORIGIN: Point = (0, 0);
const EXIT: Point = (CAVE_SIZE - 1, CAVE_SIZE - 1);
const FULL_EXIT: Point = (FULL_SIZE - 1, FULL_SIZE - 1);

type Grid = [[usize; CAVE_SIZE]; CAVE_SIZE];
type HeapGrid = Vec<Vec<usize>>;

struct CaveMap {
    rows: Grid,
}

impl CaveMap {
    pub fn new(grid: Grid) -> CaveMap {
        return CaveMap { rows: grid };
    }

    fn fix_risk(orig_risk: &usize, meta_row: &usize, meta_col: &usize) -> usize {
        (((orig_risk - 1) + meta_row + meta_col) % 9) + 1
    }

    pub fn meta_point_for(point: &Point) -> (Point, Point) {
        let (row, col) = *point;
        return (
            (row / CAVE_SIZE, col / CAVE_SIZE),
            (row % CAVE_SIZE, col % CAVE_SIZE),
        );
    }

    pub fn risk_level(&self, point: &Point) -> usize {
        let ((meta_row, meta_col), (row, col)) = Self::meta_point_for(point);
        let orig_risk = self.rows[row][col];
        return Self::fix_risk(&orig_risk, &meta_row, &meta_col);
    }

    pub fn adjacents(&self, point: &Point) -> Vec<Point> {
        let mut adjs: [Option<Point>; 4] = [None; 4]; // T R B L
        let (row, col) = *point;
        if row > 0 {
            adjs[0] = Some((row - 1, col));
        }
        if col < FULL_SIZE - 1 {
            adjs[1] = Some((row, col + 1));
        }
        if row < FULL_SIZE - 1 {
            adjs[2] = Some((row + 1, col));
        }
        if col > 0 {
            adjs[3] = Some((row, col - 1));
        }
        return adjs.iter().filter_map(|&p| p).collect();
    }

    pub fn new_heap_grid(&self, size: usize, value: usize) -> HeapGrid {
        return (0..size)
            .map(|_| (0..size).map(|_| value).collect())
            .collect();
    }

    pub fn find_paths(&self, size: usize, from: &Point) -> HeapGrid {
        let mut dist: HeapGrid = self.new_heap_grid(size, usize::MAX);

        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let (from_row, from_col) = *from;
        dist[from_row][from_col] = 0;
        heap.push(State::new(*from, 0));

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { position, cost }) = heap.pop() {
            let (pos_row, pos_col) = position;
            // Important as we may have already found a better way
            if cost > dist[pos_row][pos_col] {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for edge in self.adjacents(&position) {
                let next = State {
                    cost: cost + self.risk_level(&edge),
                    position: edge,
                };

                // If so, add it to the frontier and continue
                if next.cost < Self::get_dist_cost(&dist, &next.position) {
                    // Relaxation, we have now found a better way
                    Self::set_dist_cost(&mut dist, &next.position, &next.cost);
                    heap.push(next);
                }
            }
        }

        return dist;
    }

    pub fn get_dist_cost(dist: &HeapGrid, pos: &Point) -> usize {
        let (pos_row, pos_col) = pos;
        if *pos_row >= dist.len() || *pos_col >= dist.len() {
            return 0;
        }
        return dist[*pos_row][*pos_col];
    }

    fn set_dist_cost(dist: &mut HeapGrid, pos: &Point, cost: &usize) {
        let (pos_row, pos_col) = pos;
        if *pos_row < dist.len() {
            dist[*pos_row][*pos_col] = *cost;
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    position: Point,
    cost: usize,
}

impl State {
    fn new(position: Point, cost: usize) -> State {
        return State {
            position: position,
            cost: cost,
        };
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        return self.position == other.position;
    }
}

impl Eq for State {}

use std::hash::{Hash, Hasher};
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

use std::cmp::Ordering;

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// (Point, Point), (Point, usize)
#[derive(Debug, Copy, Clone)]
struct Exit {
    adj_meta: Point,
    adj_point: Point,
    exit_meta: Point,
    exit_point: Point,
    exit_cost: usize,
    adj_risk: usize,
}

impl PartialEq for Exit {
    fn eq(&self, other: &Self) -> bool {
        return self.exit_cost == other.exit_cost
            && self.exit_point == other.exit_point
            && self.exit_meta == other.exit_meta;
    }
}

impl Eq for Exit {}

impl Ord for Exit {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .exit_cost
            .cmp(&self.exit_cost)
            .then_with(|| self.exit_point.cmp(&other.exit_point))
    }
}

impl PartialOrd for Exit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
