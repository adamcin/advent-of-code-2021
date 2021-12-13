import { count, filter, firstValueFrom, map, ReplaySubject, skip, zip } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { readInput } from "./day01";

const testScheduler = new TestScheduler((actual, expected) => {
    // asserting the two objects are equal - required
    // for TestScheduler assertions to work via your test framework
    // e.g. using chai.
    expect(actual).toEqual(expected);
});

test('readInput emits n items', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(count()));
    }).then(n => expect(n).toBe(2000));
});

test('part 1 solution', () => {
    return testScheduler.run(async () => {
        var subject = new ReplaySubject<number>();
        var stream0 = subject.asObservable();
        var stream1 = stream0.pipe(skip(1));
        let finalStream = zip(stream1, stream0).pipe(
            map(([next, value]) => next - value),
            filter((diff) => diff > 0),
            count()
        );
        readInput().subscribe(subject);
        return firstValueFrom(finalStream);
    }).then(n => expect(n).toBe(1557));
});

test('part 2 solution', () => {
    return testScheduler.run(async () => {
        var subject = new ReplaySubject<number>();
        var stream0 = subject.asObservable();
        var stream1 = stream0.pipe(skip(1));
        var stream2 = stream0.pipe(skip(2));
        var sums0 = zip(stream2, stream1, stream0).pipe(
            map(([first, middle, last]) => first + middle + last),
        );
        var sums1 = sums0.pipe(skip(1));
        let finalStream = zip(sums1, sums0).pipe(
            map(([next, value]) => next - value),
            filter((diff) => diff > 0),
            count()
        );
        readInput().subscribe(subject);
        return firstValueFrom(finalStream);
    }).then(n => expect(n).toBe(1608));
});