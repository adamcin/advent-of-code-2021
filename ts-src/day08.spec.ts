import { bufferCount, count, filter, firstValueFrom, map, mergeMap, reduce } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { makeDecoder, readInput, sigIs1, sigIs4, sigIs7, sigIs8 } from "./day08";

const testScheduler = new TestScheduler((actual, expected) => {
    expect(actual).toEqual(expected);
});

test('readInput emits n items', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(count()));
    }).then(n => expect(n).toBe(200));
});


test('part 1 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(
            mergeMap(({ sets, outs }) => outs),
            filter((set) => sigIs1(set) || sigIs4(set) || sigIs7(set) || sigIs8(set)),
            reduce((acc, sig, index) => acc + 1, 0),
        ))
    }).then(n => expect(n).toBe(456));
});

test('part 2 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(
            map(({ sets, outs }) => makeDecoder(sets).decode(outs)),
            reduce((acc, value, index) => acc + value, 0),
        ))
    }).then(n => expect(n).toBe(1091609));
});

