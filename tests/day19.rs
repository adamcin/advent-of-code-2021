mod common;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

/// --- Day 19: Beacon Scanner ---
///
/// As your probe drifted down through this area, it released an assortment of beacons and
/// scanners into the water. It's difficult to navigate in the pitch black open waters of
/// the ocean trench, but if you can build a map of the trench using data from the scanners,
/// you should be able to safely reach the bottom.
///
/// The beacons and scanners float motionless in the water; they're designed to maintain
/// the same position for long periods of time. Each scanner is capable of detecting all
/// beacons in a large cube centered on the scanner; beacons that are at most 1000 units
/// away from the scanner in each of the three axes (x, y, and z) have their precise
/// position determined relative to the scanner. However, scanners cannot detect other
/// scanners. The submarine has automatically summarized the relative positions of beacons
/// detected by each scanner (your puzzle input).
fn read() -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();
    let mut pings: Vec<Ping> = Vec::new();
    for line in common::read_test_input("data/day-19/input.txt") {
        if line.starts_with("---") {
            continue;
        }
        if line.is_empty() {
            if !pings.is_empty() {
                scanners.push(Scanner::new(scanners.len(), pings));
                pings = Vec::new();
            }
            continue;
        }
        if let Some((xs, rest)) = line.split_once(",") {
            if let Some((ys, zs)) = rest.split_once(",") {
                if let (Some(x), Some(y), Some(z)) =
                    (xs.parse().ok(), ys.parse().ok(), zs.parse().ok())
                {
                    pings.push((x, y, z));
                }
            }
        }
    }
    if !pings.is_empty() {
        scanners.push(Scanner::new(scanners.len(), pings));
    }
    return scanners;
}

/// a point defined by relative distance from an origin in a coordinate system of
/// unknown orientation
type Ping = (isize, isize, isize);
type PingKey = (isize, isize, isize);

/// a pair of pings (a line in space)
type Pair = [Ping; 2];
type PairKey = (Ping, Ping);

type Tri = [Ping; 3];
type TriDist = [Dist; 3];
type TriDistKey = (Dist, Dist, Dist);

/// each scanner will first have to triangulate all exclusive sets of 4 pings
type Tetra = [Ping; 4];
/// elements sorted according to increasing relevant distance
/// .0 -> shortest distance -> .1 -> next shortest -> .2 -> etc. -> .3 -> longest distance back to .0
type TetraKey = (Ping, Ping, Ping, Ping);
/// as sets of 6 tetrahedron edge lengths
type TetraDist = [Dist; 6];
type TetraDistKey = (Dist, Dist, Dist, Dist, Dist, Dist);

const PING_ORIGIN: Ping = (0, 0, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dist {
    v: isize,
}

impl PartialOrd for Dist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Dist {
    fn cmp(&self, other: &Self) -> Ordering {
        other.v.cmp(&self.v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DistMap {
    ping: PingKey,
    dist: Dist,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for DistMap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist
            .cmp(&other.dist)
            .then_with(|| self.ping.cmp(&other.ping))
    }
}

impl PartialOrd for DistMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn ping_to_key(ping: &Ping) -> PingKey {
    return *ping;
}

fn ping_key_to_ping(ping_key: &PingKey) -> Ping {
    return *ping_key;
}

fn tri_dist_to_key(tri_dist: &TriDist) -> TriDistKey {
    let mut arr = *tri_dist;
    arr.sort();
    return (arr[0], arr[1], arr[2]);
}

/// return a TetraKey if all relative distances are different.
fn tetra_to_key(tetra: &Tetra) -> Option<TetraKey> {
    let mut pairs: Vec<(PairKey, isize)> = Vec::new();
    for left in 0..3 {
        for right in (left + 1)..4 {
            let dist = ping_distance(&tetra[left], &tetra[right]).v;
            pairs.push(((tetra[left], tetra[right]), dist));
        }
    }
    pairs.sort_by_key(|((_, _), dist)| *dist);
    if let Some((head_pair, head_dist)) = pairs.first() {
        if pairs.iter().filter(|(_, d)| d == head_dist).count() > 1 {
            //println!("tetra_to_key: discarding tetra for non-unique lowest dists: {:?}", pairs);
            return None;
        }
        if let Some((other_pair, _)) = pairs
            .iter()
            .filter(|(pair, _)| are_distinct_pair_keys(head_pair, pair))
            .nth(0)
        {
            let (left, right) = head_pair;
            let from_right = order_by_inner_dist(right, &other_pair);
            let from_left = order_by_inner_dist(left, &other_pair);
            return match (from_right, from_left) {
                (Some((dml, dmr)), None) => Some((
                    *left,
                    *right,
                    ping_key_to_ping(&dml.ping),
                    ping_key_to_ping(&dmr.ping),
                )),
                (None, Some((dml, dmr))) => Some((
                    *right,
                    *left,
                    ping_key_to_ping(&dml.ping),
                    ping_key_to_ping(&dmr.ping),
                )),
                (Some((fr_dml, fr_dmr)), Some((fl_dml, fl_dmr))) => {
                    if fr_dml.dist.v < fl_dml.dist.v {
                        Some((
                            *left,
                            *right,
                            ping_key_to_ping(&fr_dml.ping),
                            ping_key_to_ping(&fr_dmr.ping),
                        ))
                    } else if fr_dml.dist.v > fl_dml.dist.v {
                        Some((
                            *right,
                            *left,
                            ping_key_to_ping(&fl_dml.ping),
                            ping_key_to_ping(&fl_dmr.ping),
                        ))
                    } else {
                        //println!("tetra_to_key: discarding tetra for equidistance: ({:?}) -> ({:?})", head_pair, other_pair);
                        None
                    }
                }
                _ => None,
            };
        }
    }
    None
}

fn order_by_inner_dist(from: &Ping, other_pair: &PairKey) -> Option<(DistMap, DistMap)> {
    let (other1, other2) = other_pair;
    let dist1 = ping_distance(from, other1);
    let dist2 = ping_distance(from, other2);
    if dist1 != dist2 {
        if dist1.v < dist2.v {
            return Some((
                DistMap {
                    ping: ping_to_key(other1),
                    dist: dist1,
                },
                DistMap {
                    ping: ping_to_key(other2),
                    dist: dist2,
                },
            ));
        } else {
            return Some((
                DistMap {
                    ping: ping_to_key(other2),
                    dist: dist2,
                },
                DistMap {
                    ping: ping_to_key(other1),
                    dist: dist1,
                },
            ));
        }
    } else {
        //println!("order_by_inner_dist: discarding tetra for ping: {:?}", from);
    }
    None
}

fn pair_distance(from: &Pair, to: &Pair) -> Dist {
    [
        ping_distance(&from[0], &to[0]),
        ping_distance(&from[0], &to[1]),
        ping_distance(&from[1], &to[0]),
        ping_distance(&from[1], &to[1]),
    ]
    .iter()
    .fold(Dist { v: 0 }, |a, v| std::cmp::max_by_key(a, *v, |d| d.v))
}

fn ping_distance(from: &Ping, to: &Ping) -> Dist {
    let ((from_x, from_y, from_z), (to_x, to_y, to_z)) = (*from, *to);

    let dx = to_x - from_x;
    let dy = to_y - from_y;
    let dz = to_z - from_z;
    Dist {
        v: ((dz.pow(2) + dx.pow(2) + dy.pow(2)) as f64).sqrt().round() as isize,
    }
}

fn are_distinct_pairs(left: &Pair, right: &Pair) -> bool {
    left[0] != right[0] && left[0] != right[1] && left[1] != right[0] && left[1] != right[1]
}

fn are_distinct_pair_keys(left: &PairKey, right: &PairKey) -> bool {
    let ((left0, left1), (right0, right1)) = (left, right);
    left0 != right0 && left0 != right1 && left1 != right0 && left1 != right1
}

#[derive(Debug, Clone)]
struct Scanner {
    id: usize,
    origin: Ping,
    relative_to: usize,
    pings: Vec<Ping>,
    ping_dists: HashMap<PingKey, TriDistKey>,
}

impl Scanner {
    fn new(id: usize, pings: Vec<Ping>) -> Self {
        let ping_dists = Self::map_dists(&pings);
        let scanner = Self {
            id: id,
            origin: PING_ORIGIN,
            relative_to: id,
            pings: pings,
            ping_dists,
        };
        //scanner.sort_my_pings();
        scanner
    }

    fn transform(&self, relative_to: usize, transform: &Vec<Array2<isize>>) -> Self {
        let pings = self
            .pings
            .iter()
            .map(|ping| Self::transform_ping(ping, transform))
            .collect();
        let ping_dists = Self::map_dists(&pings);
        return Self {
            id: self.id,
            origin: Self::transform_ping(&self.origin, transform),
            relative_to: relative_to,
            pings: pings,
            ping_dists: ping_dists,
        };
    }

    fn transform_ping(ping: &Ping, transform: &Vec<Array2<isize>>) -> Ping {
        return mat_to_ping(
            &transform
                .iter()
                .fold(ping_to_mat(ping), |ping_mat, t| t.dot(&ping_mat)),
        );
    }

    fn map_best_dists(pings: &Vec<Ping>, ping: &Ping, n_dists: usize) -> Vec<DistMap> {
        let mut heap: BinaryHeap<DistMap> = BinaryHeap::new();
        for other in pings {
            if ping != other {
                let dist = ping_distance(ping, other);
                heap.push(DistMap {
                    ping: ping_to_key(other),
                    dist: dist,
                });
            }
        }
        return heap.iter().take(n_dists).cloned().collect();
    }

    fn get_tetra_for_tri_key(&self, key: &TriDistKey) -> Option<Tetra> {
        if let Some((ping, _)) = self
            .ping_dists
            .iter()
            .filter(|(ping, dist_key)| *dist_key == key)
            .nth(0)
        {
            let dists = Self::map_best_dists(&self.pings, ping, 3);
            if dists.len() >= 3 {
                return Some([*ping, dists[0].ping, dists[1].ping, dists[2].ping]);
            }
        }
        None
    }

    fn map_dist(pings: &Vec<Ping>, ping: &Ping) -> TriDistKey {
        let dists = Self::map_best_dists(pings, ping, 3);

        let mut tri_dist = [Dist { v: 0 }; 3];
        if dists.len() >= 3 {
            for i in 0..3 {
                tri_dist[i] = dists[i].dist;
            }
        }
        return tri_dist_to_key(&tri_dist);
    }

    fn map_dists(pings: &Vec<Ping>) -> HashMap<PingKey, TriDistKey> {
        let mut dists: HashMap<PingKey, TriDistKey> = HashMap::new();
        for ping in pings {
            let key = ping_to_key(ping);
            let val = Self::map_dist(pings, ping);
            dists.insert(key, val);
        }
        return dists;
    }

    fn sort_my_pings(&mut self) {
        // sort by ping as a tuple
        Self::sort_pings(&mut self.pings);
    }

    fn sort_pings(ping_vec: &mut Vec<Ping>) {
        ping_vec.sort_by_key(|ping| *ping);
    }

    fn triangulate(&self, tetra: &Tetra) -> TetraDist {
        let mut hash: TetraDist = [Dist { v: 0 }; 6];
        for from in 0..3 {
            for to in (from + 1)..=3 {
                if from == 0 {
                    // 0->1,0->2,0->3
                    hash[to - 1] = ping_distance(&tetra[from], &tetra[to]);
                } else {
                    // 1->2,1->3
                    // 2->3
                    hash[from + to] = ping_distance(&tetra[from], &tetra[to]);
                }
            }
        }
        hash.sort();
        return hash;
    }

    fn may_intersect(&self, other: &Self) -> HashSet<TriDistKey> {
        let my_dists: HashSet<_> = HashSet::from_iter(self.ping_dists.values().cloned());
        let other_dists: HashSet<_> = HashSet::from_iter(other.ping_dists.values().cloned());
        return my_dists.intersection(&other_dists).cloned().collect();
    }

    fn get_inxn(&self, other: &Self) -> Option<(Tetra, Tetra)> {
        let tri_keys = self.may_intersect(other);
        for tri_key in tri_keys {
            if let (Some(my_tetra), Some(other_tetra)) = (
                self.get_tetra_for_tri_key(&tri_key),
                other.get_tetra_for_tri_key(&tri_key),
            ) {
                let my_dists = self.triangulate(&my_tetra);
                let other_dists = other.triangulate(&other_tetra);
                if my_dists == other_dists {
                    //println!("get_inxn MATCH: my_tetra_dists:{:?} other_tetra_dists:{:?}", my_dists, other_dists);
                    return Some((my_tetra, other_tetra));
                } else {
                    //println!("get_inxn MISS : my_tetra_dists:{:?} other_tetra_dists:{:?}", my_dists, other_dists);
                }
                // if let (Some(my_tetra_key), Some(other_tetra_key)) = (tetra_to_key(&my_tetra), tetra_to_key(&other_tetra)) {
                //     if my_tetra_key == other_tetra_key {
                //         println!("get_inxn MATCH: my_tetra_key:{:?} other_tetra_key:{:?}", my_tetra_key, other_tetra_key);
                //         return Some((my_tetra, other_tetra));
                //     } else {
                //         println!("get_inxn MISS : my_tetra_key:{:?} other_tetra_key:{:?}", my_tetra_key, other_tetra_key);
                //     }
                // }
            }
        }
        None
    }

    fn triangulate_all(&self) -> HashMap<TetraDist, Tetra> {
        HashMap::new()
    }
}

fn dist_dims(from: &Ping, to: &Ping) -> TriDistKey {
    let ((from_x, from_y, from_z), (to_x, to_y, to_z)) = (from, to);
    (
        Dist { v: to_x - from_x },
        Dist { v: to_y - from_y },
        Dist { v: to_z - from_z },
    )
}

fn dist_dims_raw(from: &Ping, to: &Ping) -> (isize, isize, isize) {
    let ((from_x, from_y, from_z), (to_x, to_y, to_z)) = (from, to);
    (to_x - from_x, to_y - from_y, to_z - from_z)
}

use ndarray::arr2;
use ndarray::Array2;

/// return a list of transforms to reorient the right hand coordinate system
///
/// translation matrix:
/// [[1, 0, 0, tx],
///  [0, 1, 0, ty],
///  [0, 0, 1, tz],
///  [0, 0, 0, 1 ]],
///
/// rotation matrices:
/// Rx:
/// [[1, 0,     0,  0],
///  [0, cos,   sin,0],
///  [0, -sin,  cos,0],
///  [0, 0,     0,  1]],
///
/// Ry:
/// [[cos,  0,  -sin,   0],
///  [0,    1,  0,      0],
///  [sin,  0,  cos,    0],
///  [0,    0,  0,      1]],
///
/// Rz:
/// [[cos,  -sin,   0,  0],
///  [sin,  cos,    0,  0],
///  [0,    0,      1,  0],
///  [0,    0,      0,  1]],
///
/// cos(180) = -1
/// cos(90|-90) = 0
/// sin(180) = 0
/// sin(-90) = -1
/// sin(90)  =  1
///
fn transform_for(left_t: &Tetra, right_t: &Tetra) -> Vec<Array2<isize>> {
    let (lkping, rkping) = (left_t[0], right_t[0]);
    let ltest = left_t[1];
    let ldist = ping_distance(&lkping, &ltest);

    if let Some(rtest_base) = right_t[1..]
        .iter()
        .filter(|p| ping_distance(&rkping, p) == ldist)
        .nth(0)
    {
        let (ldx, ldy, ldz) = dist_dims_raw(&lkping, &ltest);
        let mut transforms: Vec<Array2<isize>> = Vec::new();
        let mut rfrom: Ping = rkping;
        let mut rtest: Ping = *rtest_base;
        while &ltest != &rtest {
            let (rdx, rdy, rdz) = dist_dims_raw(&rfrom, &rtest);
            let txfm = if rdx == ldx && rdy == -ldy && rdz == -ldz {
                Some(t_rotate_x(Twice))
            } else if rdx == -ldx && rdy == ldy && rdz == -ldz {
                Some(t_rotate_y(Twice))
            } else if rdx == -ldx && rdy == -ldy && rdz == ldz {
                Some(t_rotate_z(Twice))
            } else if rdy == -ldz || rdz == ldy {
                Some(t_rotate_x(Cw))
            } else if rdy == ldz || rdz == -ldy {
                Some(t_rotate_x(Ccw))
            } else if rdx == ldz || rdz == -ldx {
                Some(t_rotate_y(Cw))
            } else if rdx == -ldz || rdz == ldx {
                Some(t_rotate_y(Ccw))
            } else if rdx == -ldy || rdy == ldx {
                Some(t_rotate_z(Cw))
            } else if rdx == ldy || rdy == -ldx {
                Some(t_rotate_z(Ccw))
            } else if rdx == ldx && rdy == ldy && rdz == ldz {
                Some(t_translate(&ltest, &rtest))
            } else {
                println!("??? {:?} <> {:?}", (ldx, ldy, ldz), (rdx, rdy, rdz));
                return Vec::new();
            };
            if let Some(txfm) = txfm {
                println!(">>> {:?} -> {:?}", ltest, rtest);
                rfrom = mat_to_ping(&txfm.dot(&ping_to_mat(&rfrom)));
                rtest = mat_to_ping(&txfm.dot(&ping_to_mat(&rtest)));
                transforms.push(txfm);
                println!("<<< {:?} -> {:?}", ltest, rtest);
            } else {
                break;
            }
        }
        println!("=== {:?} == {:?}", ltest, rtest);
        return transforms;
    }
    Vec::new()
}

/// translation matrix:
/// [[1, 0, 0, tx],
///  [0, 1, 0, ty],
///  [0, 0, 1, tz],
///  [0, 0, 0, 1 ]],
fn t_translate(from: &Ping, to: &Ping) -> Array2<isize> {
    let ((lx, ly, lz), (rx, ry, rz)) = (from, to);
    arr2(&[
        [1, 0, 0, lx - rx],
        [0, 1, 0, ly - ry],
        [0, 0, 1, lz - rz],
        [0, 0, 0, 1],
    ])
}

enum RotDir {
    Cw,
    Ccw,
    Twice,
}
use RotDir::*;

/// Rx:
/// [[1, 0,     0,  0],
///  [0, cos,   sin,0],
///  [0, -sin,  cos,0],
///  [0, 0,     0,  1]],
///
/// cos(180) = -1
/// cos(90|-90) = 0
/// sin(180) = 0
/// sin(-90) = -1
/// sin(90)  =  1
fn t_rotate_x(dir: RotDir) -> Array2<isize> {
    return match dir {
        Twice => arr2(&[[1, 0, 0, 0], [0, -1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]]),
        Cw => arr2(&[[1, 0, 0, 0], [0, 0, 1, 0], [0, -1, 0, 0], [0, 0, 0, 1]]),
        Ccw => arr2(&[[1, 0, 0, 0], [0, 0, -1, 0], [0, 1, 0, 0], [0, 0, 0, 1]]),
    };
}

/// Ry:
/// [[cos,  0,  -sin,   0],
///  [0,    1,  0,      0],
///  [sin,  0,  cos,    0],
///  [0,    0,  0,      1]],
///
/// cos(180) = -1
/// cos(90|-90) = 0
/// sin(180) = 0
/// sin(-90) = -1
/// sin(90)  =  1
fn t_rotate_y(dir: RotDir) -> Array2<isize> {
    return match dir {
        Twice => arr2(&[[-1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]]),
        Cw => arr2(&[[0, 0, -1, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1]]),
        Ccw => arr2(&[[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]]),
    };
}

/// Rz:
/// [[cos,  -sin,   0,  0],
///  [sin,  cos,    0,  0],
///  [0,    0,      1,  0],
///  [0,    0,      0,  1]],
///
/// cos(180) = -1
/// cos(90|-90) = 0
/// sin(180) = 0
/// sin(-90) = -1
/// sin(90)  =  1
fn t_rotate_z(dir: RotDir) -> Array2<isize> {
    return match dir {
        Twice => arr2(&[[-1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]]),
        Cw => arr2(&[[0, 0, -1, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1]]),
        Ccw => arr2(&[[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]]),
    };
}

fn mat_to_ping(mat: &Array2<isize>) -> Ping {
    (mat[(0, 0)], mat[(1, 0)], mat[(2, 0)])
}

fn ping_to_mat(ping: &Ping) -> Array2<isize> {
    let (x, y, z) = ping;
    arr2(&[[*x], [*y], [*z], [1]])
}

/// if left_x == -right_x and left_z == -right_z, rotate right 180deg around y-axis
#[test]
fn day19part0_negX_sameY_negZ() {
    let scanners = read();

    let seven = &scanners[7];
    let nine = &scanners[9];
    if let Some((lixn, rixn)) = seven.get_inxn(nine) {
        let transforms = transform_for(&lixn, &rixn);
        println!("{:?}", transforms);
    }
}

/// 90deg cw Z, 90deg c-cw X
#[test]
fn day19part0_xtoy_ytonegz_ztonegx() {
    let scanners = read();

    let zero = &scanners[0];
    let seven = &scanners[7];
    if let Some((lixn, rixn)) = zero.get_inxn(seven) {
        let transforms = transform_for(&lixn, &rixn);
        println!("{:?}", transforms);
    }
}

#[test]
fn day19part1() {
    let mut scanners = read();
    assert_eq!(26, scanners.len(), "expect number of scanners");

    assert_eq!(
        26,
        scanners.last().unwrap().pings.len(),
        "expect number of pings in last scanner: {:?}",
        scanners.last().unwrap(),
    );

    for left in 0..(scanners.len() - 1) {
        for right in (left + 1)..scanners.len() {
            let lscan = &scanners[left];
            let rscan = &scanners[right];
            if let Some((left_t, right_t)) = lscan.get_inxn(rscan) {
                let transform = transform_for(&left_t, &right_t);
                scanners[right] = rscan.transform(lscan.id, &transform);
            }
        }
    }

    for i in 0..scanners.len() {
        let scanner = &scanners[i];
        println!("{} origin {:?}", scanner.id, scanner.origin);
    }

    for left in 0..(scanners.len() - 1) {
        'right_loop: for right in (left + 1)..scanners.len() {
            let lscan = &scanners[right];
            let rscan = &scanners[left];
            if lscan.origin != PING_ORIGIN && rscan.origin == PING_ORIGIN {
                continue 'right_loop;
            }
            if let Some((left_t, right_t)) = lscan.get_inxn(rscan) {
                let transform = transform_for(&left_t, &right_t);
                scanners[left] = rscan.transform(lscan.id, &transform);
            }
        }
    }

    for i in 0..scanners.len() {
        let scanner = &scanners[i];
        println!(
            "{} origin {:?} relative to: {}",
            scanner.id, scanner.origin, scanner.relative_to
        );
    }
}
