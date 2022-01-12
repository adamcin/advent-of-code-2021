import { bufferCount, count, filter, firstValueFrom, map, mergeMap, reduce } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { checkSyntax, readInput, SyntaxCheckResultKind } from "./day10";

const testScheduler = new TestScheduler((actual, expected) => {
    expect(actual).toEqual(expected);
});

test('readInput emits n items', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(count()));
    }).then(n => expect(n).toBe(90));
});


test('part 1 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(
            map(line => checkSyntax(line)),
            filter(({ kind, score }) => kind == SyntaxCheckResultKind.Corrupt),
            map(({ kind, score }) => score),
            reduce((acc, score) => acc + score, 0),
        ))
    }).then(n => expect(n).toBe(339411));
});

test('part 2 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(
            map(line => checkSyntax(line)),
            filter(({ kind, score }) => kind == SyntaxCheckResultKind.Incomplete && score > 0),
            map(({ kind, score }) => score),
            reduce((acc: number[], value: number) => {
                acc.push(value);
                return acc;
            }, []),
        )).then(completeScores => {
            completeScores.sort((a, b) => a - b);
            var middleIndex = Math.floor(completeScores.length / 2);
            var middleScore = completeScores[middleIndex];
            return middleScore;
        });
    }).then(n => expect(n).toBe(2289754624));
});

