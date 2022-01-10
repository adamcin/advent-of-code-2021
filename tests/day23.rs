mod common;

/// Starting positions:
/// #############
/// #...........#
/// ###D#D#A#A###
///   #C#C#B#B#
///   #########
/// Gd0 -> Lo
/// Gd1 -> Ro
/// Gc0 -> Li
/// Gc1 -> Ri
/// Gb0 -> Gd1
/// Ga0 -> Gd0
/// Gb1 -> Gc1
/// Ga1 -> Gc0
/// Li -> Ga1
/// Lo -> Ga0
/// Ri -> Gb1
/// Ro -> Gb0
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
/// Ga1, Ga0, Gb1, Gb0, Gc1, Gc0, Gd1, Gd0: outer and inner goals for amphipods
/// Lo, Li, Ri, Ro: left and right hallways, inner/outer positions
///
/// test start:
/// #############
/// #...........#
/// ###B#C#B#D###
///   #A#D#C#A#
///   #########
fn read(half: bool) -> Burrow {
    Burrow::new(half, (D, D, A, A), (C, C, B, B))
}

fn read_test(half: bool) -> Burrow {
    Burrow::new(half, (B, C, B, D), (A, D, C, A))
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
        pos.goal_for_pod().map(|pod| pod == *self).unwrap_or(false)
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
    Ga0,
    Ga1,
    Ga2,
    Ga3,
    Gb0,
    Gb1,
    Gb2,
    Gb3,
    Gc0,
    Gc1,
    Gc2,
    Gc3,
    Gd0,
    Gd1,
    Gd2,
    Gd3,
    Lo,
    Li,
    Ha,
    Hab,
    Hb,
    Hbc,
    Hc,
    Hcd,
    Hd,
    Ri,
    Ro,
}
use Pos::*;
impl Pos {
    fn depth(&self) -> usize {
        match self {
            Ga3 | Gb3 | Gc3 | Gd3 => 3,
            Ga2 | Gb2 | Gc2 | Gd2 => 2,
            Ga1 | Gb1 | Gc1 | Gd1 | Lo | Ro => 1,
            _ => 0,
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
            Ga0 | Ga1 | Ga2 | Ga3 => Some(A),
            Gb0 | Gb1 | Gb2 | Gb3 => Some(B),
            Gc0 | Gc1 | Gc2 | Gc3 => Some(C),
            Gd0 | Gd1 | Gd2 | Gd3 => Some(D),
            _ => None,
        }
    }

    fn is_same_room(&self, other: &Self) -> bool {
        self.is_room() && other.is_room() && self.goal_for_pod() == other.goal_for_pod()
    }

    fn is_no_loiter(&self) -> bool {
        match self {
            Ha | Hb | Hc | Hd => true,
            _ => false,
        }
    }

    fn is_inner(&self) -> bool {
        match self {
            Ha | Hab | Hb | Hbc | Hc | Hcd | Hd => false,
            _ => self.depth() == 0,
        }
    }

    fn is_outer(&self) -> bool {
        self.depth() > 0
    }

    fn for_depth(&self, other_depth: usize) -> Self {
        match other_depth {
            3 => match self {
                Ga0 | Ga1 | Ga2 | Ga3 => Ga3,
                Gb0 | Gb1 | Gb2 | Gb3 => Gb3,
                Gc0 | Gc1 | Gc2 | Gc3 => Gc3,
                Gd0 | Gd1 | Gd2 | Gd3 => Gd3,
                _ => *self,
            },
            2 => match self {
                Ga0 | Ga1 | Ga2 | Ga3 => Ga2,
                Gb0 | Gb1 | Gb2 | Gb3 => Gb2,
                Gc0 | Gc1 | Gc2 | Gc3 => Gc2,
                Gd0 | Gd1 | Gd2 | Gd3 => Gd2,
                _ => *self,
            },
            1 => match self {
                Ga0 | Ga1 | Ga2 | Ga3 => Ga1,
                Gb0 | Gb1 | Gb2 | Gb3 => Gb1,
                Gc0 | Gc1 | Gc2 | Gc3 => Gc1,
                Gd0 | Gd1 | Gd2 | Gd3 => Gd1,
                Li | Lo => Lo,
                Ri | Ro => Ro,
                _ => *self,
            },
            0 => match self {
                Ga0 | Ga1 | Ga2 | Ga3 => Ga0,
                Gb0 | Gb1 | Gb2 | Gb3 => Gb0,
                Gc0 | Gc1 | Gc2 | Gc3 => Gc0,
                Gd0 | Gd1 | Gd2 | Gd3 => Gd0,
                Li | Lo => Li,
                Ri | Ro => Ri,
                _ => *self,
            },
            _ => *self,
        }
    }

    fn as_inner(&self) -> Self {
        if self.is_outer() {
            return self.for_depth(self.depth() - 1);
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
            Ga0 | Ga1 | Ga2 | Ga3 => Some(Ha),
            Gb0 | Gb1 | Gb2 | Gb3 => Some(Hb),
            Gc0 | Gc1 | Gc2 | Gc3 => Some(Hc),
            Gd0 | Gd1 | Gd2 | Gd3 => Some(Hd),
            _ => None,
        }
    }

    fn get_nearest_inner_hall(&self) -> Option<Self> {
        match self {
            Lo | Li => Some(Ha),
            Ro | Ri => Some(Hd),
            _ => self.get_inner_hall(),
        }
    }

    /// #############
    /// #...........#
    /// ###A#B#C#D###
    ///   #A#B#C#D#
    ///   #########
    fn path_to(&self, other: &Self) -> Option<Vec<Self>> {
        //println!(".. path_to({:?}, {:?})", self, other);
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
            let maybe_inner_hall = self.get_nearest_inner_hall();
            return maybe_inner_hall.and_then(|inner_hall| {
                inner_hall
                    .path_to(other)
                    .map(|to_inner_hall| vec![vec![inner_hall], to_inner_hall].concat())
            });
        } else if other.is_inner() {
            let maybe_inner_hall = &other.get_nearest_inner_hall();
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
            Ga0, Ga1, Ga2, Ga3, Gb0, Gb1, Gb2, Gb3, Gc0, Gc1, Gc2, Gc3, Gd0, Gd1, Gd2, Gd3, Lo, Li,
            Ha, Hab, Hb, Hbc, Hc, Hcd, Hd, Ri, Ro,
        ]
    }

    fn all_stops() -> Vec<Pos> {
        Self::all()
            .iter()
            .filter(|pos| !pos.is_no_loiter())
            .cloned()
            .collect()
    }

    fn all_rooms() -> Vec<Pos> {
        Self::all()
            .iter()
            .filter(|pos| pos.is_room())
            .cloned()
            .collect()
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Room {
    half: bool,
    dest: Pod,
    pods: [Option<Pod>; 4],
}

impl Room {
    fn new(half: bool, dest: Pod, pods: [Option<Pod>; 4]) -> Self {
        Self {
            half: half,
            dest: dest,
            pods: pods,
        }
    }

    fn inner_pos(&self) -> Pos {
        match self.dest {
            A => Ga0,
            B => Gb0,
            C => Gc0,
            D => Gd0,
        }
    }

    fn pos_at_depth(&self, depth: usize) -> Pos {
        self.inner_pos().for_depth(depth)
    }

    fn take_depth(&self) -> usize {
        if self.half {
            2
        } else {
            4
        }
    }

    fn is_complete(&self) -> bool {
        self.pods.iter().take(self.take_depth()).fold(true, |a, v| {
            a && v.map(|pod| pod == self.dest).unwrap_or(false)
        })
    }

    fn non_dest_pods(&self) -> Vec<(Pos, Pod)> {
        self.to_places()
            .iter()
            .filter(|(_, pod)| *pod != self.dest)
            .cloned()
            .collect()
    }

    fn obscurity(&self, pod: &Pod) -> usize {
        let mut obscurity = 0;
        let mut obscuring: Vec<usize> = Vec::new();
        for maybe_pod in self.pods {
            if let Some(opod) = maybe_pod {
                if *pod == opod {
                    let add_obscurity = obscuring.iter().fold(0, |a, v| a + v);
                    obscurity = obscurity + add_obscurity;
                    obscuring = Vec::new();
                } else {
                    obscuring.push(opod.energy());
                }
            }
        }
        obscurity
    }

    fn deepest_dest(&self, max_depth: usize) -> Option<(Pos, Pod)> {
        self.pods
            .iter()
            .enumerate()
            .take(std::cmp::min(self.take_depth(), max_depth))
            .rev()
            .filter_map(|(i, cell)| {
                cell.filter(|pod| *pod == self.dest)
                    .map(|pod| (self.pos_at_depth(i), pod))
            })
            .nth(0)
    }

    fn empty_depth(&self) -> Option<usize> {
        self.pods
            .iter()
            .enumerate()
            .take(self.take_depth())
            .rev()
            .filter_map(|(i, cell)| if cell.is_none() { Some(i) } else { None })
            .nth(0)
    }

    fn next_evicting(&self) -> Option<(Pos, Pod)> {
        if !self.non_dest_pods().is_empty() {
            self.pods
                .iter()
                .enumerate()
                .filter_map(|(i, mpod)| mpod.map(|pod| (self.pos_at_depth(i), pod)))
                .nth(0)
        } else {
            self.empty_depth().and_then(|ed| self.deepest_dest(ed))
        }
    }

    fn next_accepting(&self) -> Option<Pos> {
        if self.non_dest_pods().is_empty() {
            self.empty_depth().map(|d| self.pos_at_depth(d))
        } else {
            None
        }
    }

    fn to_places(&self) -> Vec<(Pos, Pod)> {
        self.pods
            .iter()
            .enumerate()
            .take(self.take_depth())
            .filter_map(|(i, cell)| cell.map(|pod| (self.pos_at_depth(i), pod)))
            .collect()
    }
}

use std::ops::Index;
impl Index<usize> for Room {
    type Output = Option<Pod>;

    fn index(&self, depth: usize) -> &Self::Output {
        &self.pods[depth]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    half: bool,
    ga: Room,
    gb: Room,
    gc: Room,
    gd: Room,
    hab: Option<Pod>,
    hbc: Option<Pod>,
    hcd: Option<Pod>,
    li: Option<Pod>,
    lo: Option<Pod>,
    ri: Option<Pod>,
    ro: Option<Pod>,
}

impl State {
    fn new(half: bool, places: Vec<(Pos, Pod)>) -> Self {
        let mut place_map: HashMap<Pos, Pod> = HashMap::from_iter(places.into_iter());
        Self {
            half: half,
            ga: Room::new(
                half,
                A,
                [
                    place_map.remove(&Ga0),
                    place_map.remove(&Ga1),
                    place_map.remove(&Ga2),
                    place_map.remove(&Ga3),
                ],
            ),
            gb: Room::new(
                half,
                B,
                [
                    place_map.remove(&Gb0),
                    place_map.remove(&Gb1),
                    place_map.remove(&Gb2),
                    place_map.remove(&Gb3),
                ],
            ),
            gc: Room::new(
                half,
                C,
                [
                    place_map.remove(&Gc0),
                    place_map.remove(&Gc1),
                    place_map.remove(&Gc2),
                    place_map.remove(&Gc3),
                ],
            ),
            gd: Room::new(
                half,
                D,
                [
                    place_map.remove(&Gd0),
                    place_map.remove(&Gd1),
                    place_map.remove(&Gd2),
                    place_map.remove(&Gd3),
                ],
            ),
            hab: place_map.remove(&Hab),
            hbc: place_map.remove(&Hbc),
            hcd: place_map.remove(&Hcd),
            li: place_map.remove(&Li),
            lo: place_map.remove(&Lo),
            ri: place_map.remove(&Ri),
            ro: place_map.remove(&Ro),
        }
    }

    fn to_places(&self) -> Vec<(Pos, Pod)> {
        vec![
            self.ga.to_places(),
            self.gb.to_places(),
            self.gc.to_places(),
            self.gd.to_places(),
            [
                self.hab.map(|pod| (Hab, pod)),
                self.hbc.map(|pod| (Hbc, pod)),
                self.hcd.map(|pod| (Hcd, pod)),
                self.li.map(|pod| (Li, pod)),
                self.lo.map(|pod| (Lo, pod)),
                self.ri.map(|pod| (Ri, pod)),
                self.ro.map(|pod| (Ro, pod)),
            ]
            .iter()
            .filter_map(|pair| *pair)
            .collect::<Vec<_>>(),
        ]
        .concat()
    }

    fn is_valid(&self) -> bool {
        let places = self.to_places();
        let count = if self.half { 2 } else { 4 };
        for ipod in [A, B, C, D] {
            if count
                != places
                    .iter()
                    .fold(0, |a, (_, pod)| if *pod == ipod { a + 1 } else { a })
            {
                return false;
            }
        }
        true
    }

    fn is_complete(&self) -> bool {
        self.ga.is_complete()
            && self.gb.is_complete()
            && self.gc.is_complete()
            && self.gd.is_complete()
    }

    fn at_pos(&self, pos: &Pos, pod: &Pod) -> bool {
        self.get_pos(pos).map(|ipod| ipod == *pod).unwrap_or(false)
    }

    fn get_pos(&self, pos: &Pos) -> Option<Pod> {
        match pos {
            Ga0 | Ga1 | Ga2 | Ga3 => self.ga[pos.depth()],
            Gb0 | Gb1 | Gb2 | Gb3 => self.gb[pos.depth()],
            Gc0 | Gc1 | Gc2 | Gc3 => self.gc[pos.depth()],
            Gd0 | Gd1 | Gd2 | Gd3 => self.gd[pos.depth()],
            Hab => self.hab,
            Hbc => self.hbc,
            Hcd => self.hcd,
            Li => self.li,
            Lo => self.lo,
            Ri => self.ri,
            Ro => self.ro,
            _ => None,
        }
    }

    fn at_rooms(&self) -> Vec<(Pos, Option<Pod>)> {
        Pos::all_rooms()
            .iter()
            .map(|pos| (*pos, self.get_pos(pos)))
            .collect()
    }

    fn rooms_accepting(&self) -> Vec<Pos> {
        vec![
            self.ga.next_accepting(),
            self.gb.next_accepting(),
            self.gc.next_accepting(),
            self.gd.next_accepting(),
        ]
        .iter()
        .filter_map(|e| *e)
        .collect()
    }

    fn are_empty(&self, poss: &[Pos]) -> bool {
        poss.iter().fold(!poss.is_empty(), |acc, pos| {
            acc && self.get_pos(pos).is_none()
        })
    }

    fn halls_accepting(&self, room: &Pos) -> Vec<Pos> {
        match room.get_inner_hall() {
            Some(inner_hall) => [Lo, Li, Hab, Hbc, Hcd, Ri, Ro]
                .iter()
                .filter(|hall| self.are_empty(&inner_hall.path_to(hall).unwrap()))
                .cloned()
                .collect(),
            _ => vec![],
        }
    }

    fn room_for<'s>(&'s self, pod: &Pod) -> &'s Room {
        match pod {
            A => &self.ga,
            B => &self.gb,
            C => &self.gc,
            D => &self.gd,
        }
    }

    fn obscurity(&self, pod: &Pod) -> usize {
        self.ga.obscurity(pod)
            + self.gb.obscurity(pod)
            + self.gc.obscurity(pod)
            + self.gd.obscurity(pod)
    }

    fn halls_evicting(&self, room: &Pos) -> Vec<(Pos, Pod)> {
        match room.get_inner_hall() {
            Some(inner_hall) => [Lo, Li, Hab, Hbc, Hcd, Ri, Ro]
                .iter()
                .filter_map(|hall| {
                    if let Some(pod) = self.get_pos(hall) {
                        if pod.is_dest(room) && self.are_empty(&hall.path_to(&inner_hall).unwrap()) {
                            Some((*hall, pod))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect(),
            None => vec![],
        }
    }

    fn rooms_evicting(&self) -> Vec<(Pos, Pod)> {
        let mut evictable: Vec<(Pos, Pod)> = vec![
            self.ga.next_evicting(),
            self.gb.next_evicting(),
            self.gc.next_evicting(),
            self.gd.next_evicting(),
        ]
        .iter()
        .filter_map(|e| *e)
        .collect();
        let take = if self.half { 2 } else { 4 };
        evictable.sort_by_key(|(pos, pod)| {
            (
                (take - pos.depth()) * self.obscurity(&pos.goal_for_pod().unwrap()),
                pod.energy(),
            )
        });
        evictable
    }

    fn nexts(&self) -> Vec<(Self, usize)> {
        if self.is_complete() {
            vec![]
        } else {
            let from_rooms_accepting: Vec<(Self, usize)> = self
                .rooms_accepting()
                .iter()
                .flat_map(|aroom| -> Vec<(Self, usize)> {
                    vec![
                        self.rooms_evicting()
                            .iter()
                            .filter_map(|(eroom, epod)| -> Option<(Self, usize)> {
                                if epod.is_dest(aroom) {
                                    self.after_move(eroom, aroom)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>(),
                        self.halls_evicting(aroom)
                            .iter()
                            .filter_map(|(ehall, epod)| -> Option<(Self, usize)> {
                                if epod.is_dest(aroom) {
                                    self.after_move(ehall, aroom)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>(),
                    ]
                    .concat()
                })
                .collect();
            let from_rooms_evicting: Vec<(Self, usize)> = self
                .rooms_evicting()
                .iter()
                .flat_map(|(eroom, epod)| -> Vec<(Self, usize)> {
                    self.halls_accepting(eroom)
                        .iter()
                        .filter_map(|ahall| self.after_move(eroom, ahall))
                        .collect()
                })
                .collect();
            vec![from_rooms_accepting, from_rooms_evicting].concat()
        }
    }

    fn after_moves(&self, moves: &[(Pos, Pos)]) -> Option<(Self, usize)> {
        moves.iter().fold(Some((*self, 0)), |acc, (from, to)| {
            acc.and_then(|(acc_state, acc_energy)| {
                acc_state
                    .after_move(&from, &to)
                    .map(|(new_state, new_energy)| (new_state, acc_energy + new_energy))
            })
        })
    }

    fn after_move(&self, from: &Pos, to: &Pos) -> Option<(Self, usize)> {
        if from == to || to.is_no_loiter() {
            None
        } else if let (Some(pod), Some(path)) = (self.get_pos(from), from.path_to(to)) {
            if self.are_empty(&path) {
                let energy = pod.energy() * path.len();
                let mut places: HashMap<Pos, Pod> =
                    HashMap::from_iter(self.to_places().into_iter());
                places.remove(from);
                places.insert(*to, pod);
                Some((Self::new(self.half, places.drain().collect()), energy))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn pos_str(&self, pos: Pos) -> String {
        match self.get_pos(&pos) {
            Some(pod) => format!("{:?}", pod),
            None => ".".to_owned(),
        }
    }
}

use std::cell::Cell;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Burrow {
    energy: usize,
    state: State,
}

impl Burrow {
    fn new_from_state(state: State) -> Self {
        Self {
            energy: 0,
            state: state,
        }
    }

    fn new(half: bool, inner: (Pod, Pod, Pod, Pod), outer: (Pod, Pod, Pod, Pod)) -> Self {
        let (ga, gb, gc, gd) = inner;
        let (goa, gob, goc, god) = outer;

        let rooms = if half {
            vec![
                (Ga0, ga),
                (Gb0, gb),
                (Gc0, gc),
                (Gd0, gd),
                (Ga1, goa),
                (Gb1, gob),
                (Gc1, goc),
                (Gd1, god),
            ]
        } else {
            vec![
                (Ga0, ga),
                (Gb0, gb),
                (Gc0, gc),
                (Gd0, gd),
                (Ga1, D),
                (Gb1, C),
                (Gc1, B),
                (Gd1, A),
                (Ga2, D),
                (Gb2, B),
                (Gc2, A),
                (Gd2, C),
                (Ga3, goa),
                (Gb3, gob),
                (Gc3, goc),
                (Gd3, god),
            ]
        };

        Self::new_from_state(State::new(half, rooms))
    }

    fn pod_at(&self, pos: Pos) -> Option<Pod> {
        self.state.get_pos(&pos)
    }

    fn pods(&self) -> Vec<(Pod, Pos)> {
        self.state
            .to_places()
            .iter()
            .map(|(pos, pod)| (*pod, *pos))
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
        self.state.get_pos(&pos1).zip(self.state.get_pos(&pos2))
    }

    fn find_solutions(&self, states: &mut HashMap<State, usize>, min_energy: &mut Cell<usize>) {
        let original_min_energy = min_energy.get();
        let state_min_energy = *states.get(&self.state).unwrap_or(&original_min_energy);
        if self.state.ga.pods.is_empty()
            || self.state.gb.pods.is_empty()
            || self.state.gc.pods.is_empty()
            || self.state.gd.pods.is_empty()
        {
            println!("<inter>:\n{}", self.to_string());
        }
        if self.energy >= state_min_energy {
            //println!("returning {} >= {}", self.energy, state_min_energy);
            return;
        } else if self.is_solution() {
            println!("solution!:\n{}", self.to_string());
            if self.energy < original_min_energy {
                min_energy.set(self.energy);
            }
            //
            return;
        } else {
            states.insert(self.state, self.energy);
            let next_moves = self.state.nexts();
            for next_move in &next_moves {
                let next_bur = self.transition(*next_move);
                if min_energy.get() < original_min_energy {
                    println!("{}", next_bur.to_string());
                    println!("min_egy: {}", min_energy.get());
                }
                next_bur.find_solutions(states, min_energy);
                // if next_bur.is_dead_end() {
                //     println!("dead end:\n{}", next_bur.to_string());
                //     continue;
                // }
            }
        }
    }

    fn transition(&self, new_state: (State, usize)) -> Self {
        let (state, energy) = new_state;
        Self {
            energy: self.energy + energy,
            state: state,
        }
    }

    fn is_solution(&self) -> bool {
        self.state.is_complete()
    }
}

#[test]
fn day23part1() {
    let bur = read(true);
    let mut min_energy = Cell::new(usize::MAX);
    let mut states: HashMap<State, usize> = HashMap::new();
    bur.find_solutions(&mut states, &mut min_energy);
    assert_eq!(16489, min_energy.get(), "expect min energy");
}

#[test]
fn day23part2() {
    let bur = read(false);
    let mut min_energy = Cell::new(usize::MAX);
    let mut states: HashMap<State, usize> = HashMap::new();
    bur.find_solutions(&mut states, &mut min_energy);
    assert_eq!(43413, min_energy.get(), "expect min energy");
}

/// #############
/// #...........#
/// ###A#B#C#D###
///   #A#B#C#D#
///   #########
impl ToString for Burrow {
    fn to_string(&self) -> String {
        let mut out: String = self.state.to_string();
        out = out + format!("energy:  {}\n", self.energy).as_str();
        //out = out + format!("history: {}\n", self.history.len()).as_str();
        out
    }
}

/// #############
/// #...........#
/// ###A#B#C#D###
///   #A#B#C#D#
///   #########
impl ToString for State {
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
                self.pos_str(Ga0),
                self.pos_str(Gb0),
                self.pos_str(Gc0),
                self.pos_str(Gd0),
            )
            .as_str();

        out = out
            + format!(
                "  #{}#{}#{}#{}#  \n",
                self.pos_str(Ga1),
                self.pos_str(Gb1),
                self.pos_str(Gc1),
                self.pos_str(Gd1),
            )
            .as_str();

        if !self.half {
            out = out
                + format!(
                    "  #{}#{}#{}#{}#  \n",
                    self.pos_str(Ga2),
                    self.pos_str(Gb2),
                    self.pos_str(Gc2),
                    self.pos_str(Gd2),
                )
                .as_str();

            out = out
                + format!(
                    "  #{}#{}#{}#{}#  \n",
                    self.pos_str(Ga3),
                    self.pos_str(Gb3),
                    self.pos_str(Gc3),
                    self.pos_str(Gd3),
                )
                .as_str();
        }

        out = out + "  #########  \n";
        out
    }
}

#[test]
fn day23_test_out() {
    let bur = read_test(true);
    println!("burrow: \n{}", bur.to_string());
}

#[test]
fn day23_test_is_solution() {
    let bur = read_test(true);
    assert!(!bur.is_solution());

    let bur_solved = Burrow::new(true, (A, B, C, D), (A, B, C, D));
    assert!(bur_solved.is_solution());
}

#[test]
fn day23_test_list_moves() {
    let bur = read_test(true);
    let paths = PosPaths::new();
    println!("paths: {:?}", paths);
}

#[test]
fn day23_test_state_nexts() {
    let bur = read_test(true);
    for next in bur.state.nexts() {
        println!("{}", bur.transition(next).to_string());
    }
}

#[test]
fn day23_test_rooms_accepting() {
    let bur_complete = Burrow::new(true, (A, B, C, D), (A, B, C, D));
    let ras_complete = bur_complete.state.rooms_accepting();
    assert!(bur_complete.is_solution());
    assert_eq!(0, ras_complete.len());

    let bur_accept_d = Burrow::new_from_state(
        bur_complete
            .state
            .after_move(&Gd0, &Li)
            .and_then(|(state, _)| state.after_move(&Gd1, &Ri))
            .unwrap()
            .0,
    );
    assert!(!bur_accept_d.is_solution());

    let ras_accept_d = bur_accept_d.state.rooms_accepting();
    assert_eq!(vec![Gd1], ras_accept_d);
}

#[test]
fn day23_test_rooms_evicting() {
    let bur = Burrow::new(true, (A, B, C, D), (A, B, C, D));
    let ras = bur.state.rooms_evicting();
    assert!(bur.is_solution());
    assert_eq!(0, ras.len());

    let bur_start = Burrow::new(true, (D, D, C, A), (B, C, B, A));
    let res_start = bur_start.state.rooms_evicting();
    assert!(!bur_start.is_solution());
    assert_eq!(vec![(Gd0, A), (Ga0, D), (Gc0, C), (Gb0, D)], res_start);
}

#[test]
fn day23_test_full_rooms_evicting() {
    let bur_complete = Burrow::new_from_state(State::new(
        false,
        Pos::all_rooms()
            .iter()
            .map(|pos| (*pos, pos.goal_for_pod().unwrap()))
            .collect(),
    ));

    assert!(bur_complete.is_solution());

    // #############
    // #...........#
    // ###D#D#C#A###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #B#C#B#A#
    //   #########
    let bur_start = Burrow::new(false, (D, D, C, A), (B, C, B, A));

    let res_start = bur_start.state.rooms_evicting();
    assert!(!bur_start.is_solution());
    assert_eq!(vec![(Gd0, A), (Ga0, D), (Gc0, C), (Gb0, D)], res_start);

    let (state1, energy1) = bur_start
        .state
        .after_moves(&vec![(Gd0, Lo), (Gd1, Ro)])
        .unwrap();
    assert_eq!(
        vec![(Gd2, C), (Ga0, D), (Gc0, C), (Gb0, D)],
        state1.rooms_evicting()
    );
    assert!(state1.rooms_accepting().is_empty());
    assert_eq!(9 + 4, energy1);

    let (state2, energy2) = state1.after_moves(&vec![(Gd2, Li), (Gd3, Ri)]).unwrap();
    assert_eq!(vec![(Ga0, D), (Gc0, C), (Gb0, D)], state2.rooms_evicting());

    assert_eq!(1000 + 5, energy2);
    assert_eq!(vec![Gd3], state2.rooms_accepting());
    for gd in [Gd0, Gd1, Gd2, Gd3] {
        assert!(state2.halls_evicting(&gd).is_empty());
    }

    let (state3, energy3) = state2
        .after_moves(&vec![(Ga0, Gd3), (Ga1, Gd2), (Ga2, Gd1)])
        .unwrap();
    assert_eq!(vec![Gd0], state3.rooms_accepting());
    assert_eq!(vec![Hab, Hbc, Hcd], state3.halls_accepting(&Ga3));
    assert_eq!(vec![(Ri, A)], state3.halls_evicting(&Ga2));

    // let (state2, energy3) = state2.after_moves(&vec![(Gd2, Li), (Gd3, Ri)]).unwrap();
    // assert_eq!(
    //     vec![(Gd2, C), (Ga0, D), (Gc0, C), (Gb0, D)],
    //     state1.rooms_evicting()
    // );

    // assert_eq!(vec![Gd3], state2.rooms_accepting());
    // assert_eq!(1000 + 5, energy2);
}

#[test]
fn day23pre_part1() {
    let bur = read_test(true);
    let mut min_energy = Cell::new(usize::MAX);
    let mut states: HashMap<State, usize> = HashMap::new();
    bur.find_solutions(&mut states, &mut min_energy);
    assert_eq!(12521, min_energy.get(), "expect min energy");
}

#[test]
fn day23manual_part1() {
    let bur = read(true);
    /// #############
    /// #...........#
    /// ###D#D#A#A###
    ///   #C#C#B#B#
    ///   #########
    // [A, B, C, D]
    let ad_wait = Li;
    let ac_wait = Hcd;
    let bd_wait = Ri;
    let bc_wait = Hab;
    let moves: Vec<(Pos, Pos)> = vec![
        // Ad to right
        (Gd0, ad_wait),
        // one B just past Aroom
        (Gd1, bd_wait),
        // move Ds
        (Gb0, Gd1),
        (Ga0, Gd0),
        // move A
        (Gc0, ac_wait),
        // move B to wait
        (Gc1, bc_wait),
        // clear C from Broom
        (Gb1, Gc1),
        (bc_wait, Gb1),
        // clear C from Aroom
        (Ga1, Gc0),
        (ac_wait, Ga1),
        (ad_wait, Ga0),
        // move outer B into Broom
        (bd_wait, Gb0),
    ];
    println!("{}\n", bur.state.to_string());
    let (fin, energy) = moves
        .iter()
        .fold((bur.state, 0), |(state, etotal), (from, to)| {
            let (next_state, enext) = state.after_move(from, to).unwrap();
            println!("{}\n", next_state.to_string());
            (next_state, etotal + enext)
        });
    assert!(fin.is_complete(), "expect complete state: {:?}", fin);
    // let mut min_energy = Cell::new(usize::MAX);
    // let mut solutions: Vec<Burrow> = bur.find_solutions(&vec![], &mut min_energy);
    // solutions.sort_by_key(|b| b.energy);
    assert_ne!(20685, energy, "too high");
    assert_ne!(16535, energy, "too high");
    assert_ne!(16551, energy, "too high");
    assert_ne!(16529, energy, "too high");
    assert_eq!(16489, energy, "expect min energy");
}

/// insert these two new rows between existing rooms:
///
/// #D#C#B#A#
/// #D#B#A#C#
///
/// Starting positions:
/// #############
/// #...........#
/// ###D#D#A#A###
///   #D#C#B#A#
///   #D#B#A#C#
///   #C#C#B#B#
///   #########
///
#[test]
fn day23manual_part2() {
    let bur = read(false);
    let ad0_wait = Lo;
    let ad1_wait = Li;
    let cd2_wait1 = Ro;
    let cd2_wait2 = Hcd;
    let bd3_wait = Ri;
    let bc1_wait = Li;
    let bc3_wait = Hab;
    let moves: Vec<(Pos, Pos)> = vec![
        (Gd0, ad0_wait),
        (Gd1, ad1_wait),
        (Gd2, cd2_wait1),
        (Gd3, bd3_wait),
        //(cd2_wait1, cd2_wait2),
        (Gb0, Gd3),
        (Ga0, Gd2),
        (Ga1, Gd1),
        (Ga2, Gd0),
        (Ga3, cd2_wait2), // clear A
        (Gc0, Ga3),
        (ad1_wait, Ga2),
        (ad0_wait, Ga1),
        (Gc1, bc1_wait),
        (Gc2, Ga0),
        (Gc3, bc3_wait),
        (Gb1, Gc3),
        (cd2_wait2, Gc2),
        (Gb2, cd2_wait2),
        (Gb3, Gc1),
        (cd2_wait2, Gb3),
        (bc3_wait, Gb2),
        (bc1_wait, Gb1),
        (bd3_wait, Gb0),
        (cd2_wait1, Gc0),
        // (Gb2, Hab),
        // (Gb3, Gc0),
        // (Hab, Gb3),
        // (Li, Gb2),
        // (Ro, Gb1),
        // (Lo, Gb0),
    ];
    println!("{}\n", bur.state.to_string());
    let (fin, energy) = moves
        .iter()
        .fold((bur.state, 0), |(state, etotal), (from, to)| {
            let (next_state, enext) = state.after_move(from, to).unwrap();
            println!("{}\n", next_state.to_string());
            (next_state, etotal + enext)
        });
    assert!(fin.is_complete(), "expect complete state: {:?}", fin);
    // let mut min_energy = Cell::new(usize::MAX);
    // let mut solutions: Vec<Burrow> = bur.find_solutions(&vec![], &mut min_energy);
    // solutions.sort_by_key(|b| b.energy);
    // assert_ne!(20685, energy, "too high");
    // assert_ne!(16535, energy, "too high");
    // assert_ne!(16551, energy, "too high");
    assert_ne!(43453, energy, "too high");
    assert_eq!(43413, energy, "expect min energy");
}
