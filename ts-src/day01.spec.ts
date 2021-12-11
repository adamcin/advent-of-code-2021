import { count, filter, map, ReplaySubject, skip, zip, zipWith } from "rxjs";
import { readInput } from "./day01";

test('readInput emits n items', done => {
    readInput().pipe(count()).subscribe(n => {
        expect(n).toBe(2000);
        done();
    });
});

test('part 1 solution', done => {
    var stream0 = readInput();
    var stream1 = readInput().pipe(skip(1));
    zip(stream1, stream0).pipe(
        map(([next, value]) => next - value),
        filter((diff) => diff > 0),
        count()
    ).subscribe(n => {
        expect(n).toBe(1557);
        done();
    });
})