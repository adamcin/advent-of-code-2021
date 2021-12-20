mod common;
fn read() -> Target {
    return Target {
        x_min: 169,
        x_max: 206,
        y_min: -108,
        y_max: -68,
    };
}
/// --- Day 17: Trick Shot ---
///
/// You finally decode the Elves' message. HI, the message says. You continue searching
/// for the sleigh keys.
///
/// Ahead of you is what appears to be a large ocean trench. Could the keys have fallen
/// into it? You'd better send a probe to investigate.
///
/// The probe launcher on your submarine can fire the probe with any integer velocity in
/// the x (forward) and y (upward, or downward if negative) directions. For example, an
/// initial x,y velocity like 0,10 would fire the probe straight up, while an initial
/// velocity like 10,-1 would fire the probe forward at a slight downward angle.
///
/// The probe's x,y position starts at 0,0. Then, it will follow some trajectory by moving
/// in steps. On each step, these changes occur in the following order:
///
/// 1. The probe's x position increases by its x velocity.
/// 2. The probe's y position increases by its y velocity.
/// 3. Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it
///    decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or
///    does not change if it is already 0.
/// 4. Due to gravity, the probe's y velocity decreases by 1.
///
/// For the probe to successfully make it into the trench, the probe must be on some
/// trajectory that causes it to be within a target area after any step. The submarine
/// computer has already calculated this target area (your puzzle input).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn init() -> Pos {
        return Pos { x: 0, y: 0 };
    }
    /// 1. The probe's x position increases by its x velocity.
    /// 2. The probe's y position increases by its y velocity.
    fn next(inst: &Inst) -> Pos {
        return Pos {
            x: inst.pos.x + inst.vel.x,
            y: inst.pos.y + inst.vel.y,
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vel {
    x: isize,
    y: isize,
}
impl Vel {
    fn init(x: isize, y: isize) -> Vel {
        return Vel { x: x, y: y };
    }
    /// 3. Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it
    ///    decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or
    ///    does not change if it is already 0.
    /// 4. Due to gravity, the probe's y velocity decreases by 1.
    fn next(inst: &Inst) -> Vel {
        let new_x = if inst.vel.x < 0 {
            inst.vel.x + 1
        } else if inst.vel.x > 0 {
            inst.vel.x - 1
        } else {
            0
        };
        return Vel {
            x: new_x,
            y: inst.vel.y - 1,
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Inst {
    pos: Pos,
    vel: Vel,
}

impl Inst {
    fn init(vel: Vel) -> Inst {
        return Inst {
            pos: Pos::init(),
            vel: vel,
        };
    }
    fn next(&self) -> Inst {
        return Inst {
            pos: Pos::next(self),
            vel: Vel::next(self),
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Eval {
    Hit,
    Miss { reason: String },
    Adjust { vel: Vel },
}
use Eval::*;

impl Target {
    fn eval(&self, inst: &Inst) -> Option<Eval> {
        if inst.pos.x >= self.x_min
            && inst.pos.x <= self.x_max
            && inst.pos.y >= self.y_min
            && inst.pos.y <= self.y_max
        {
            return Some(Hit);
        }
        if inst.pos.y < self.y_min && inst.vel.y < 0 {
            return Some(Miss {
                reason: "exceeded min_y".to_owned(),
            });
        }
        if (inst.pos.x < self.x_min || inst.pos.x > self.x_max) && inst.vel.x == 0 {
            return Some(Miss {
                reason: "zero x velocity out of bounds".to_owned(),
            });
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Probe {
    tgt: Target,
    path: Vec<Inst>,
    eval: Eval,
    label: String,
}
impl Probe {
    /// The probe's x,y position starts at 0,0. Then, it will follow some trajectory by moving
    /// in steps. On each step, these changes occur in the following order:
    ///
    /// 1. The probe's x position increases by its x velocity.
    /// 2. The probe's y position increases by its y velocity.
    /// 3. Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it
    ///    decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or
    ///    does not change if it is already 0.
    /// 4. Due to gravity, the probe's y velocity decreases by 1.
    fn fire(tgt: Target, vel: Vel, label: &str) -> Option<Probe> {
        let init = Inst::init(vel);
        let mut path = vec![init];
        for _ in (0..) {
            let last = path.last().unwrap();
            if let Some(eval) = tgt.eval(&last) {
                let probe = Probe {
                    tgt: tgt,
                    path: path,
                    eval: eval,
                    label: label.to_owned(),
                };
                if !probe.label.is_empty() {
                    println!("{}", probe.term_message());
                }
                return Some(probe);
            }
            path.push(last.next());
        }
        None
    }

    fn highest_position(&self) -> &Inst {
        return self
            .path
            .iter()
            .fold(self.path.first().unwrap(), |a, inst| {
                if a.pos.y < inst.pos.y {
                    return inst;
                } else {
                    return a;
                }
            });
    }

    fn term_message(&self) -> String {
        return format!(
            "{:?} -> {:?} -> {:?} -> {:?} : {}",
            self.path.first().unwrap().vel,
            self.highest_position().pos,
            self.path.last().unwrap().pos,
            self.eval,
            self.label,
        );
    }
}

/// If you're going to fire a highly scientific probe out of a super cool probe launcher,
/// you might as well do it with style. How high can you make the probe go while still
/// reaching the target area?
///
/// Find the initial velocity that causes the probe to reach the highest y position and
/// still eventually be within the target area after any step. What is the highest y
/// position it reaches on this trajectory?
#[test]
fn day17part1() {
    let tgt = read();
    // return Target {
    //     x_min: 169,
    //     x_max: 206,
    //     y_min: -108,
    //     y_max: -68,
    // };
    //
    // my process was purely manual here, based on a simple logic.
    // 1. use the fact that x velocity will degrade to zero, given enough hang time
    //    so, I used a sufficiently high y to let me tune the initial x velocity
    //    to reach zero within the min_x/max_x target bounds
    // 2. then, because there is a constant y acceleration, it implies that for
    //    whatever positive y velocity the probe leaves origin, it will be traveling
    //    at the same velocity in the opposite direction on the way down. when
    //    traveling at the fastest rate, the probe will travel from y=0 to y=-min in
    //    one step. so I tried an initial velocity of 108, and it just missed the
    //    target. changed it to 107 and it hit the lower bound exactly.
    // 3. the highest instant in the path was the correct asnwer
    let probe = Probe::fire(tgt, Vel::init(19, 107), "find highest y").unwrap();
    assert_eq!(
        5778,
        probe.highest_position().pos.y,
        "expect highest position"
    );
}

use std::collections::HashSet;

#[test]
fn day17_p2_lazy() {
    let hits = find_lazy_hits();
}

fn find_lazy_hits() -> HashSet<Vel> {
    let tgt = read();
    // returning to the solution from part1 to find the lazy lower x bound.
    // the laxy x velocities are those that will reach zero before the y position
    // reaches y_max.
    let lazy_x_probe = Probe::fire(tgt, Vel::init(18, 107), "lazy x probe").unwrap();
    let lazy_x_range = || 18..=19;
    // for each of these lazy x's, they implicitly limit the possible range of
    // associated y's. because it takes a minimum amount of time for x velocity
    // to reduce to zero.
    // since the y velocity degrades at the same rate as the x velocity, the
    // minimum initial y velocity is the one that reaches tgt.y_min in 18 or 19
    // steps.
    let lazy_y_probe = Probe::fire(tgt, Vel::init(18, 2), "lazy y probe").unwrap();
    // found misses between y=53 and y=66
    let lazy_y_range: Vec<isize> = (2..=107).filter(|y| *y < 53 || *y > 66).collect();

    let mut lazy_hits: HashSet<Vel> = HashSet::new();
    for x in lazy_x_range() {
        for y in &lazy_y_range {
            let vel = Vel::init(x, *y);
            let probe = Probe::fire(tgt, vel, "").unwrap();
            assert_eq!(
                Hit,
                probe.eval,
                "expect hit for combination of lazy x and lazy y: {}",
                probe.term_message()
            );
            lazy_hits.insert(vel);
        }
    }
    assert_eq!(
        (lazy_x_range().count() * lazy_y_range.len()) as isize,
        lazy_hits.len() as isize,
        "expect all hits with lazy x and lazy y combos"
    );
    return lazy_hits;
}

#[test]
fn day17part2() {
    let tgt = read();
    // then, we have to determine the bounds for all the flatter curves
    //
    // (min_x_vel,y) = (18,2) // minimum x velocity that hits the inner-bottom
    // corner of the target area
    //
    use std::collections::HashSet;
    let flat_xs: Vec<isize> = (0..=206).collect();
    let flat_x_range: HashSet<isize> = flat_xs.into_iter().collect();

    let mut hits: HashSet<Vel> = HashSet::new();
    hits = hits.union(&find_lazy_hits()).cloned().collect();
    for (x, ys) in flat_x_range.iter().map(|x| (x, (-108..107))) {
        for y in ys {
            let vel = Vel::init(*x, y);
            let probe = Probe::fire(tgt, vel, "").unwrap();
            if probe.eval == Hit {
                hits.insert(vel);
            }
        }
    }

    assert_eq!(2576, hits.len(), "expect number of hit velocities");
}
