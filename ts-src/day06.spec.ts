import { count, filter, first, firstValueFrom, from, lastValueFrom, map, Observable, of, reduce, takeUntil, takeWhile } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { readInput } from "./day06";

const testScheduler = new TestScheduler((actual, expected) => {
    expect(actual).toEqual(expected);
});

test('readInput emits n items', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(count()));
    }).then(n => expect(n).toBe(1));
});

test('part 1 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput());
    }).then(async (fishPool) => {
        for (var i = 0; i < 80; i++) {
            fishPool.tick();
        }
        return fishPool.fishCount();
    }).then(fishCount => expect(fishCount).toBe(365862));
});

test('part 2 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput());
    }).then(async (fishPool) => {
        for (var i = 0; i < 256; i++) {
            fishPool.tick();
        }
        return fishPool.fishCount();
    }).then(fishCount => expect(fishCount).toBe(1653250886439));
});

