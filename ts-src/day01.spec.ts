import { count, filter, map, ReplaySubject, skip, zip } from "rxjs";
import { readInput } from "./day01";

test('readInput emits n items', done => {
    readInput().pipe(count()).subscribe(n => {
        expect(n).toBe(2000);
        done();
    });
});

test('part 1 solution', done => {
    var subject = new ReplaySubject<number>();
    var stream0 = subject.asObservable();
    var stream1 = stream0.pipe(skip(1));
    zip(stream1, stream0).pipe(
        map(([next, value]) => next - value),
        filter((diff) => diff > 0),
        count()
    ).subscribe(n => {
        expect(n).toBe(1557);
        done();
    });
    readInput().subscribe(subject);
});

test('part 2 solution', done => {
    var subject = new ReplaySubject<number>();
    var stream0 = subject.asObservable();
    var stream1 = stream0.pipe(skip(1));
    var stream2 = stream0.pipe(skip(2));
    var sums0 = zip(stream2, stream1, stream0).pipe(
        map(([first, middle, last]) => first + middle + last),
    );
    var sums1 = sums0.pipe(skip(1));
    zip(sums1, sums0).pipe(
        map(([next, value]) => next - value),
        filter((diff) => diff > 0),
        count()
    ).subscribe(n => {
        expect(n).toBe(1608);
        done();
    });
    readInput().subscribe(subject);
});