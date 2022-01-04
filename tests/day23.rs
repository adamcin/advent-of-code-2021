mod common;

/// Starting positions:
/// #############
/// #...........#
/// ###D#D#A#A###
///   #C#C#B#B#
///   #########
///
/// Solution positions:
/// #############
/// #...........#
/// ###A#B#C#D###
///   #A#B#C#D#
///   #########
///
/// Position codes:
/// (i) inner: one step to leave room
/// (o) outer: two steps to leave room
/// (H) hallway
/// (G) goal room
/// (L) left room
/// (R) right room
/// Ha, Hb, Hc, Hd: immediate hallway spaces (no loitering) outside each goal room
/// Hab, Hbc, Hcd: hallway spaces between goal rooms (loitering allowed)
/// Goa, Gia, Gob, Gib, Goc, Gic, God, Gid: outer and inner goals for amphipods
/// Lo, Li, Ri, Ro: left and right hallways, inner/outer positions
///
/// test start:
/// #############
/// #...........#
/// ###B#C#B#D###
///   #A#D#C#A#
///   #########
fn read() -> Burrow {
    Burrow::new((D, D, A, A), (C, C, B, B))
}

fn read_test() -> Burrow {
    Burrow::new((B, C, B, D), (A, D, C, A))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pod {
    A,
    B,
    C,
    D,
}
use Pod::*;
impl Pod {
    fn is_dest(&self, pos: &Pos) -> bool {
        match (self, pos) {
            (A, Gia)
            | (A, Goa)
            | (B, Gib)
            | (B, Gob)
            | (C, Gic)
            | (C, Goc)
            | (D, Gid)
            | (D, God) => true,
            _ => false,
        }
    }
    fn energy(&self) -> usize {
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pos {
    Lo,
    Li,
    Ha,
    Goa,
    Gia,
    Hab,
    Hb,
    Gob,
    Gib,
    Hbc,
    Hc,
    Goc,
    Gic,
    Hcd,
    Hd,
    God,
    Gid,
    Ro,
    Ri,
}
use Pos::*;
impl Pos {
    fn neighbors_in(&self, empties: &Vec<Self>) -> bool {
        match self {
            Lo | Goa | Gob | Goc | God | Ro => empties.contains(&self.as_inner()),
            Li | Gia | Gib | Gic | Gid | Ri => {
                empties.contains(&self.as_outer())
                    || self
                        .get_inner_hall()
                        .as_ref()
                        .unwrap()
                        .neighbors_in(empties)
            }
            Ha => [Li, Gia, Hab]
                .iter()
                .fold(false, |a, v| a || empties.contains(v)),
            Hab => [Ha, Hb].iter().fold(false, |a, v| a || empties.contains(v)),
            Hb => [Gib, Hab, Hbc]
                .iter()
                .fold(false, |a, v| a || empties.contains(v)),
            Hbc => [Hb, Hc].iter().fold(false, |a, v| a || empties.contains(v)),
            Hc => [Gic, Hbc, Hcd]
                .iter()
                .fold(false, |a, v| a || empties.contains(v)),
            Hcd => [Hc, Hd].iter().fold(false, |a, v| a || empties.contains(v)),
            Hd => [Ri, Gid, Hcd]
                .iter()
                .fold(false, |a, v| a || empties.contains(v)),
        }
    }

    fn is_hallway(&self) -> bool {
        match self {
            Lo | Li | Ha | Hab | Hb | Hbc | Hc | Hcd | Hd | Ri | Ro => true,
            _ => false,
        }
    }

    fn is_room(&self) -> bool {
        !self.is_hallway()
    }

    fn goal_for_pod(&self) -> Option<Pod> {
        match self {
            Gia | Goa => Some(A),
            Gib | Gob => Some(B),
            Gic | Goc => Some(C),
            Gid | God => Some(D),
            _ => None,
        }
    }

    fn is_same_room(&self, other: &Self) -> bool {
        match (self, other) {
            (Gia, Goa)
            | (Goa, Gia)
            | (Gib, Gob)
            | (Gob, Gib)
            | (Gic, Goc)
            | (Goc, Gic)
            | (Gid, God)
            | (God, Gid) => true,
            _ => false,
        }
    }

    fn is_no_loiter(&self) -> bool {
        match self {
            Ha | Hb | Hc | Hd => true,
            _ => false,
        }
    }

    fn is_inner(&self) -> bool {
        match self {
            Li | Gia | Gib | Gic | Gid | Ri => true,
            _ => false,
        }
    }

    fn is_outer(&self) -> bool {
        match self {
            Lo | Goa | Gob | Goc | God | Ro => true,
            _ => false,
        }
    }

    fn as_inner(&self) -> Self {
        if self.is_outer() {
            return match self {
                Lo => Li,
                Goa => Gia,
                Gob => Gib,
                Goc => Gic,
                God => Gid,
                Ro => Ri,
                _ => unreachable!(),
            };
        }
        *self
    }

    fn as_outer(&self) -> Self {
        if self.is_inner() {
            return match self {
                Li => Lo,
                Gia => Goa,
                Gib => Gob,
                Gic => Goc,
                Gid => God,
                Ri => Ro,
                _ => unreachable!(),
            };
        }
        *self
    }

    fn get_hall_to_left(&self) -> Option<Self> {
        match self {
            Hab => Some(Ha),
            Hb => Some(Hab),
            Hbc => Some(Hb),
            Hc => Some(Hbc),
            Hcd => Some(Hc),
            Hd => Some(Hcd),
            _ => None,
        }
    }

    fn get_hall_to_right(&self) -> Option<Self> {
        match self {
            Hcd => Some(Hd),
            Hc => Some(Hcd),
            Hbc => Some(Hc),
            Hb => Some(Hbc),
            Hab => Some(Hb),
            Ha => Some(Hab),
            _ => None,
        }
    }

    fn is_hall_to_left(&self, other: &Self) -> bool {
        self.get_hall_to_left()
            .map(|h| h == *other || h.is_hall_to_left(other))
            .unwrap_or(false)
    }

    fn is_hall_to_right(&self, other: &Self) -> bool {
        self.get_hall_to_right()
            .map(|h| h == *other || h.is_hall_to_right(other))
            .unwrap_or(false)
    }

    fn get_inner_hall(&self) -> Option<Self> {
        match self {
            Li | Gia => Some(Ha),
            Gib => Some(Hb),
            Gic => Some(Hc),
            Ri | Gid => Some(Hd),
            _ => None,
        }
    }

    /// #############
    /// #...........#
    /// ###A#B#C#D###
    ///   #A#B#C#D#
    ///   #########
    fn path_to(&self, other: &Self) -> Option<Vec<Self>> {
        if self == other {
            return Some(vec![]);
        } else if self.is_outer() {
            let maybe_from_inner = self.as_inner().path_to(other);
            return maybe_from_inner
                .map(|from_inner| vec![vec![self.as_inner()], from_inner].concat());
        } else if other.is_outer() {
            let maybe_to_inner = self.path_to(&other.as_inner());
            return maybe_to_inner.map(|to_inner| vec![to_inner, vec![*other]].concat());
        } else if self.is_inner() {
            let maybe_inner_hall = self.get_inner_hall();
            return maybe_inner_hall.and_then(|inner_hall| {
                inner_hall
                    .path_to(other)
                    .map(|to_inner_hall| vec![vec![inner_hall], to_inner_hall].concat())
            });
        } else if other.is_inner() {
            let maybe_inner_hall = &other.get_inner_hall();
            return maybe_inner_hall.and_then(|inner_hall| {
                self.path_to(&inner_hall)
                    .map(|from_inner_hall| vec![from_inner_hall, vec![*other]].concat())
            });
        } else {
            if self.is_hall_to_left(other) {
                let maybe_hall_to_left = self.get_hall_to_left();
                return maybe_hall_to_left.and_then(|hall_to_left| {
                    hall_to_left
                        .path_to(other)
                        .map(|to_hall_to_left| vec![vec![hall_to_left], to_hall_to_left].concat())
                });
            } else if self.is_hall_to_right(other) {
                let maybe_hall_to_right = self.get_hall_to_right();
                return maybe_hall_to_right.and_then(|hall_to_right| {
                    hall_to_right.path_to(other).map(|to_hall_to_right| {
                        vec![vec![hall_to_right], to_hall_to_right].concat()
                    })
                });
            } else {
                return None;
            }
        }
    }

    fn all() -> Vec<Pos> {
        vec![
            Lo, Li, Ha, Hab, Hb, Hbc, Hc, Hcd, Hd, Ri, Ro, Gia, Gib, Gic, Gid, Goa, Gob, Goc, God,
        ]
    }
}

#[derive(Debug, Clone)]
struct PosPaths {
    paths: HashMap<(Pos, Pos), Vec<Pos>>,
}

impl PosPaths {
    fn new() -> Self {
        Self {
            paths: Self::generate_paths(),
        }
    }

    fn steps(&self, from: &Pos, to: &Pos) -> usize {
        if let Some(path) = self.paths.get(&(*from, *to)) {
            return path.len();
        }
        0
    }

    fn check_path(&self, from: &Pos, to: &Pos, occupieds: &Vec<Pos>) -> bool {
        if let Some(path) = self.paths.get(&(*from, *to)) {
            return !path.is_empty()
                && !occupieds
                    .iter()
                    .fold(false, |acc, occ| acc || path.contains(occ));
        }
        false
    }

    fn generate_paths() -> HashMap<(Pos, Pos), Vec<Pos>> {
        let mut all_paths: HashMap<(Pos, Pos), Vec<Pos>> = HashMap::new();
        for from in Pos::all() {
            for to in Pos::all() {
                if from == to {
                    continue;
                }
                if let Some(path) = from.path_to(&to) {
                    all_paths.insert((from, to), path);
                }
            }
        }
        all_paths
    }
}

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Burrow {
    energy: usize,
    places: HashMap<Pos, Pod>,
    history: HashSet<Vec<(Pos, Pod)>>,
}

impl Burrow {
    fn new(inner: (Pod, Pod, Pod, Pod), outer: (Pod, Pod, Pod, Pod)) -> Self {
        let (gia, gib, gic, gid) = inner;
        let (goa, gob, goc, god) = outer;
        let places: HashMap<Pos, Pod> = HashMap::from([
            (Gia, gia),
            (Gib, gib),
            (Gic, gic),
            (Gid, gid),
            (Goa, goa),
            (Gob, gob),
            (Goc, goc),
            (God, god),
        ]);
        Self {
            energy: 0,
            places: places,
            history: HashSet::new(),
        }
    }

    fn pod_at(&self, pos: Pos) -> Option<Pod> {
        self.places.get(&pos).cloned()
    }

    fn pods(&self) -> Vec<(Pod, Pos)> {
        self.places.iter().map(|(pos, pod)| (*pod, *pos)).collect()
    }

    fn empties(&self) -> Vec<Pos> {
        Pos::all()
            .iter()
            .filter(|pos| !pos.is_no_loiter() && !self.places.contains_key(pos))
            .cloned()
            .collect()
    }

    fn pos_str(&self, pos: Pos) -> String {
        match self.pod_at(pos) {
            Some(pod) => format!("{:?}", pod),
            None => ".".to_owned(),
        }
    }

    fn pods_at(&self, poss: (Pos, Pos)) -> Option<(Pod, Pod)> {
        let (pos1, pos2) = poss;
        self.places
            .get(&pos1)
            .cloned()
            .zip(self.places.get(&pos2).cloned())
    }

    fn copy(&self) -> Self {
        Self {
            energy: self.energy,
            places: self.places.to_owned(),
            history: self.history.to_owned(),
        }
    }

    fn find_solutions(&self, paths: &PosPaths) -> Vec<Self> {
        if self.is_solution() {
            println!("{}", self.to_string());
            return vec![self.copy()];
        } else {
            let next_moves = self.list_moves(paths);
            let mut solutions: Vec<Self> = Vec::new();
            for next_move in next_moves.iter() {
                let next_bur = self.perform_move(paths, *next_move);
                if next_bur.is_dead_end() {
                    println!("dead end:\n{}", next_bur.to_string());
                    continue;
                }
                solutions.extend(next_bur.find_solutions(paths).into_iter());
            }
            return solutions;
        }
    }

    fn places(&self) -> Vec<(Pos, Pod)> {
        let mut sorted: Vec<_> = self.places.iter().map(|(pos, pod)|(*pos, *pod)).collect();
        sorted.sort();
        sorted
    }

    fn is_dead_end(&self) -> bool {
        self.history.contains(&self.places())
    }

    fn list_moves(&self, paths: &PosPaths) -> Vec<(Pos, Pos)> {
        let empties = self.empties();
        let pods: Vec<(Pod, Pos)> = self
            .pods()
            .iter()
            .filter(|(_, pos)| pos.neighbors_in(&empties))
            .cloned()
            .collect();
        let occupieds: Vec<Pos> = pods.iter().map(|(_, pos)| pos).cloned().collect();
        let no_reentry: Vec<Pod> = pods
            .iter()
            .flat_map(|(pod, pos)| pos.goal_for_pod().filter(|gpod| gpod != pod))
            .collect();
        let pre_moves: Vec<(Pod, (Pos, Pos))> = pods
            .iter()
            .flat_map(|(pod, pos)| -> Vec<_> {
                empties
                    .iter()
                    .filter(|empty| {
                        (pod.is_dest(empty) && !no_reentry.contains(&pod))
                            || pos.is_same_room(empty)
                            || (pos.is_room() && empty.is_hallway())
                    })
                    .map(|empty| (*pod, (*pos, *empty)))
                    .collect()
            })
            .collect();
        let may_moves: Vec<(Pod, (Pos, Pos))> = pre_moves
            .iter()
            .filter(|(_, (from, to))| paths.check_path(from, to, &occupieds))
            .cloned()
            .collect();
        may_moves.iter().map(|(_, mv)| mv).cloned().collect()
    }

    fn perform_move(&self, paths: &PosPaths, mv: (Pos, Pos)) -> Self {
        let (from, to) = mv;
        let mut new_places = self.places.to_owned();
        let steps = paths.steps(&from, &to);
        let may_pod = new_places.remove(&from);
        let add_energy = match may_pod {
            Some(pod) => pod.energy() * steps,
            _ => 0,
        };
        if let Some(pod) = may_pod {
            new_places.insert(to, pod);
        }
        let mut history = self.history.to_owned();
        history.insert(self.places());
        Self {
            energy: self.energy + add_energy,
            places: new_places,
            history: history,
        }
    }

    fn is_solution(&self) -> bool {
        self.pods_at((Gia, Goa))
            .map(|pods| pods == (A, A))
            .unwrap_or(false)
            && self
                .pods_at((Gib, Gob))
                .map(|pods| pods == (B, B))
                .unwrap_or(false)
            && self
                .pods_at((Gic, Goc))
                .map(|pods| pods == (C, C))
                .unwrap_or(false)
            && self
                .pods_at((Gid, God))
                .map(|pods| pods == (D, D))
                .unwrap_or(false)
    }
}

/// #############
/// #...........#
/// ###A#B#C#D###
///   #A#B#C#D#
///   #########
impl ToString for Burrow {
    fn to_string(&self) -> String {
        let mut out: String = "#############\n".to_owned();
        out = out
            + format!(
                "#{}{}{}{}{}{}{}{}{}{}{}#\n",
                self.pos_str(Lo),
                self.pos_str(Li),
                self.pos_str(Ha),
                self.pos_str(Hab),
                self.pos_str(Hb),
                self.pos_str(Hbc),
                self.pos_str(Hc),
                self.pos_str(Hcd),
                self.pos_str(Hd),
                self.pos_str(Ri),
                self.pos_str(Ro),
            )
            .as_str();

        out = out
            + format!(
                "###{}#{}#{}#{}###\n",
                self.pos_str(Gia),
                self.pos_str(Gib),
                self.pos_str(Gic),
                self.pos_str(Gid),
            )
            .as_str();

        out = out
            + format!(
                "  #{}#{}#{}#{}#  \n",
                self.pos_str(Goa),
                self.pos_str(Gob),
                self.pos_str(Goc),
                self.pos_str(God),
            )
            .as_str();
        out = out + "  #########  \n";
        out = out + format!("energy:  {}\n", self.energy).as_str();
        out = out + format!("history: {}\n", self.history.len()).as_str();
        out
    }
}

#[test]
fn day23_test_out() {
    let bur = read_test();
    println!("burrow: \n{}", bur.to_string());
}

#[test]
fn day23_test_is_solution() {
    let bur = read_test();
    assert!(!bur.is_solution());

    let bur_solved = Burrow::new((A, B, C, D), (A, B, C, D));
    assert!(bur_solved.is_solution());
}

#[test]
fn day23_test_list_moves() {
    let bur = read_test();
    let paths = PosPaths::new();
    println!("paths: {:?}", paths);
}

#[test]
fn day23pre_part1() {
    let bur = read_test();
    let paths = PosPaths::new();
    let mut solutions: Vec<Burrow> = bur.find_solutions(&paths);
    solutions.sort_by_key(|b| b.energy);
    assert_eq!(12521, solutions.first().unwrap().energy);
}

