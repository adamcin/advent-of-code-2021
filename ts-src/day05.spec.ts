import { count, filter, firstValueFrom, from, map, Observable, reduce } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { comparePoints, dedupePoints, parseSegment, Point, readInput, Segment } from "./day05";

const testScheduler = new TestScheduler((actual, expected) => {
    expect(actual).toEqual(expected);
});

test('intersection', done => {
    let vert = new Segment({x: 7, y: 0}, {x: 7, y: 4});
    let horz = new Segment({x: 9, y: 4}, {x: 3, y: 4});
    let xns = vert.intersections(horz);
    expect(xns.length).toBe(1);   
    done(); 
});

test('readInput emits n items', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(count()));
    }).then(n => expect(n).toBe(500));
});

test('part 0 solution', () => {
    return testScheduler.run(async (_helpers) => {
        let src = `
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`;
        let input: Observable<Segment> = from(src.split('\n')
            .filter(line => line.length > 0)
            .map(line => parseSegment(line)));
        return firstValueFrom(input.pipe(
            filter((seg: Segment) => seg.isHorz() || seg.isVert()),
            reduce((segs: Segment[], seg: Segment) => {
                segs.push(seg);
                return segs;
            }, []),
            map(straights => {
                expect(straights.length).toBe(6);
                let xns: Point[] = straights
                    .flatMap(seg => straights
                        .flatMap(other =>
                            seg.intersections(other)));
                            // 0, 9
                            // 1, 9
                            // 2, 9
                            // 3, 4
                            // 7, 4
                console.log(dedupePoints(xns));
                return dedupePoints(xns);
            }),
        ));
    }).then(xns => expect(xns.length).toBe(5));
});

/// To avoid the most dangerous areas, you need to determine the number of points where at least 
/// two lines overlap.
/// 
/// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
test('part 1 solution', () => {
    return testScheduler.run(async (_helpers) => {
        return firstValueFrom(readInput().pipe(
            filter((seg: Segment) => seg.isHorz() || seg.isVert()),
            reduce((segs: Segment[], seg: Segment) => {
                segs.push(seg);
                return segs;
            }, []),
            map(straights => {
                expect(straights.length).toBe(357);
                let xns: Point[] = straights
                    .flatMap(seg => straights
                        .flatMap(other => seg.intersections(other)));
                return dedupePoints(xns);
            }),
        ));
    }).then(xns => expect(xns.length).toBe(7142));
});

/// To avoid the most dangerous areas, you need to determine the number of points where at least 
/// two lines overlap.
/// 
/// Consider all of the lines. At how many points do at least two lines overlap?
test('part 2 solution', () => {
    return testScheduler.run(async (_helpers) => {
        return firstValueFrom(readInput().pipe(
            reduce((segs: Segment[], seg: Segment) => {
                segs.push(seg);
                return segs;
            }, []),
            map(segs => {
                let xns: Point[] = segs
                    .flatMap(seg => segs
                        .flatMap(other => seg.intersections(other)));
                return dedupePoints(xns);
            }),
        ));
    }).then(xns => expect(xns.length).toBe(20012));
});
