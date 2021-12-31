mod common;
use ndarray::arr2;
use ndarray::Array2;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;

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
    return parse_scanners(&common::read_test_input("data/day-19/input.txt"));
}

fn read_test() -> Vec<Scanner> {
    return parse_scanners(&common::read_test_input("data/day-19/input_test.txt"));
}

fn parse_scanners(lines: &Vec<String>) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();
    let mut points: Vec<Point> = Vec::new();
    for line in lines.iter().map(|line| line.trim()) {
        if line.starts_with("---") {
            continue;
        }
        if line.is_empty() {
            if !points.is_empty() {
                scanners.push(Scanner::new(
                    scanners.len(),
                    points.iter().map(|p| Ping::from_point(p)).collect(),
                ));
                points = Vec::new();
            }
            continue;
        }
        if let Some((xs, rest)) = line.split_once(",") {
            if let Some((ys, zs)) = rest.split_once(",") {
                if let (Some(x), Some(y), Some(z)) =
                    (xs.parse().ok(), ys.parse().ok(), zs.parse().ok())
                {
                    points.push((x, y, z));
                }
            }
        }
    }
    if !points.is_empty() {
        scanners.push(Scanner::new(
            scanners.len(),
            points.iter().map(|p| Ping::from_point(p)).collect(),
        ));
    }
    return scanners;
}

/// a point defined by relative distance from an origin in a coordinate system of
/// unknown orientation
type Point = (isize, isize, isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum RotVal {
    Zero = 0,
    Cw1,
    Cw2,
    Cw3,
}
use RotVal::*;
impl RotVal {
    fn next(&self) -> RotVal {
        match self {
            Zero => Cw1,
            Cw1 => Cw2,
            Cw2 => Cw3,
            Cw3 => Zero,
        }
    }

    fn invert(&self) -> RotVal {
        match self {
            Zero | Cw2 => *self,
            Cw1 => Cw3,
            Cw3 => Cw1,
        }
    }

    fn prev(&self) -> RotVal {
        match self {
            Zero => Cw3,
            Cw1 => Zero,
            Cw2 => Cw1,
            Cw3 => Cw2,
        }
    }
}

impl Add for RotVal {
    type Output = RotVal;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Zero, _) => rhs,
            (_, Zero) => self,
            (Cw1, Cw3) | (Cw3, Cw1) | (Cw2, Cw2) => Zero,
            (Cw2, Cw3) | (Cw3, Cw2) => Cw1,
            (Cw1, Cw1) | (Cw3, Cw3) => Cw2,
            (Cw1, Cw2) | (Cw2, Cw1) => Cw3,
            _ => {
                println!("*** {:?}", (self, rhs));
                unreachable!()
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dim {
    X,
    NegX,
    Y,
    NegY,
    Z,
    NegZ,
}

use Dim::*;
impl Dim {
    fn abs(&self) -> Self {
        match self {
            X | NegX => X,
            Y | NegY => Y,
            Z | NegZ => Z,
        }
    }

    fn arr_idx(&self) -> usize {
        match self {
            X | NegX => 0,
            Y | NegY => 1,
            Z | NegZ => 2,
        }
    }

    fn as_up_get_right_for(&self, fwd: &Dim) -> Dim {
        assert_ne!(
            self.abs(),
            fwd.abs(),
            "self and fwd must be different abs dims"
        );
        (match self.abs() {
            Z => match fwd.abs() {
                Y => X.invert_if_neg(fwd),
                X => NegY.invert_if_neg(fwd),
                _ => unreachable!(),
            },
            Y => match fwd.abs() {
                X => Z.invert_if_neg(fwd),
                Z => NegX.invert_if_neg(fwd),
                _ => unreachable!(),
            },
            X => match fwd.abs() {
                Z => Y.invert_if_neg(fwd),
                Y => NegZ.invert_if_neg(fwd),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
        .invert_if_neg(self)
    }

    fn invert(&self) -> Self {
        match self {
            X => NegX,
            Y => NegY,
            Z => NegZ,
            NegX => X,
            NegY => Y,
            NegZ => Z,
        }
    }

    fn same_sign(&self, other: &Self) -> bool {
        self.is_neg() == other.is_neg()
    }

    fn to_neg(&self) -> Dim {
        match self {
            X | NegX => NegX,
            Y | NegY => NegY,
            Z | NegZ => NegZ,
        }
    }

    fn is_neg(&self) -> bool {
        *self != self.abs()
    }

    fn invert_if_pos(&self, other: &Self) -> Self {
        if !other.is_neg() {
            self.invert()
        } else {
            *self
        }
    }

    fn invert_if_neg(&self, other: &Self) -> Self {
        if other.is_neg() {
            self.invert()
        } else {
            *self
        }
    }

    fn abs_remaining(abs1: &Dim, abs2: &Dim) -> Dim {
        [X, Y, Z]
            .iter()
            .cloned()
            .filter(|dim| *dim != abs1.abs() && *dim != abs2.abs())
            .nth(0)
            .unwrap()
    }

    fn neg_remaining(abs1: &Dim, abs2: &Dim) -> Dim {
        Self::abs_remaining(abs1, abs2).to_neg()
    }
}

#[test]
fn day19_test_corner_rotate() {
    let all_pos = Corner::new(X, Y, Z);
    assert_eq!(
        all_pos,
        all_pos.rotate(&Rotation::ZERO),
        "expect same corner with no rotation"
    );
    let rot_1x = all_pos.rotate(&Rotation::new(Cw1, Zero, Zero));
    assert_eq!(Corner::new(X, Y, NegZ), rot_1x);
    let rot_2x = all_pos.rotate(&Rotation::new(Cw2, Zero, Zero));
    assert_eq!(Corner::new(X, NegY, NegZ), rot_2x);
    let rot_3x = all_pos.rotate(&Rotation::new(Cw3, Zero, Zero));
    assert_eq!(Corner::new(X, NegY, Z), rot_3x);

    let rot_1y = all_pos.rotate(&Rotation::new(Zero, Cw1, Zero));
    assert_eq!(Corner::new(NegX, Y, Z), rot_1y);
    let rot_2y = all_pos.rotate(&Rotation::new(Zero, Cw2, Zero));
    assert_eq!(Corner::new(NegX, Y, NegZ), rot_2y);
    let rot_3y = all_pos.rotate(&Rotation::new(Zero, Cw3, Zero));
    assert_eq!(Corner::new(X, Y, NegZ), rot_3y);

    let rot_1z = all_pos.rotate(&Rotation::new(Zero, Zero, Cw1));
    assert_eq!(Corner::new(X, NegY, Z), rot_1z);
    let rot_2z = all_pos.rotate(&Rotation::new(Zero, Zero, Cw2));
    assert_eq!(Corner::new(NegX, NegY, Z), rot_2z);
    let rot_3z = all_pos.rotate(&Rotation::new(Zero, Zero, Cw3));
    assert_eq!(Corner::new(NegX, Y, Z), rot_3z);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Corner {
    x: Dim,
    y: Dim,
    z: Dim,
}

impl Corner {
    fn new(x: Dim, y: Dim, z: Dim) -> Self {
        assert!(x == X || x == NegX, "x must be either X or NegX");
        assert!(y == Y || y == NegY, "x must be either X or NegX");
        assert!(z == Z || z == NegZ, "x must be either X or NegX");
        Self { x: x, y: y, z: z }
    }

    fn of(dim1: &Dim, dim2: &Dim, dim3: &Dim) -> Option<Self> {
        let maybe_x = [dim1, dim2, dim3]
            .iter()
            .cloned()
            .filter(|dim| dim.abs() == X)
            .nth(0);
        let maybe_y = [dim1, dim2, dim3]
            .iter()
            .cloned()
            .filter(|dim| dim.abs() == Y)
            .nth(0);
        let maybe_z = [dim1, dim2, dim3]
            .iter()
            .cloned()
            .filter(|dim| dim.abs() == Z)
            .nth(0);
        if let (Some(x), Some(y), Some(z)) = (maybe_x, maybe_y, maybe_z) {
            Some(Corner::new(*x, *y, *z))
        } else {
            println!("Corner::of({:?}, {:?}, {:?})", dim1, dim2, dim3);
            None
        }
    }

    fn invert(&self) -> Self {
        Self::new(self.x.invert(), self.y.invert(), self.z.invert())
    }

    fn to_point(&self, origin: Option<Point>) -> Point {
        let (ox, oy, oz) = origin.unwrap_or((0, 0, 0));
        (
            ox + if self.x.is_neg() { SCAN_MIN } else { SCAN_MAX },
            oy + if self.y.is_neg() { SCAN_MIN } else { SCAN_MAX },
            oz + if self.z.is_neg() { SCAN_MIN } else { SCAN_MAX },
        )
    }

    fn rotate(&self, rotation: &Rotation) -> Self {
        let (x, y, z) = transform_point(&self.to_point(None), &rotation.to_matrices());
        Self::new(
            if x < 0 { NegX } else { X },
            if y < 0 { NegY } else { Y },
            if z < 0 { NegZ } else { Z },
        )
    }

    /// (corner, inner bound)
    fn iterate_from_corner(&self, points: &Vec<Point>) -> Vec<Point> {
        let mut boxes: Vec<Point> = Vec::new();
        let mut xs = all_xs(points);
        xs.sort();
        xs.dedup();
        if !self.x.is_neg() {
            xs.reverse();
        }

        let mut ys = all_ys(points);
        ys.sort();
        ys.dedup();
        if !self.y.is_neg() {
            ys.reverse();
        }

        let mut zs = all_zs(points);
        zs.sort();
        zs.dedup();
        if !self.z.is_neg() {
            zs.reverse();
        }
        for x in &xs {
            for y in &ys {
                for z in &zs {
                    boxes.push((*x, *y, *z));
                }
            }
        }
        boxes
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CornerPosition {
    RearLowerLeft,
    RearLowerRight,
    ForeLowerRight,
    ForeLowerLeft,
    ForeUpperLeft,
    ForeUpperRight,
    RearUpperRight,
    RearUpperLeft,
}

impl CornerPosition {
    fn all() -> Vec<CornerPosition> {
        vec![
            RearLowerLeft,
            RearLowerRight,
            ForeLowerRight,
            ForeLowerLeft,
            ForeUpperLeft,
            ForeUpperRight,
            RearUpperRight,
            RearUpperLeft,
        ]
    }
}

use CornerPosition::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Orient {
    fwd: Dim,
    up: Dim,
}

const DEFAULT_UP: Dim = Z;
const DEFAULT_FWD: Dim = Y;

impl Orient {
    fn default() -> Self {
        Self {
            fwd: DEFAULT_FWD,
            up: DEFAULT_UP,
        }
    }

    fn is_default(&self) -> bool {
        self.up == DEFAULT_UP && self.fwd == DEFAULT_FWD
    }

    fn get_corner(&self, position: &CornerPosition) -> Corner {
        match *position {
            RearLowerLeft => self.get_rear_lower_left_corner(),
            RearLowerRight => self.get_rear_lower_right_corner(),
            ForeLowerRight => self.get_fore_lower_right_corner(),
            ForeLowerLeft => self.get_fore_lower_left_corner(),
            ForeUpperLeft => self.get_fore_upper_left_corner(),
            ForeUpperRight => self.get_fore_upper_right_corner(),
            RearUpperRight => self.get_rear_upper_right_corner(),
            RearUpperLeft => self.get_rear_upper_left_corner(),
        }
    }

    fn get_corner_position(&self, corner: &Corner) -> CornerPosition {
        for position in CornerPosition::all() {
            if self.get_corner(&position) == *corner {
                return position;
            }
        }
        unreachable!()
    }

    fn get_rear_lower_left_corner(&self) -> Corner {
        let rear_dim = self.fwd.invert();
        let lower_dim = self.up.invert();
        let left_dim = self.up.as_up_get_right_for(&self.fwd).invert();
        Corner::of(&rear_dim, &lower_dim, &left_dim).unwrap()
    }

    /// rear lower right is -x -y -z for both Up:Z Fwd:X and Up:X Fwd:Y
    fn get_rear_lower_right_corner(&self) -> Corner {
        let rear_dim = self.fwd.invert();
        let lower_dim = self.up.invert();
        let right_dim = self.up.as_up_get_right_for(&self.fwd);
        Corner::of(&rear_dim, &lower_dim, &right_dim).unwrap()
    }

    fn get_fore_lower_right_corner(&self) -> Corner {
        let fore_dim = self.fwd;
        let lower_dim = self.up.invert();
        let right_dim = self.up.as_up_get_right_for(&self.fwd);
        Corner::of(&fore_dim, &lower_dim, &right_dim).unwrap()
    }

    fn get_fore_lower_left_corner(&self) -> Corner {
        let fore_dim = self.fwd;
        let lower_dim = self.up.invert();
        let left_dim = self.up.as_up_get_right_for(&self.fwd).invert();
        Corner::of(&fore_dim, &lower_dim, &left_dim).unwrap()
    }

    fn get_fore_upper_left_corner(&self) -> Corner {
        let fore_dim = self.fwd;
        let upper_dim = self.up;
        let left_dim = self.up.as_up_get_right_for(&self.fwd).invert();
        Corner::of(&fore_dim, &upper_dim, &left_dim).unwrap()
    }

    fn get_fore_upper_right_corner(&self) -> Corner {
        let fore_dim = self.fwd;
        let upper_dim = self.up;
        let right_dim = self.up.as_up_get_right_for(&self.fwd);
        Corner::of(&fore_dim, &upper_dim, &right_dim).unwrap()
    }

    fn get_rear_upper_right_corner(&self) -> Corner {
        let rear_dim = self.fwd.invert();
        let upper_dim = self.up;
        let right_dim = self.up.as_up_get_right_for(&self.fwd);
        Corner::of(&rear_dim, &upper_dim, &right_dim).unwrap()
    }

    fn get_rear_upper_left_corner(&self) -> Corner {
        let rear_dim = self.fwd.invert();
        let upper_dim = self.up;
        let left_dim = self.up.as_up_get_right_for(&self.fwd).invert();
        Corner::of(&rear_dim, &upper_dim, &left_dim).unwrap()
    }

    fn new(up: Dim, fwd: Dim) -> Self {
        Self { fwd: fwd, up: up }
    }

    fn all() -> Vec<Self> {
        let mut orients: Vec<Self> = Vec::new();
        for dim_up in [Z, X, Y] {
            for dim_fwd in [Y, X, Z] {
                if dim_up == dim_fwd {
                    continue;
                }
                orients.push(Self::new(dim_up, dim_fwd));
                orients.push(Self::new(dim_up.to_neg(), dim_fwd));
                orients.push(Self::new(dim_up, dim_fwd.to_neg()));
                orients.push(Self::new(dim_up.to_neg(), dim_fwd.to_neg()));
            }
        }
        orients
    }

    fn rotation_to(&self, other: &Self) -> Rotation {
        if self != other {
            if self.fwd == other.fwd {
                let (next_rot, next_orient) = self.roll_right(None);
                return next_rot + next_orient.rotation_to(other);
            } else if self.up == other.up {
                let (next_rot, next_orient) = self.yaw_left(None);
                return next_rot + next_orient.rotation_to(other);
            } else {
                let (next_rot, next_orient) = self.pitch_up(None);
                return next_rot + next_orient.rotation_to(other);
            }
        }
        return Rotation::zero();
    }

    fn rotate(&self, rotation: &Rotation) -> Orient {
        if rotation.is_zero() {
            return *self;
        }
        if rotation.around_x != Zero {
            let (_, new_orient) = self.pitch_up(None);
            return new_orient.rotate(&rotation.new_around_x(rotation.around_x.prev()));
        }
        if rotation.around_y != Zero {
            let (_, new_orient) = self.roll_right(None);
            return new_orient.rotate(&rotation.new_around_y(rotation.around_y.prev()));
        }
        if rotation.around_z != Zero {
            let (_, new_orient) = self.yaw_left(None);
            return new_orient.rotate(&rotation.new_around_z(rotation.around_z.prev()));
        }
        unreachable!()
    }

    fn new_up_auto_invert(&self, up: Dim) -> Self {
        Self {
            up: up.invert_if_neg(&self.fwd).invert_if_neg(&self.up),
            fwd: self.fwd,
        }
    }

    fn new_fwd_auto_invert(&self, fwd: Dim) -> Self {
        Self {
            up: self.up,
            fwd: fwd.invert_if_neg(&self.fwd).invert_if_neg(&self.up),
        }
    }

    fn yaw_left(&self, last: Option<Rotation>) -> (Rotation, Self) {
        let rot_from = last.unwrap_or(Rotation::ZERO);
        let rotation = rot_from.new_around_z(rot_from.around_z.next());
        let new_orient = match (self.up.abs(), self.fwd.abs()) {
            (Z, Y) => self.new_fwd_auto_invert(NegX),
            (Z, X) => self.new_fwd_auto_invert(Y),
            (Y, X) => self.new_fwd_auto_invert(NegZ),
            (Y, Z) => self.new_fwd_auto_invert(X),
            (X, Y) => self.new_fwd_auto_invert(Z),
            (X, Z) => self.new_fwd_auto_invert(NegY),
            _ => unreachable!(),
        };
        (rotation, new_orient)
    }

    fn yaw_right(&self, last: Option<Rotation>) -> (Rotation, Self) {
        let rot_from = last.unwrap_or(Rotation::ZERO);
        let rotation = rot_from.new_around_z(rot_from.around_z.prev());
        let new_orient = match (self.up.abs(), self.fwd.abs()) {
            (Z, Y) => self.new_fwd_auto_invert(X),
            (Z, X) => self.new_fwd_auto_invert(NegY),
            (Y, X) => self.new_fwd_auto_invert(Z),
            (Y, Z) => self.new_fwd_auto_invert(NegX),
            (X, Y) => self.new_fwd_auto_invert(NegZ),
            (X, Z) => self.new_fwd_auto_invert(Y),
            _ => unreachable!(),
        };
        (rotation, new_orient)
    }

    fn pitch_up(&self, last: Option<Rotation>) -> (Rotation, Self) {
        let rot_from = last.unwrap_or(Rotation::ZERO);
        let rotation = rot_from.new_around_x(rot_from.around_z.next());
        let new_orient = Self {
            up: self.fwd.invert(),
            fwd: self.up,
        };
        (rotation, new_orient)
    }

    fn pitch_down(&self, last: Option<Rotation>) -> (Rotation, Self) {
        let rot_from = last.unwrap_or(Rotation::ZERO);
        let rotation = rot_from.new_around_x(rot_from.around_x.prev());
        let new_orient = Self {
            up: self.fwd,
            fwd: self.up.invert(),
        };
        (rotation, new_orient)
    }

    fn roll_left(&self, last: Option<Rotation>) -> (Rotation, Self) {
        let rot_from = last.unwrap_or(Rotation::ZERO);
        let rotation = rot_from.new_around_y(rot_from.around_y.prev());
        let new_orient = match (self.up.abs(), self.fwd.abs()) {
            (Z, Y) => self.new_up_auto_invert(NegX),
            (Z, X) => self.new_up_auto_invert(Y),
            (Y, X) => self.new_up_auto_invert(NegZ),
            (Y, Z) => self.new_up_auto_invert(X),
            (X, Y) => self.new_up_auto_invert(Z),
            (X, Z) => self.new_up_auto_invert(NegY),
            _ => unreachable!(),
        };
        (rotation, new_orient)
    }

    fn roll_right(&self, last: Option<Rotation>) -> (Rotation, Self) {
        let rot_from = last.unwrap_or(Rotation::ZERO);
        let rotation = rot_from.new_around_y(rot_from.around_y.next());
        let new_orient = match (self.up.abs(), self.fwd.abs()) {
            (Z, Y) => self.new_up_auto_invert(X),
            (Z, X) => self.new_up_auto_invert(NegY),
            (Y, X) => self.new_up_auto_invert(Z),
            (Y, Z) => self.new_up_auto_invert(NegX),
            (X, Y) => self.new_up_auto_invert(NegZ),
            (X, Z) => self.new_up_auto_invert(Y),
            _ => unreachable!(),
        };
        (rotation, new_orient)
    }
}

impl ToString for Orient {
    fn to_string(&self) -> String {
        return format!("Orient::new({:?}, {:?})", self.up, self.fwd);
    }
}

#[test]
fn day19_test_orient_corner_positions() {
    let looking_right = Orient::new(Z, X);
    assert_eq!(
        Corner::new(NegX, NegY, NegZ),
        looking_right.get_corner(&RearLowerRight),
        "expect correct corner"
    );
}

#[test]
fn day19_test_orient_nexts() {
    assert_eq!(
        Orient::new(X, Y),
        Orient::default().roll_right(None).1,
        "roll right"
    );
    assert_eq!(
        Rotation::new(Zero, Cw1, Zero),
        Orient::default().roll_right(None).0,
        "roll right"
    );
    assert_eq!(
        Orient::new(NegX, Y),
        Orient::default().roll_left(None).1,
        "roll left"
    );
    assert_eq!(
        Rotation::new(Zero, Cw3, Zero),
        Orient::default().roll_left(None).0,
        "roll left"
    );
}

#[test]
fn day19_test_orient() {
    let orients = Orient::all();
    for orient in &orients {
        println!("orient: {:?}, {}", orient, orients.len());
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Rotation {
    around_x: RotVal,
    around_y: RotVal,
    around_z: RotVal,
}

trait Rotations {
    const ZERO: Rotation;
}

impl Rotations for Rotation {
    const ZERO: Self = Self {
        around_x: Zero,
        around_y: Zero,
        around_z: Zero,
    };
}

impl Rotation {
    fn new(around_x: RotVal, around_y: RotVal, around_z: RotVal) -> Self {
        Self {
            around_x: around_x,
            around_y: around_y,
            around_z: around_z,
        }
    }

    fn is_zero(&self) -> bool {
        self.around_x == Zero && self.around_y == Zero && self.around_z == Zero
    }

    fn zero() -> Self {
        Self::ZERO
    }

    fn invert(&self) -> Self {
        Self {
            around_x: self.around_x.invert(),
            around_y: self.around_y.invert(),
            around_z: self.around_z.invert(),
        }
    }

    fn new_around_x(&self, around_x: RotVal) -> Self {
        Self::new(around_x, self.around_y, self.around_z)
    }

    fn new_around_y(&self, around_y: RotVal) -> Self {
        Self::new(self.around_x, around_y, self.around_z)
    }

    fn new_around_z(&self, around_z: RotVal) -> Self {
        Self::new(self.around_x, self.around_y, around_z)
    }

    fn to_matrices_pretranslate(&self, pretrans: &(Point, Point)) -> Vec<Array2<isize>> {
        let (from, to) = pretrans;
        vec![vec![t_translate(from, to)], self.to_matrices()].concat()
    }

    fn to_matrices(&self) -> Vec<Array2<isize>> {
        let mut matrices: Vec<Array2<isize>> = Vec::new();
        matrices.push(t_rotate_x(self.around_x));
        matrices.push(t_rotate_y(self.around_y));
        matrices.push(t_rotate_z(self.around_z));
        matrices
    }
}

impl ToString for Rotation {
    fn to_string(&self) -> String {
        format!(
            "Rotation::new({:?}, {:?}, {:?})",
            self.around_x, self.around_y, self.around_z
        )
    }
}

impl Add for Rotation {
    type Output = Rotation;
    fn add(self, rhs: Self) -> Self::Output {
        if rhs == Rotation::ZERO {
            return self;
        } else if self == Rotation::ZERO {
            return rhs;
        } else {
            return Self::new(
                self.around_x + rhs.around_x,
                self.around_y + rhs.around_y,
                self.around_z + rhs.around_z,
            );
        }
    }
}

#[derive(Debug, Clone)]
struct SubspaceCalc {
    corner_pos: CornerPosition,
    vert: Ping,
    pings: Vec<Ping>,
    dists_ext: Vec<isize>,
    dists_int: Vec<isize>,
}

impl SubspaceCalc {
    fn new(corner_pos: &CornerPosition, pings: &Vec<Ping>) -> Self {
        let corner = Orient::default().get_corner(corner_pos);
        let points: Vec<Point> = pings.iter().map(|ping| (*ping).into()).collect();
        let vert: Ping = (
            if corner.x.is_neg() {
                all_xs(&points).iter().fold(SCAN_MIN, |a, x| max(a, *x))
            } else {
                all_xs(&points).iter().fold(SCAN_MAX, |a, x| min(a, *x))
            },
            if corner.y.is_neg() {
                all_ys(&points).iter().fold(SCAN_MIN, |a, y| max(a, *y))
            } else {
                all_ys(&points).iter().fold(SCAN_MAX, |a, y| min(a, *y))
            },
            if corner.z.is_neg() {
                all_zs(&points).iter().fold(SCAN_MIN, |a, z| max(a, *z))
            } else {
                all_zs(&points).iter().fold(SCAN_MAX, |a, z| min(a, *z))
            },
        )
            .into();
        let mut sub = Self {
            corner_pos: *corner_pos,
            vert: vert,
            pings: pings.to_owned(),
            dists_ext: Vec::new(),
            dists_int: Vec::new(),
        };

        sub.dists_int = sub.dists_internal();
        sub.dists_ext = sub.dists_external();
        sub
    }

    fn new_subspace(&self, orient: &Orient) -> Subspace {
        Subspace {
            corner: orient.get_corner(&self.corner_pos),
            points: self.points(orient, &DEFAULT_ORIGIN),
            dists_ext: self.dists_ext.to_owned(),
            dists_int: self.dists_int.to_owned(),
            orient: *orient,
            vert: self.vert.to_point(orient, &DEFAULT_ORIGIN),
        }
    }

    fn new_subspace_key(&self, orient: &Orient) -> SubspaceKey {
        let corner = orient.get_corner(&self.corner_pos);
        SubspaceKey {
            orient: *orient,
            corner: corner,
            dists_ext: self.dists_ext.to_owned(),
        }
    }

    fn select_all(
        min: usize,
        scanner: &Scanner,
    ) -> HashMap<CornerPosition, HashMap<Vec<Ping>, Self>> {
        let all_points = scanner.points(Some(Orient::default()), Some(DEFAULT_ORIGIN));
        CornerPosition::all()
            .iter()
            .map(|corner_pos| {
                let corner = Orient::default().get_corner(corner_pos);
                let corner_subs: HashMap<Vec<Ping>, SubspaceCalc> = corner
                    .iterate_from_corner(&all_points)
                    .iter()
                    .flat_map(|vert| Self::select(12, corner_pos, &vert, scanner))
                    .map(|sub| (sub.pings.to_owned(), sub))
                    .collect();
                (*corner_pos, corner_subs)
            })
            .collect()
    }

    fn subspace_key_lookup<'m>(
        originals: &HashMap<CornerPosition, HashMap<Vec<Ping>, Self>>,
    ) -> HashMap<SubspaceKey, SubspaceCalcKey> {
        let orients = Orient::all();

        HashMap::from_iter(
            originals
                .values()
                .flat_map(|cornmap| cornmap.values())
                .flat_map(|calc| {
                    let calc_p = calc;
                    orients.iter().map(move |orient| {
                        (
                            calc_p.new_subspace_key(orient),
                            SubspaceCalcKey {
                                position: calc_p.corner_pos,
                                pings: calc_p.pings.to_owned(),
                            },
                        )
                    })
                })
                .into_iter(),
        )
    }

    fn select(
        min: usize,
        corner_pos: &CornerPosition,
        vert: &Point,
        scanner: &Scanner,
    ) -> Option<Self> {
        let corner = Orient::default().get_corner(corner_pos);
        let cpoint = corner.to_point(None);
        let mut subpings: Vec<Ping> = scanner
            .pings
            .iter()
            .filter(|test| filter_box(&cpoint, vert, &(**test).into()))
            .cloned()
            .collect();
        if subpings.len() >= min {
            subpings.sort();
            Some(SubspaceCalc::new(corner_pos, &subpings))
        } else {
            None
        }
    }

    fn dists_internal(&self) -> Vec<isize> {
        if self.dists_int.is_empty() {
            let mut dists: Vec<isize> = self
                .points_default()
                .iter()
                .map(|point| dist(&self.vert.into(), &point))
                .collect();
            dists.sort();
            dists
        } else {
            self.dists_int.to_owned()
        }
    }

    fn dists_external(&self) -> Vec<isize> {
        if self.dists_ext.is_empty() {
            let vert = self.external_vert();
            let mut dists: Vec<isize> = self
                .points_default()
                .iter()
                .map(|point| dist(&vert.into(), &point))
                .collect();
            dists.sort();
            dists
        } else {
            self.dists_ext.to_owned()
        }
    }

    fn points(&self, orient: &Orient, origin: &Point) -> Vec<Point> {
        self.pings
            .iter()
            .map(|ping| ping.to_point(orient, origin))
            .collect()
    }

    fn points_default(&self) -> Vec<Point> {
        self.pings.iter().map(|ping| (*ping).into()).collect()
    }

    fn external_vert(&self) -> Ping {
        let points = self.points_default();
        let corner = Orient::default().get_corner(&self.corner_pos);
        (
            if corner.x.is_neg() {
                all_xs(&points).iter().fold(SCAN_MAX, |a, x| min(a, *x))
            } else {
                all_xs(&points).iter().fold(SCAN_MIN, |a, x| max(a, *x))
            },
            if corner.y.is_neg() {
                all_ys(&points).iter().fold(SCAN_MAX, |a, y| min(a, *y))
            } else {
                all_ys(&points).iter().fold(SCAN_MIN, |a, y| max(a, *y))
            },
            if corner.z.is_neg() {
                all_zs(&points).iter().fold(SCAN_MAX, |a, z| min(a, *z))
            } else {
                all_zs(&points).iter().fold(SCAN_MIN, |a, z| max(a, *z))
            },
        )
            .into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Subspace {
    corner: Corner,
    orient: Orient,
    vert: Point,
    points: Vec<Point>,
    dists_ext: Vec<isize>,
    dists_int: Vec<isize>,
}

impl Subspace {

    fn copy(&self) -> Self {
        Self {
            corner: self.corner,
            orient: self.orient,
            vert: self.vert,
            points: self.points.to_owned(),
            dists_ext: self.dists_ext.to_owned(),
            dists_int: self.dists_int.to_owned(),
        }
    }

    fn dists_internal(&self) -> Vec<isize> {
        if self.dists_int.is_empty() {
            let mut dists: Vec<isize> = self
                .points
                .iter()
                .map(|point| dist(&self.vert, &point))
                .collect();
            dists.sort();
            dists
        } else {
            self.dists_int.to_owned()
        }
    }

    fn dists_external(&self) -> Vec<isize> {
        if self.dists_ext.is_empty() {
            let vert = self.external_vert();
            let mut dists: Vec<isize> = self
                .points
                .iter()
                .map(|point| dist(&vert, &point))
                .collect();
            dists.sort();
            dists
        } else {
            self.dists_ext.to_owned()
        }
    }

    fn external_vert(&self) -> Point {
        (
            if self.corner.x.is_neg() {
                all_xs(&self.points)
                    .iter()
                    .fold(SCAN_MAX, |a, x| min(a, *x))
            } else {
                all_xs(&self.points)
                    .iter()
                    .fold(SCAN_MIN, |a, x| max(a, *x))
            },
            if self.corner.y.is_neg() {
                all_ys(&self.points)
                    .iter()
                    .fold(SCAN_MAX, |a, y| min(a, *y))
            } else {
                all_ys(&self.points)
                    .iter()
                    .fold(SCAN_MIN, |a, y| max(a, *y))
            },
            if self.corner.z.is_neg() {
                all_zs(&self.points)
                    .iter()
                    .fold(SCAN_MAX, |a, z| min(a, *z))
            } else {
                all_zs(&self.points)
                    .iter()
                    .fold(SCAN_MIN, |a, z| max(a, *z))
            },
        )
    }

    fn intersects(&self, other: &Subspace) -> Option<Relativity> {
        if let possible @ Some((ping, _)) = self.may_intersect(other) {
            let mut lpoints: Vec<Point> = self.points.to_owned();
            lpoints.sort();
            let mut rpoints: Vec<Point> = other
                .points
                .iter()
                .map(|point| Ping::from_point(point).to_point(&Orient::default(), &ping.into()))
                .collect();
            rpoints.sort();
            if lpoints == rpoints {
                return possible;
            }
        }
        None
    }

    fn may_intersect(&self, other: &Subspace) -> Option<Relativity> {
        if self.dists_internal() == other.dists_external() {
            // 1. create translation mats from other.external_vert, to DEFAULT_ORIGIN
            // 2. translate default origin to save as other origin
            // 3. add x,y,z from other origin to self.vert (x,y,z)
            let mats = t_translate(&other.external_vert(), &(0, 0, 0));
            let (ox, oy, oz) = transform_point(&(0, 0, 0), &vec![mats]);
            let (sx, sy, sz) = self.vert;

            return Some((Ping::from_point(&(sx + ox, sy + oy, sz + oz)), other.orient));
        }
        None
    }
}

impl ToString for Subspace {
    fn to_string(&self) -> String {
        let mut s: String = format!("corner: {:?}", self.corner);
        let vert = self.vert;
        let ext_vert = self.external_vert();
        s = s + format!("\n vert [int: {:?}] [ext: {:?}]", vert, ext_vert).as_str();
        for point in &self.points {
            s = s + format!(
                "\n  [{} / {}] {:?}",
                dist(&vert, &point),
                dist(&ext_vert, &point),
                point
            )
            .as_str();
        }
        s
    }
}

const DEFAULT_ORIGIN: Point = (0, 0, 0);
const IDENTITY_TRANSLATION: (Point, Point) = (DEFAULT_ORIGIN, DEFAULT_ORIGIN);

fn _tx_point(
    point: &Point,
    rot_mats: &Vec<Array2<isize>>,
    trans_mats: &Vec<Array2<isize>>,
) -> Point {
    transform_point(&transform_point(point, &rot_mats), &trans_mats)
}

#[test]
fn day19_test_scanner_corners_0_1() {
    let scanners = read_test();
    let scan_0 = &scanners[0].search();
    let scan_1 = scanners[1].search();
    // Comparing 0 to 1 must be at 68,-1246,-43 (relative to scanner 0).
    println!("comparing {} to {}", scan_0.scanner.id, scan_1.scanner.id);
    let result = scan_0.intersects(&scan_1);
    assert_ne!(None, result, "expect intersection");
    let tx_1_0 = &result.unwrap().1;
    let scan_1_scanner = &scanners[1].points(Some(tx_1_0.1), Some(tx_1_0.0.into()));
    assert_eq!((68, -1246, -43), tx_1_0.0.into(), "expect new origin");
    assert_eq!(
        ((68, -1246, -43).into(), Orient::new(NegZ, Y)),
        *tx_1_0,
        "expect transformation"
    );

    let within_1: Point = (686, 422, 578);
    let within_0: Point = (-618, -824, -621);

    assert!(
        &scanners[1].points(None, None).contains(&within_1),
        "expect scanner 1 contains test point"
    );
    assert!(
        !scan_1_scanner.contains(&within_1),
        "expect new scanner does not contain untranslated test point"
    );
    assert!(
        scan_1_scanner.contains(&within_0),
        "expect new scanner contains translated test point"
    );
}

struct Relatives {
    root: usize,
    rels: HashMap<(usize, usize), Relativity>,
}

impl Relatives {
    fn get(&self, id: usize) -> Vec<Relativity> {
        return self.get_with_path(id, &vec![]).unwrap_or(vec![]);
    }

    fn trace(&self, id: usize) -> (Point, Orient) {
        Self::trace_relativity(&self.get(id))
    }

    fn as_root(&self, root: usize) -> Self {
        Self {
            root: root,
            rels: self.rels.to_owned(),
        }
    }

    /// returns last scanner origin, and the last orient at that origin
    fn trace_relativity(hops: &[Relativity]) -> (Point, Orient) {
        hops.iter().fold(
            (DEFAULT_ORIGIN, Orient::default()),
            |(origin, old_orient), (ping, new_orient)| {
                (
                    ping.to_point(&old_orient, &origin),
                    old_orient.rotate(&Orient::default().rotation_to(&new_orient)),
                )
            },
        )
    }
    fn get_with_path(&self, id: usize, path: &Vec<usize>) -> Option<Vec<Relativity>> {
        if id == self.root {
            return Some(vec![]);
        } else {
            if let Some(to_root) = self
                .rels
                .iter()
                .filter(|((left, right), _)| *right == id && *left == self.root)
                .map(|(_, rel)| rel)
                .nth(0)
            {
                return Some(vec![*to_root]);
            } else {
                for (left, rel) in self
                    .rels
                    .iter()
                    .filter(|((left, right), _)| *right == id && !path.contains(left))
                    .map(|((left, _), rel)| (left, rel))
                {
                    let next_path = vec![path.to_owned(), vec![id]].concat();
                    if let Some(prior_rels) = self.get_with_path(*left, &next_path) {
                        return Some(vec![prior_rels, vec![*rel]].concat());
                    }
                }
            }
        }
        None
    }
}

impl ToString for Relatives {
    fn to_string(&self) -> String {
        return format!(
            "Relatives {{ root: {}, rels: {} }}",
            self.root,
            format!(
                "HashMap::from_iter([{}].iter().cloned())",
                self.rels
                    .iter()
                    .map(|(k, v)| format!("({:?}, {}),", k, relativity_to_string(v)))
                    .fold("".to_owned(), |acc, v| acc + v.as_str())
            )
        );
    }
}

fn day19part1_solution_relatives() -> Relatives {
    Relatives {
        root: 0,
        rels: HashMap::from_iter(
            [
                ((7, 2), ((-1359, -32, -81).into(), Orient::new(NegZ, NegX))),
                ((9, 21), ((-146, -97, -1174).into(), Orient::new(X, NegY))),
                ((20, 19), ((74, -1154, 54).into(), Orient::new(X, Z))),
                ((24, 6), ((-129, -168, -1129).into(), Orient::new(NegZ, Y))),
                ((18, 4), ((18, -54, -1215).into(), Orient::new(NegY, Z))),
                ((4, 18), ((-18, 1215, -54).into(), Orient::new(Y, NegZ))),
                ((4, 25), ((-135, -1292, -15).into(), Orient::new(NegZ, Y))),
                ((10, 6), ((49, 95, -1334).into(), Orient::new(NegX, NegY))),
                ((13, 6), ((116, 27, 1308).into(), Orient::new(Y, NegZ))),
                ((21, 23), ((-24, -1181, 30).into(), Orient::new(Y, NegZ))),
                ((17, 15), ((-1310, 101, 28).into(), Orient::new(NegZ, NegY))),
                ((6, 13), ((-116, 1308, -27).into(), Orient::new(NegY, Z))),
                ((17, 7), ((-11, 104, -1040).into(), Orient::new(X, Z))),
                ((25, 4), ((-135, 1292, -15).into(), Orient::new(NegZ, Y))),
                ((2, 1), ((-27, 1061, -64).into(), Orient::new(NegY, Z))),
                ((21, 9), ((1174, -97, 146).into(), Orient::new(X, NegY))),
                ((7, 17), ((-104, 1040, 11).into(), Orient::new(Y, X))),
                ((12, 15), ((-1043, -103, 0).into(), Orient::new(NegZ, Y))),
                ((9, 0), ((39, -1254, -75).into(), Orient::new(X, Z))),
                ((6, 10), ((-1334, 95, 49).into(), Orient::new(NegX, NegY))),
                ((21, 25), ((58, -3, 1211).into(), Orient::new(Y, NegX))),
                ((2, 14), ((-1070, -126, 44).into(), Orient::new(X, NegZ))),
                ((2, 16), ((-31, -154, -1282).into(), Orient::new(NegX, Z))),
                ((18, 6), ((37, 1063, -108).into(), Orient::new(NegX, Z))),
                ((19, 20), ((1154, -54, -74).into(), Orient::new(Y, X))),
                ((25, 17), ((-121, -1071, 85).into(), Orient::new(NegZ, X))),
                ((15, 12), ((-1043, 103, 0).into(), Orient::new(NegZ, Y))),
                ((11, 14), ((11, -1378, -55).into(), Orient::new(NegY, NegX))),
                ((0, 7), ((35, 41, 1043).into(), Orient::new(NegY, X))),
                ((1, 22), ((9, 1163, -33).into(), Orient::new(Y, X))),
                ((8, 1), ((152, -1245, 1).into(), Orient::new(NegY, NegZ))),
                ((7, 0), ((1043, -35, 41).into(), Orient::new(NegX, NegZ))),
                ((17, 25), ((1071, 121, 85).into(), Orient::new(NegZ, X))),
                (
                    (20, 16),
                    ((-1090, -106, 57).into(), Orient::new(NegY, NegX)),
                ),
                ((0, 9), ((1254, 75, -39).into(), Orient::new(Y, X))),
                ((2, 7), ((-32, -1359, -81).into(), Orient::new(NegZ, NegX))),
                ((4, 19), ((1090, -9, -93).into(), Orient::new(Y, NegX))),
                ((15, 17), ((1310, 101, 28).into(), Orient::new(NegZ, NegY))),
                ((14, 11), ((55, 11, -1378).into(), Orient::new(X, NegZ))),
                ((14, 2), ((-126, 44, 1070).into(), Orient::new(NegY, NegX))),
                ((22, 1), ((33, -9, -1163).into(), Orient::new(X, Z))),
                ((14, 17), ((1129, 136, -2).into(), Orient::new(NegZ, X))),
                ((21, 5), ((6, 44, -1008).into(), Orient::new(Z, NegX))),
                ((6, 24), ((-129, 168, -1129).into(), Orient::new(NegZ, Y))),
                ((20, 24), ((1358, -1, -4).into(), Orient::new(NegZ, X))),
                ((6, 18), ((1063, 108, 37).into(), Orient::new(Y, NegX))),
                ((8, 3), ((92, 68, 1158).into(), Orient::new(Y, X))),
                ((17, 9), ((-45, 1186, 179).into(), Orient::new(NegX, Z))),
                ((24, 20), ((1, -1358, -4).into(), Orient::new(NegZ, X))),
                ((3, 8), ((-1158, -92, -68).into(), Orient::new(X, Z))),
                ((16, 20), ((-57, -1090, -106).into(), Orient::new(X, NegZ))),
                ((5, 21), ((-44, 6, 1008).into(), Orient::new(Z, X))),
                ((25, 21), ((1211, 58, 3).into(), Orient::new(NegX, Z))),
                ((16, 2), ((-154, 1282, -31).into(), Orient::new(Y, NegX))),
                ((19, 4), ((-93, 1090, 9).into(), Orient::new(NegX, Z))),
                ((1, 8), ((152, 1, -1245).into(), Orient::new(NegY, NegZ))),
                ((1, 2), ((27, 64, 1061).into(), Orient::new(Y, NegZ))),
                ((17, 14), ((-136, -1129, -2).into(), Orient::new(NegZ, X))),
                ((9, 17), ((1186, -179, -45).into(), Orient::new(Y, NegX))),
                ((23, 21), ((24, 30, 1181).into(), Orient::new(NegY, Z))),
            ]
            .iter()
            .cloned(),
        ),
    }
}

#[test]
fn day19part1() {
    let scanners = read();
    // switch these declarations to rebuild the relationships. takes over two minutes.
    let relatives = day19part1_solution_relatives();
    //let relatives = ScannerSearch::search_all(&scanners);
    //println!("relatives:");
    //println!("{}", relatives.to_string());

    assert_eq!(
        332,
        ScannerSearch::collect_beacons(&scanners, &relatives).len(),
        "expect true number of beacons"
    );
}

#[test]
fn day19part2() {
    let scanners = read();
    let relatives = day19part1_solution_relatives();
    let dists = ScannerSearch::man_dists(&scanners, &relatives);
    let (pair, max_dist) = dists.iter().fold(((0, 0), 0), |(last_pair, last_dist), (pair, dist)| {
        if *dist > last_dist {
            (*pair, *dist)
        } else {
            (last_pair, last_dist)
        }
    });
    assert_eq!(8507, max_dist, "expect max dist");
}

#[test]
fn day19_test_pre_part2() {
    let scanners = read_test();
    let relatives = Relatives {
        root: 0,
        rels: HashMap::from_iter(
            [
                ((4, 2), ((168, -1125, 72).into(), Orient::new(NegZ, X))),
                ((2, 4), ((1125, -168, 72).into(), Orient::new(NegZ, X))),
                ((1, 4), ((88, 113, -1104).into(), Orient::new(NegY, X))),
                ((1, 3), ((160, -1134, -23).into(), Orient::new(Z, Y))),
                ((3, 1), ((-160, 1134, 23).into(), Orient::new(Z, Y))),
                ((1, 0), ((68, 1246, -43).into(), Orient::new(NegZ, Y))),
                ((4, 1), ((-1104, -88, 113).into(), Orient::new(NegX, NegZ))),
                ((0, 1), ((68, -1246, -43).into(), Orient::new(NegZ, Y))),
            ]
            .iter()
            .cloned(),
        ),
    };

    let dists = ScannerSearch::man_dists(&scanners, &relatives);
    let (pair, max_dist) = dists.iter().fold(((0, 0), 0), |(last_pair, last_dist), (pair, dist)| {
        if *dist > last_dist {
            (*pair, *dist)
        } else {
            (last_pair, last_dist)
        }
    });
    assert_eq!(3621, max_dist, "expect max dist");
    assert!(pair == (2, 3) || pair == (3, 2), "expect pair is of two and three");
}

#[test]
fn day19_test_scanner_search_all() {
    let scanners = read_test();
    let relatives = ScannerSearch::search_all(&scanners);

    println!("matches:\n{:?}", relatives.rels.keys());

    {
        let (last_origin, _) = relatives.trace(0);
        assert_eq!((0, 0, 0), last_origin, "scanner 0 location")
    }

    {
        //let (last_origin, _) = trace_relativity(&[rels[&(0, 1)]]);
        let (last_origin, _) = relatives.trace(1);
        assert_eq!((68, -1246, -43), last_origin, "scanner 1 location")
    }

    {
        //let (last_origin, _) = trace_relativity(&[rels[&(0, 1)], rels[&(1, 4)], rels[&(4, 2)]]);
        let (last_origin, _) = relatives.trace(2);
        assert_eq!((1105, -1205, 1229), last_origin, "scanner 2 location")
    }

    {
        //let (last_origin, _) = trace_relativity(&[rels[&(0, 1)], rels[&(1, 3)]]);
        let (last_origin, _) = relatives.trace(3);
        assert_eq!((-92, -2380, -20), last_origin, "scanner 3 location")
    }

    {
        //let (last_origin, _) = trace_relativity(&[rels[&(0, 1)], rels[&(1, 4)]]);
        let (last_origin, _) = relatives.trace(4);
        assert_eq!((-20, -1133, 1061), last_origin, "scanner 4 location")
    }

    assert_eq!(
        79,
        ScannerSearch::collect_beacons(&scanners, &relatives).len(),
        "expect true number of beacons"
    );

    println!("relatives:");
    println!("{}", relatives.to_string());

    let relatives_from_str = Relatives {
        root: 0,
        rels: HashMap::from_iter(
            [
                ((4, 2), ((168, -1125, 72).into(), Orient::new(NegZ, X))),
                ((2, 4), ((1125, -168, 72).into(), Orient::new(NegZ, X))),
                ((1, 4), ((88, 113, -1104).into(), Orient::new(NegY, X))),
                ((1, 3), ((160, -1134, -23).into(), Orient::new(Z, Y))),
                ((3, 1), ((-160, 1134, 23).into(), Orient::new(Z, Y))),
                ((1, 0), ((68, 1246, -43).into(), Orient::new(NegZ, Y))),
                ((4, 1), ((-1104, -88, 113).into(), Orient::new(NegX, NegZ))),
                ((0, 1), ((68, -1246, -43).into(), Orient::new(NegZ, Y))),
            ]
            .iter()
            .cloned(),
        ),
    };
    assert_eq!(
        79,
        ScannerSearch::collect_beacons(&scanners, &relatives_from_str).len(),
        "expect true number of beacons from deserialized relatives"
    );
    // let scan_0 = scanners[0].search();
    // let scan_1 = scanners[1].search(); //.search_with_options(Some(Orient::new(NegZ, Y)), Some((68,-1246,-43)));
    // let scan_4 = scanners[4].search(); //.search_with_options(None, Some((-20,-1133,1061))); // .search_with_options(Some(Orient::new(NegZ, Y)), Some((68,-1246,-43)));

    // for ((lsub, rsub), ixn) in scan_1.all_intersects(&mut scan_4) {
    //     println!("ixn {}", relativity_to_string(&ixn));
    // }

    // if let Some(((lsub, rsub), rel_1_4)) = scan_1.intersects(&scan_4) {
    //     println!("rel_1_4 {}", relativity_to_string(&rel_1_4));
    // }
}

#[test]
fn day19_test_trace_relativity() {
    let hop1: Relativity = ((68, -1246, -43).into(), Orient::new(NegZ, Y));
    let (one_origin, _) = Relatives::trace_relativity(&[hop1]);
    assert_eq!((68, -1246, -43), one_origin, "expect one origin hop");

    let corrected = (88, 113, -1104);
    let hop2: Relativity = (corrected.into(), Orient::new(Z, NegY));
    let (last_origin, _) = Relatives::trace_relativity(&[hop1, hop2]);

    assert_eq!((-20, -1133, 1061), last_origin, "expect last origin");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Ping {
    aright: isize, // x dim
    afore: isize,  // y dim
    above: isize,  // z dim
}

impl Ping {
    fn from_point(point: &Point) -> Self {
        let (x, y, z) = *point;
        Self {
            aright: x,
            afore: y,
            above: z,
        }
    }

    fn to_point(&self, orient: &Orient, origin: &Point) -> Point {
        let mut as_arr: [isize; 3] = [0; 3];
        as_arr[orient.up.arr_idx()] = self.above * (if orient.up.is_neg() { -1 } else { 1 });
        as_arr[orient.fwd.arr_idx()] = self.afore * (if orient.fwd.is_neg() { -1 } else { 1 });
        let right_dim = orient.up.as_up_get_right_for(&orient.fwd);
        as_arr[right_dim.arr_idx()] = self.aright * (if right_dim.is_neg() { -1 } else { 1 });
        let (ox, oy, oz) = *origin;
        (ox + as_arr[0], oy + as_arr[1], oz + as_arr[2])
    }
}

impl Into<Point> for Ping {
    fn into(self) -> Point {
        self.to_point(&Orient::default(), &DEFAULT_ORIGIN)
    }
}

impl From<Point> for Ping {
    fn from(p: Point) -> Ping {
        Ping::from_point(&p)
    }
}

impl ToString for Ping {
    fn to_string(&self) -> String {
        let point: Point = (*self).into();
        return format!("{:?}.into()", point);
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: usize,
    pings: Vec<Ping>,
}

/// new approach #423:
/// intersection from corners
/// you shouldn't need to compute the intersection from the entirety of both spaces
/// as a corner encroaches
impl Scanner {
    fn new(id: usize, pings: Vec<Ping>) -> Self {
        Self {
            id: id,
            pings: pings,
        }
    }

    // fn filter_box(rotation: &Rotation, lower_bound: Point, upper_bound: Point) -> Vec<Point> {
    //     self.rotations[rotation]
    // }

    fn copy(&self) -> Self {
        Self {
            id: self.id,
            pings: self.pings.iter().cloned().collect(),
        }
    }

    fn points(&self, orient: Option<Orient>, origin: Option<Point>) -> Vec<Point> {
        let _orient = orient.unwrap_or(Orient::default());
        let _origin = origin.unwrap_or(DEFAULT_ORIGIN);
        self.pings
            .iter()
            .map(|ping| ping.to_point(&_orient, &_origin))
            .collect()
    }

    fn search(&self) -> ScannerSearch {
        ScannerSearch::new(self)
    }
}

type Relativity = (Ping, Orient);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SubspaceKey {
    orient: Orient,
    corner: Corner,
    dists_ext: Vec<isize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SubspaceCalcKey {
    position: CornerPosition,
    pings: Vec<Ping>,
}

#[derive(Debug, Clone)]
struct ScannerSearch {
    scanner: Scanner,
    calc_storage: HashMap<CornerPosition, HashMap<Vec<Ping>, SubspaceCalc>>,
    calc_lookup: HashMap<SubspaceKey, SubspaceCalcKey>,
}

/// new approach #423:
/// intersection from corners
/// you shouldn't need to compute the intersection from the entirety of both spaces
impl ScannerSearch {
    fn collect_beacons(scanners: &Vec<Scanner>, relatives: &Relatives) -> HashSet<Point> {
        let mut collected: HashSet<Point> = HashSet::new();
        for i in 0..scanners.len() {
            let (origin, orient) = relatives.trace(i);
            collected.extend(scanners[i].points(Some(orient), Some(origin)));
        }
        collected
    }

    fn man_dists(scanners: &Vec<Scanner>, relatives: &Relatives) -> HashMap<(usize, usize), isize> {
        let mut dists: HashMap<(usize, usize), isize> = HashMap::new();
        for left in 0..scanners.len() {
            let i_rels = relatives.as_root(left);
            for right in 0..scanners.len() {
                let ((x, y, z), _) = i_rels.trace(right);
                dists.insert((left, right), x.abs() + y.abs() + z.abs());
            }
        }
        dists
    }

    fn search_all(scanners: &Vec<Scanner>) -> Relatives {
        let searches: Vec<ScannerSearch> = scanners.iter().map(|scan| scan.search()).collect();
        let mut rels: HashMap<(usize, usize), Relativity> = HashMap::new();
        for left in 0..searches.len() {
            for right in 0..searches.len() {
                if left == right {
                    continue;
                }
                if let Some((_, relativity)) = searches[left].intersects(&searches[right]) {
                    rels.insert((left, right), relativity);
                }
            }
        }
        Relatives {
            root: 0,
            rels: rels,
        }
    }

    fn new(scanner: &Scanner) -> Self {
        let mut search = Self {
            scanner: scanner.copy(),
            calc_storage: HashMap::new(),
            calc_lookup: HashMap::new(),
        };
        search.build_lookup();
        search
    }

    fn build_lookup<'c>(&'c mut self) {
        self.calc_storage = SubspaceCalc::select_all(12, &self.scanner);
        self.calc_lookup = SubspaceCalc::subspace_key_lookup(&self.calc_storage);
    }

    fn get_subspace(&self, key: &SubspaceKey) -> Option<Subspace> {
        self.calc_lookup
            .get(key)
            .and_then(|calc_key| {
                self.calc_storage
                    .get(&calc_key.position)
                    .and_then(|inner| inner.get(&calc_key.pings))
            })
            .map(|calc| calc.new_subspace(&key.orient))
    }

    fn lookup_calc<'s>(&'s self, calc_key: &SubspaceCalcKey) -> Option<&'s SubspaceCalc> {
        self.calc_storage
            .get(&calc_key.position)
            .and_then(|inner| inner.get(&calc_key.pings))
    }

    fn get_default_subspaces(&self) -> Vec<Subspace> {
        self.calc_lookup
            .iter()
            .filter(|(key, _)| key.orient.is_default())
            .flat_map(|(_, calc_key)| self.lookup_calc(calc_key))
            .map(|calc| calc.new_subspace(&Orient::default()))
            .collect()
    }

    fn compare_subspace(
        &self,
        rkey: &SubspaceKey,
        lsub: &Subspace,
    ) -> Option<((Subspace, Subspace), Relativity)> {
        if let Some(rsub) = self.get_subspace(rkey) {
            if rsub.points.len() != lsub.points.len() {
                return None;
            }
            if let Some(tx) = lsub.intersects(&rsub) {
                // println!(
                //     "{}\n{}\n{}",
                //     lsub.to_string(),
                //     rsub.to_string(),
                //     relativity_to_string(&tx)
                // );
                return Some(((lsub.copy(), rsub.copy()), tx));
            }
        }
        None
    }

    fn intersects(&self, other: &ScannerSearch) -> Option<((Subspace, Subspace), Relativity)> {
        for lsub in self.get_default_subspaces() {
            for rorient in Orient::all() {
                let rkey = SubspaceKey {
                    orient: rorient,
                    corner: lsub.corner.invert(),
                    dists_ext: lsub.dists_internal(),
                };
                let ixn = other.compare_subspace(&rkey, &lsub);
                if ixn.is_some() {
                    return ixn;
                }
            }
        }
        None
    }
}

const SCAN_MIN: isize = -1000;
const SCAN_MAX: isize = 1000;

use std::cmp::{max, min};
fn filter_box(lowerb: &Point, upperb: &Point, test: &Point) -> bool {
    let (lx, ly, lz) = *lowerb;
    let (ux, uy, uz) = *upperb;
    let (tx, ty, tz) = *test;
    tx >= min(lx, ux)
        && tx <= max(lx, ux)
        && ty >= min(ly, uy)
        && ty <= max(ly, uy)
        && tz >= min(lz, uz)
        && tz <= max(lz, uz)
}

fn dist(from: &Point, to: &Point) -> isize {
    let (x1, y1, z1) = *from;
    let (x2, y2, z2) = *to;
    let (dx, dy, dz) = (x2 - x1, y2 - y1, z2 - z1);
    ((dx.pow(2) + dy.pow(2) + dz.pow(2)) as f64).sqrt() as isize
}

fn dist_dims_raw(from: &Point, to: &Point) -> Point {
    let ((from_x, from_y, from_z), (to_x, to_y, to_z)) = (from, to);
    (to_x - from_x, to_y - from_y, to_z - from_z)
}

/// translation matrix:
/// [[1, 0, 0, tx],
///  [0, 1, 0, ty],
///  [0, 0, 1, tz],
///  [0, 0, 0, 1 ]],
fn t_translate(from: &Point, to: &Point) -> Array2<isize> {
    let (dx, dy, dz) = dist_dims_raw(from, to);
    arr2(&[[1, 0, 0, dx], [0, 1, 0, dy], [0, 0, 1, dz], [0, 0, 0, 1]])
}

fn t_identity() -> Array2<isize> {
    arr2(&[[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]])
}

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
fn t_rotate_x(rot: RotVal) -> Array2<isize> {
    return match rot {
        Zero => t_identity(),
        Cw1 => arr2(&[[1, 0, 0, 0], [0, 0, 1, 0], [0, -1, 0, 0], [0, 0, 0, 1]]),
        Cw2 => arr2(&[[1, 0, 0, 0], [0, -1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]]),
        Cw3 => arr2(&[[1, 0, 0, 0], [0, 0, -1, 0], [0, 1, 0, 0], [0, 0, 0, 1]]),
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
fn t_rotate_y(rot: RotVal) -> Array2<isize> {
    return match rot {
        Zero => t_identity(),
        Cw1 => arr2(&[[0, 0, -1, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1]]),
        Cw2 => arr2(&[[-1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]]),
        Cw3 => arr2(&[[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]]),
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
fn t_rotate_z(rot: RotVal) -> Array2<isize> {
    return match rot {
        Zero => t_identity(),
        // reversed CW1 and CW3 matrices to match left-hand modeling
        Cw3 => arr2(&[[0, -1, 0, 0], [1, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
        Cw2 => arr2(&[[-1, 0, 0, 0], [0, -1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
        Cw1 => arr2(&[[0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
    };
}

fn mat_to_point(mat: &Array2<isize>) -> Point {
    (mat[(0, 0)], mat[(1, 0)], mat[(2, 0)])
}

fn point_to_mat(point: &Point) -> Array2<isize> {
    let (x, y, z) = point;
    arr2(&[[*x], [*y], [*z], [1]])
}

fn relativity_to_string(relativity: &Relativity) -> String {
    let (ping, orient) = relativity;
    format!("({}, {})", ping.to_string(), orient.to_string())
}

fn _transform_point(point: &Point, transform: &Array2<isize>) -> Point {
    // let do_log = *point == (-618, -824, -621) || *point == (686, 422, 578);
    // if do_log {
    //     println!("_transform_point/bef: {:?} {:?}", point, transform);
    // }
    let result = mat_to_point(&transform.dot(&point_to_mat(point)));
    // if do_log {
    //     println!("_transform_point/aft: {:?}", result);
    // }
    result
}

fn transform_point(point: &Point, transform: &Vec<Array2<isize>>) -> Point {
    return _transform_point(
        point,
        &transform.iter().fold(t_identity(), |acc, t| acc.dot(t)),
    );
}

fn all_xs(points: &Vec<Point>) -> Vec<isize> {
    points.iter().map(|(x, _, _)| x).cloned().collect()
}

fn all_ys(points: &Vec<Point>) -> Vec<isize> {
    points.iter().map(|(_, y, _)| y).cloned().collect()
}

fn all_zs(points: &Vec<Point>) -> Vec<isize> {
    points.iter().map(|(_, _, z)| z).cloned().collect()
}
