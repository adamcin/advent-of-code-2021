import { map, Observable } from "rxjs";
import { readTestInputLines } from "./util";

export type Point = { x: number, y: number };

function between(bound1: number, middle: number, bound2: number): boolean {
    return (middle >= bound1 && middle <= bound2)
        || (middle <= bound1 && middle >= bound2);
}

function urange(from: number, to: number): number[] {
    if (from > to) {
        return Array.from({ length: (from - to) + 1 }, (v, k) => k + to);
    } else {
        return Array.from({ length: (to - from) + 1 }, (v, k) => k + from);
    }
}

export function dedupePoints(points: Point[]): Point[] {
    var sorted = Array.from(points);
    sorted.sort(comparePoints);
    return sorted.reduce((acc: Point[], next: Point): Point[] => {
        if (acc.length == 0 || !pointsEqual(acc[acc.length - 1], next)) {
            acc.push(next);
        }
        return acc;
    }, []);
}

export function pointsEqual({ x: lx, y: ly }: Point, { x: rx, y: ry }: Point): boolean {
    return lx === rx && ly === ry;
}

export const comparePoints = (end1: Point, end2: Point): number => {
    if (pointsEqual(end1, end2)) {
        return 0;
    }
    let [{ x: x1, y: y1 }, { x: x2, y: y2 }] = [end1, end2];
    if (x2 < x1 || x2 === x1 && y2 < y1) {
        return 1;
    }
    return -1;
};

export class Segment {
    from: Point;
    to: Point;

    constructor(end1: Point, end2: Point) {
        [this.from, this.to] = [end1, end2].sort(comparePoints);
    }

    endpoints(): [Point, Point] {
        return [this.from, this.to];
    }

    isVert(): boolean {
        let [{ x: x1 }, { x: x2 }] = this.endpoints();
        return x1 === x2;
    }

    isHorz(): boolean {
        let [{ y: y1 }, { y: y2 }] = this.endpoints();
        return y1 === y2;
    }

    risesOverX(): boolean {
        let [{ x: x1, y: y1 }, { x: x2, y: y2 }] = this.endpoints();
        return x2 > x1 && y2 > y1;
    }

    fallsOverX(): boolean {
        let [{ x: x1, y: y1 }, { x: x2, y: y2 }] = this.endpoints();
        return x2 > x1 && y2 < y1;
    }

    isIdentical(other: Segment): boolean {
        return pointsEqual(this.from, other.from) && pointsEqual(this.to, other.to);
    }

    isCoincidentWith(other: Segment): boolean {
        if (this.isIdentical(other)) {
            return false;
        }
        return pointsEqual(this.from, other.from)
            || pointsEqual(this.from, other.to)
            || pointsEqual(this.to, other.from)
            || pointsEqual(this.to, other.to);
    }

    isParallelTo(other: Segment): boolean {
        return (this.isHorz() && other.isHorz())
            || (this.isVert() && other.isVert())
            || (this.risesOverX() && other.risesOverX())
            || (this.fallsOverX() && other.fallsOverX());
    }

    isCollinearWith(other: Segment): boolean {
        if (this.isIdentical(other)) {
            return false;
        }
        if (!this.isParallelTo(other)) {
            return false;
        }
        if (this.isVert()) {
            let [{ x: sx }, { x: ox }] = [this.from, other.from];
            return sx === ox;
        } else if (this.isHorz()) {
            let [{ y: sy }, { y: oy }] = [this.from, other.from];
            return sy === oy;
        } else if (this.risesOverX()) {
            let [{ x: sx, y: sy }, { x: ox, y: oy }] = [this.from, other.from];
            return sy - sx === oy - ox;
        } else if (this.fallsOverX()) {
            let [{ x: sx, y: sy }, { x: ox, y: oy }] = [this.from, other.from];
            return sy + sx === oy + ox;
        }
        return false;
    }

    solveForX(other: Segment): number | undefined {
        if (this.isParallelTo(other)) {
            return undefined;
        } else if (other.isHorz()) {
            return other.solveForX(this);
        } else if (this.isHorz()) {
            let { y: sy1 } = this.from;
            let { x: ox1, y: oy1 } = other.from;
            if (other.risesOverX()) {
                let z = oy1 - ox1;
                let x = sy1 - z;
                if (x >= 0) {
                    return x;
                }
            } else if (other.fallsOverX()) {
                let z = oy1 + ox1;
                let x = -1 * (sy1 - z);
                if (x >= 0) {
                    return x;
                }
            }
        } else if (this.risesOverX()) {
            let { x: sx1, y: sy1 } = this.from;
            let sz = sy1 - sx1;
            if (other.fallsOverX()) {
                let { x: ox1, y: oy1 } = other.from;
                let twoX = (oy1 + ox1) - sz // (oy1 + ox1) = oz
                if (twoX % 2 === 0 && twoX >= 0) {
                    return twoX / 2;
                }
            }
        }
        return undefined;
    }

    linearIntersection(other: Segment): Point | undefined {
        if (!this.isParallelTo(other)) {
            if (this.isVert() || other.isVert()) {
                return this.vertIntersection(other);
            }
            let [{ x: sx1, y: sy1 }, { x: sx2, y: sy2 }] = this.endpoints();
            let [{ x: ox1, y: oy1 }, { x: ox2, y: oy2 }] = other.endpoints();
            let x: number | undefined = this.solveForX(other);
            if (x != undefined && between(sx1, x, sx2) && between(ox1, x, ox2)) {
                let solved: Point | undefined = this.yForX(x);
                if (solved != undefined) {
                    let { y } = solved;
                    if (between(sy1, y, sy2) && between(oy1, y, oy2)) {
                        return { x: x, y: y };
                    }
                }
            }
        }
        return undefined;
    }

    yForX(x: number): Point | undefined {
        let [{ x: x1, y: y1 }, { y: y2 }] = [this.from, this.to];
        if (this.isHorz()) {
            return { x: x, y: y1 };
        } else if (this.fallsOverX()) {
            let z = y1 + x1;
            let y = z - x;
            if (y >= 0 && between(y1, y, y2)) {
                return { x: x, y: y };
            }
        } else if (this.risesOverX()) {
            let z = y1 - x1;
            let y = z + x;
            if (y >= 0 && between(y1, y, y2)) {
                return { x: x, y: y };
            }
        }
        return undefined;
    }

    vertIntersection(other: Segment): Point | undefined {
        if (this.isVert()) {
            if (other.isVert()) {
                return undefined;
            }
            let [{ x: sx1, y: sy1 }, { y: sy2 }] = this.endpoints();
            let [{ x: ox1, y: oy1 }, { x: ox2, y: oy2 }] = other.endpoints();
            let solved: Point | undefined = other.yForX(sx1);
            if (solved != undefined) {
                let { x, y } = solved;
                if (sx1 === x
                    && between(ox1, x, ox2)
                    && between(sy1, y, sy2)
                    && between(oy1, y, oy2)) {
                    return solved;
                }
            }
        } else if (other.isVert()) {
            return other.vertIntersection(this);
        }
        return undefined;
    }

    points(): Point[] {
        let [{ x: x1, y: y1 }, { x: x2, y: y2 }] = this.endpoints();
        if (this.isHorz()) {
            return urange(x1, x2).map(x => { return { x: x, y: y1 }; });
        } else if (this.isVert()) {
            return urange(y1, y2).map(y => { return { x: x1, y: y }; });
        } else if (this.risesOverX()) {
            let [minX, minY] = [Math.min(x1, x2), Math.min(y1, y2)];
            return urange(y1, y2).map(y => { return { x: minX + (y - minY), y }; });
        } else if (this.fallsOverX()) {
            let [maxX, minY] = [Math.max(x1, x2), Math.min(y1, y2)];
            return urange(y1, y2).map(y => { return { x: maxX - (y - minY), y }; });
        }
        return [];
    }

    intersections(other: Segment): Point[] {
        if (this.isIdentical(other)) {
            return [];
        }
        var linxn: Point | undefined = this.linearIntersection(other);
        if (linxn != undefined) {
            return [linxn];
        } else if (this.isCollinearWith(other)) {
            return this.points().filter(p => {
                return other.points().reduce((found: boolean, op: Point) => {
                    if (found) {
                        return found;
                    }
                    return pointsEqual(p, op);
                }, false);
            });
        }
        return [];
    }
};

export function parseSegment(line: string): Segment {
    let parts = line.split(' -> ').flatMap(half => half.split(',')).map(quart => parseInt(quart));
    return new Segment({ x: parts[0], y: parts[1] }, { x: parts[2], y: parts[3] });
}

export const readInput = (): Observable<Segment> => readTestInputLines('day-05').pipe(
    map(line => parseSegment(line)));