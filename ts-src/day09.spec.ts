import { bufferCount, count, filter, firstValueFrom, map, mergeMap, reduce } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { readInput } from "./day09";

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
        return firstValueFrom(readInput().pipe(
            mergeMap(input => input.lowPoints().map(p => input.riskLevel(p))),
            reduce((acc, risk) => acc + risk, 0),
        ))
    }).then(n => expect(n).toBe(570));
});

test('part 2 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(
            mergeMap(input => input.lowPoints().map(p => input.expandBasin(p).length)),
            reduce((acc: number[], value: number) => {
                var sortable = Array.of(...acc, value);
                sortable.sort((a, b) => a - b);
                return sortable.slice(1);
            }, [0, 0, 0]),
        ))
    }).then(triple => expect(triple.reduce((acc, value) => acc * value, 1)).toBe(899392));
});

