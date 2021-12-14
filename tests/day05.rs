mod common;
/// --- Day 5: Hydrothermal Venture ---
///
/// You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce 
/// large, opaque clouds, so it would be best to avoid them if possible.
///
/// They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents 
/// (your puzzle input) for you to review.
/// 
/// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the 
/// coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These 
/// line segments include the points at both ends.
fn read() -> Vec<Segment> {
    let values: Vec<Segment> = common::read_test_input("data/day-05/input.txt")
        .iter()
        .cloned()
        .map(|line| read_segment(&line))
        .collect();
    values
}

fn read_segment(line: &str) -> Segment {
    let scalars: Vec<usize> = (*line)
        .split(" -> ")
        .take(2)
        .flat_map(|point| point.split(','))
        .flat_map(|scalar_s| scalar_s.parse().ok())
        .collect();
    let segment = Segment {
        from: (scalars[0], scalars[1]),
        to: (scalars[2], scalars[3]),
    };
    return segment.canonical();
}

#[test]
fn day05_subline_coincidence() {
    let short_h: Segment = Segment {
        from: (0, 9),
        to: (2, 9),
    };
    let long_h: Segment = Segment {
        from: (0, 9),
        to: (5, 9),
    };

    assert_eq!(
        true,
        short_h.is_coincident_with(&long_h),
        "expect horiz coincidence"
    );

    let interxs_h: Vec<Point> = short_h.intersections(&long_h);
    assert_eq!(3, interxs_h.len(), "expect size of horiz intersection");

    let short_v: Segment = Segment {
        from: (9, 0),
        to: (9, 2),
    };
    let long_v: Segment = Segment {
        from: (9, 0),
        to: (9, 5),
    };

    assert_eq!(
        true,
        short_v.is_coincident_with(&long_v),
        "expect vert coincidence"
    );

    let interxs_v: Vec<Point> = short_v.intersections(&long_v);
    assert_eq!(3, interxs_v.len(), "expect size of vert intersection");
}

#[test]
fn day05_headtail_coincidence() {
    let head_l: Segment = Segment {
        from: (9, 4),
        to: (3, 4),
    };
    let tail_l: Segment = Segment {
        from: (3, 4),
        to: (1, 4),
    };

    assert_eq!(
        true,
        head_l.is_coincident_with(&tail_l),
        "expect left coincidence"
    );

    let interxs_l: Vec<Point> = head_l.intersections(&tail_l);
    assert_eq!(1, interxs_l.len(), "expect size of left intersection");

    let head_u: Segment = Segment {
        from: (4, 9),
        to: (4, 3),
    };
    let tail_u: Segment = Segment {
        from: (4, 3),
        to: (4, 1),
    };

    assert_eq!(
        true,
        head_u.is_coincident_with(&tail_u),
        "expect up coincidence"
    );

    let interxs_u: Vec<Point> = head_u.intersections(&tail_u);
    assert_eq!(1, interxs_u.len(), "expect size of up intersection");

    let head_r: Segment = Segment {
        from: (1, 4),
        to: (3, 4),
    };
    let tail_r: Segment = Segment {
        from: (3, 4),
        to: (9, 4),
    };

    assert_eq!(
        true,
        head_r.is_coincident_with(&tail_r),
        "expect right coincidence"
    );

    let interxs_r: Vec<Point> = head_r.intersections(&tail_r);
    assert_eq!(1, interxs_r.len(), "expect size of right intersection");

    let head_d: Segment = Segment {
        from: (4, 1),
        to: (4, 3),
    };
    let tail_d: Segment = Segment {
        from: (4, 3),
        to: (4, 9),
    };

    assert_eq!(
        true,
        head_d.is_coincident_with(&tail_d),
        "expect down coincidence"
    );

    let interxs_d: Vec<Point> = head_d.intersections(&tail_d);
    assert_eq!(1, interxs_d.len(), "expect size of down intersection");
}

#[test]
fn day05part0() {
    let src = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let straights: Vec<Segment> = src
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| read_segment(line))
        .filter(|segment| segment.is_horizontal() || segment.is_vertical())
        .collect();
    assert_eq!(6, straights.len(), "expect number of straight segments");

    let comparisons: Vec<Segment> = straights.iter().cloned().collect();
    use std::collections::HashSet;
    let interxs: HashSet<Point> = straights
        .iter()
        .flat_map(|segment| {
            comparisons
                .iter()
                .flat_map(|other| segment.intersections(other))
        })
        .collect();

    let mut sorted: Vec<Point> = interxs.iter().cloned().collect();
    sorted.sort();

    let mut display: String = "".to_owned();
    for (x, y) in sorted.iter() {
        let formatted = format!("{}, {}\n", x, y);
        display.push_str(&formatted);
    }
    println!("{}", display.as_str());
    assert_eq!(
        5,
        interxs.len(),
        "expect number of intersections: {}",
        display
    );
}

/// To avoid the most dangerous areas, you need to determine the number of points where at least 
/// two lines overlap.
/// 
/// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
#[test]
fn day05part1() {
    use std::collections::HashSet;
    let segments = read();
    assert_eq!(500, segments.len(), "expect number of segments");

    let straights: HashSet<Segment> = segments
        .iter()
        .map(|&e| e.canonical())
        .filter(|segment| segment.is_horizontal() || segment.is_vertical())
        .collect();
    assert_eq!(357, straights.len(), "expect number of straight segments");

    let comparisons: Vec<Segment> = straights.iter().cloned().collect();

    let interxs: HashSet<Point> = straights
        .iter()
        .flat_map(|segment| {
            comparisons
                .iter()
                .flat_map(|other| segment.intersections(other))
        })
        .collect();

    let mut sorted: Vec<Point> = interxs.iter().cloned().collect();
    sorted.sort();
    assert_ne!(4022, interxs.len(), "4022 is too low");
    assert_ne!(8508, interxs.len(), "8508 is too high");
    assert_eq!(7142, interxs.len(), "expect number of intersections");
}

/// To avoid the most dangerous areas, you need to determine the number of points where at least 
/// two lines overlap.
/// 
/// Consider all of the lines. At how many points do at least two lines overlap?
#[test]
fn day05part2() {
    use std::collections::HashSet;

    let segments = read();
    assert_eq!(500, segments.len(), "expect number of segments");

    let comparisons: Vec<Segment> = segments.iter().cloned().collect();

    let interxs: HashSet<Point> = segments
        .iter()
        .flat_map(|segment| {
            comparisons
                .iter()
                .flat_map(|other| segment.intersections(other))
        })
        .collect();

    assert_ne!(10198, interxs.len(), "10198 is too low");
    assert_eq!(20012, interxs.len(), "expect number of intersections");
}

type Point = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Segment {
    from: Point,
    to: Point,
}

fn between(num: usize, lower: usize, upper: usize) -> bool {
    return (num >= lower && num <= upper) || (num >= upper && num <= lower);
}

fn urange(from: usize, to: usize) -> std::ops::RangeInclusive<usize> {
    if from > to {
        return to..=from;
    } else {
        return from..=to;
    }
}

impl Segment {
    fn is_vertical(&self) -> bool {
        return self.map_points1(|(x1, _), (x2, _)| -> bool {
            return x1 == x2;
        });
    }

    fn is_horizontal(&self) -> bool {
        return self.map_points1(|(_, y1), (_, y2)| -> bool {
            return y1 == y2;
        });
    }

    fn falls_over_x(&self) -> bool {
        return self.canonical().map_points1(|(x1, y1), (x2, y2)| -> bool {
            return x2 > x1 && y2 < y1;
        });
    }

    fn rises_over_x(&self) -> bool {
        return self.canonical().map_points1(|(x1, y1), (x2, y2)| -> bool {
            return x2 > x1 && y2 > y1;
        });
    }

    fn map_points1<T>(&self, f: fn(Point, Point) -> T) -> T {
        return f(self.from, self.to);
    }

    fn map_points2<T>(&self, other: &Segment, f: fn(Point, Point, Point, Point) -> T) -> T {
        return f(self.from, self.to, other.from, other.to);
    }

    fn is_coincident_with(&self, other: &Segment) -> bool {
        let left: Segment = self.canonical();
        let right: Segment = other.canonical();
        if left.from == right.from && left.to == right.to {
            return false;
        }
        return left.from == right.from
            || left.from == right.to
            || left.to == right.from
            || left.to == right.to;
    }

    fn is_colinear_with(&self, other: &Segment) -> bool {
        let left: Segment = self.canonical();
        let right: Segment = other.canonical();
        if left.from == right.from && left.to == right.to {
            return false;
        }

        if left.is_parallel_to(&right) {
            if left.is_vertical() {
                return left.map_points2(&right, |(sx1, _), (_, _), (ox1, _), (_, _)| -> bool {
                    return sx1 == ox1;
                });
            } else if left.is_horizontal() {
                return left.map_points2(&right, |(_, sy1), (_, _), (_, oy1), (_, _)| -> bool {
                    return sy1 == oy1;
                });
            } else if left.rises_over_x() {
                return left.map_points2(
                    &right,
                    |(sx1, sy1), (_, _), (ox1, oy1), (_, _)| -> bool {
                        let isy1: i32 = sy1 as i32;
                        let isx1: i32 = sx1 as i32;
                        let ioy1: i32 = oy1 as i32;
                        let iox1: i32 = ox1 as i32;
                        return isy1 - isx1 == ioy1 - iox1;
                    },
                );
            } else if left.falls_over_x() {
                return left.map_points2(
                    &right,
                    |(sx1, sy1), (_, _), (ox1, oy1), (_, _)| -> bool {
                        let isy1: i32 = sy1 as i32;
                        let isx1: i32 = sx1 as i32;
                        let ioy1: i32 = oy1 as i32;
                        let iox1: i32 = ox1 as i32;
                        return isy1 + isx1 == ioy1 + iox1;
                    },
                );
            }
        }
        return false;
    }

    fn canonical(&self) -> Segment {
        let (x1, y1) = self.from;
        let (x2, y2) = self.to;
        if x2 < x1 || x2 == x1 && y2 < y1 {
            return Segment {
                from: self.to,
                to: self.from,
            };
        } else {
            return *self;
        }
    }

    fn intersections(&self, other_segment: &Segment) -> Vec<Point> {
        let left: Segment = self.canonical();
        let right: Segment = other_segment.canonical();
        if left.from == right.from && left.to == right.to {
            return Vec::new();
        }
        if let Some(point) = left.linear_intersection(&right) {
            let mut interxs = Vec::new();
            interxs.push(point);
            return interxs;
        } else if left.is_colinear_with(&right) {
            use std::collections::HashSet;
            let left_points: HashSet<Point> = left.points().iter().cloned().collect();
            let right_points: HashSet<Point> = right.points().iter().cloned().collect();
            return left_points.intersection(&right_points).cloned().collect();
        }
        return Vec::new();
    }

    fn is_parallel_to(&self, other: &Segment) -> bool {
        return (self.is_horizontal() && other.is_horizontal())
            || (self.is_vertical() && other.is_vertical())
            || (self.rises_over_x() && other.rises_over_x())
            || (self.falls_over_x() && other.falls_over_x());
    }

    fn _solve_for_x(&self, other: &Segment) -> Option<usize> {
        if other.is_horizontal() {
            return other._solve_for_x(self);
        } else if self.is_horizontal() {
            let (_, sy1) = self.from;
            if other.rises_over_x() {
                let (ox1, oy1) = other.from;
                let z = (oy1 as i32) - (ox1 as i32);
                let ix = (sy1 as i32) - z;
                if ix >= 0 {
                    return Some(ix as usize);
                }
            } else if other.falls_over_x() {
                let (ox1, oy1) = other.from;
                let z = (oy1 as i32) + (ox1 as i32);
                let ix = -1 * ((sy1 as i32) - z);
                if ix >= 0 {
                    return Some(ix as usize);
                }
            }
        } else if self.rises_over_x() {
            let (sx1, sy1) = self.from;
            let sz = (sy1 as i32) - (sx1 as i32);
            if other.falls_over_x() {
                let (ox1, oy1) = other.from;
                let oz = (oy1 as i32) + (ox1 as i32);
                if (oz - sz) % 2 != 0 {
                    return None;
                }
                let ix = (oz - sz) / 2;
                if ix >= 0 {
                    return Some(ix as usize);
                }
            }
        }
        None
    }

    fn linear_intersection(&self, other: &Segment) -> Option<Point> {
        if !self.is_parallel_to(&other) {
            if self.is_vertical() || other.is_vertical() {
                return self._vertical_intersection(other);
            }
            let (sx1, sy1) = self.from;
            let (sx2, sy2) = self.to;
            let (ox1, oy1) = other.from;
            let (ox2, oy2) = other.to;
            return self
                ._solve_for_x(other)
                .filter(|x| between(*x, sx1, sx2) && between(*x, ox1, ox2))
                .and_then(|x| self._y_for_x(x))
                .filter(|(_, y1)| between(*y1, sy1, sy2) && between(*y1, oy1, oy2));
        }
        return None;
    }

    fn _y_for_x(&self, x: usize) -> Option<Point> {
        if self.is_horizontal() {
            let (_, y1) = self.from;
            return Some((x, y1));
        } else if self.falls_over_x() {
            let (x1, y1) = self.from;
            let (_, y2) = self.to;
            let iz = (y1 as i32) + (x1 as i32);
            let y = iz - (x as i32);
            if y >= 0 && between(y as usize, y1, y2) {
                return Some((x, (y as usize)));
            }
        } else if self.rises_over_x() {
            let (x1, y1) = self.from;
            let (_, y2) = self.to;
            let iz = (y1 as i32) - (x1 as i32);
            let y = iz + (x as i32);
            if y >= 0 && between(y as usize, y1, y2) {
                return Some((x, (y as usize)));
            }
        }
        None
    }

    fn _vertical_intersection(&self, other: &Segment) -> Option<Point> {
        if self.is_vertical() {
            if other.is_vertical() {
                return None;
            }
            let (sx1, sy1) = self.from;
            let (_, sy2) = self.to;
            let (ox1, oy1) = other.from;
            let (ox2, oy2) = other.to;
            return other._y_for_x(sx1).filter(|(x1, y1)| {
                sx1 == *x1
                    && between(*x1, ox1, ox2)
                    && between(*y1, sy1, sy2)
                    && between(*y1, oy1, oy2)
            });
        } else if other.is_vertical() {
            return other._vertical_intersection(self);
        }
        return None;
    }

    fn points(&self) -> Vec<Point> {
        let right = self.canonical();
        if right.is_horizontal() {
            return right.map_points1(|(x1, y1), (x2, _)| -> Vec<Point> {
                let mut points: Vec<Point> = Vec::new();
                for x in urange(x1, x2) {
                    points.push((x, y1))
                }
                return points;
            });
        }
        if right.is_vertical() {
            return right.map_points1(|(x1, y1), (_, y2)| -> Vec<Point> {
                let mut points: Vec<Point> = Vec::new();
                for y in urange(y1, y2) {
                    points.push((x1, y))
                }
                return points;
            });
        }
        if right.rises_over_x() {
            return right.map_points1(|(x1, y1), (x2, y2)| -> Vec<Point> {
                let mut points: Vec<Point> = Vec::new();
                for y in urange(y1, y2) {
                    points.push((std::cmp::min(x1, x2) + (y - std::cmp::min(y1, y2)), y))
                }
                return points;
            });
        }
        if right.falls_over_x() {
            return right.map_points1(|(x1, y1), (x2, y2)| -> Vec<Point> {
                let mut points: Vec<Point> = Vec::new();
                for y in urange(y1, y2) {
                    points.push((std::cmp::max(x1, x2) - (y - std::cmp::min(y1, y2)), y))
                }
                return points;
            });
        }
        return Vec::new();
    }
}

#[test]
fn day05_crossing() {
    let short_h: Segment = Segment {
        from: (0, 1),
        to: (2, 1),
    };
    let short_v: Segment = Segment {
        from: (1, 0),
        to: (1, 2),
    };

    assert_eq!(
        false,
        short_h.is_coincident_with(&short_v),
        "expect no coincidence"
    );

    let interxs: Vec<Point> = short_h.intersections(&short_v);
    assert_eq!(1, interxs.len(), "expect size of intersection");
    assert_eq!((1, 1), interxs[0], "expect intersection");
}
#[test]
fn day05_right_t() {
    let short_h: Segment = Segment {
        from: (0, 1),
        to: (2, 1),
    };
    let short_v: Segment = Segment {
        from: (0, 0),
        to: (0, 2),
    };

    assert_eq!(
        false,
        short_h.is_coincident_with(&short_v),
        "expect no coincidence"
    );

    let interxs: Vec<Point> = short_h.intersections(&short_v);
    assert_eq!(1, interxs.len(), "expect size of intersection");
    assert_eq!((0, 1), interxs[0], "expect intersection");
}

#[test]
fn day05_left_t() {
    let short_h: Segment = Segment {
        from: (0, 1),
        to: (2, 1),
    };
    let short_v: Segment = Segment {
        from: (2, 0),
        to: (2, 2),
    };

    assert_eq!(
        false,
        short_h.is_coincident_with(&short_v),
        "expect no coincidence"
    );

    let interxs: Vec<Point> = short_h.intersections(&short_v);
    assert_eq!(1, interxs.len(), "expect size of intersection");
    assert_eq!((2, 1), interxs[0], "expect intersection");
}

#[test]
fn day05_up_t() {
    let short_h: Segment = Segment {
        from: (0, 2),
        to: (2, 2),
    };
    let short_v: Segment = Segment {
        from: (1, 0),
        to: (1, 2),
    };

    assert_eq!(
        false,
        short_h.is_coincident_with(&short_v),
        "expect no coincidence"
    );

    let interxs: Vec<Point> = short_h.intersections(&short_v);
    assert_eq!(1, interxs.len(), "expect size of intersection");
    assert_eq!((1, 2), interxs[0], "expect intersection");
}

#[test]
fn day05_down_t() {
    let short_h: Segment = Segment {
        from: (0, 0),
        to: (2, 0),
    };
    let short_v: Segment = Segment {
        from: (1, 0),
        to: (1, 2),
    };

    assert_eq!(
        false,
        short_h.is_coincident_with(&short_v),
        "expect no coincidence"
    );

    let interxs: Vec<Point> = short_h.intersections(&short_v);
    assert_eq!(1, interxs.len(), "expect size of intersection");
    assert_eq!((1, 0), interxs[0], "expect intersection");
}
