import { count, filter, firstValueFrom, map, mergeMap, Observable, reduce, ReplaySubject, skip, zip } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { Board, readBoardInput, readCallInput } from "./day04";

const testScheduler = new TestScheduler((actual, expected) => {
    // asserting the two objects are equal - required
    // for TestScheduler assertions to work via your test framework
    // e.g. using chai.
    expect(actual).toEqual(expected);
});

test('readCallInput emits n items', () => {
    return testScheduler.run(async (helpers) => {
        return await firstValueFrom(readCallInput().pipe(count()));
    }).then(n => expect(n).toBe(100));
});

test('readBoardInput emits n items', () => {
    return testScheduler.run(async (helpers) => {
        return await firstValueFrom(readBoardInput().pipe(count()));
    }).then(n => expect(n).toBe(100));
});

test('part 1 solution', () => {
    return testScheduler.run(async (helpers) => {
        var boards: Board[] = await firstValueFrom(readBoardInput().pipe(
            reduce((boards: Board[], board: Board) => {
                boards.push(board);
                return boards;
            }, [])
        ));
        return await firstValueFrom(readCallInput().pipe(
            mergeMap((call): number[] => {
                var scores: number[] = [];
                for (var i = 0; i < boards.length; i++) {
                    let board = boards[i];
                    if (board.mark(call) && board.checkBingo()) {
                        scores.push(board.score());
                    }
                }
                return scores;
            }),
            filter(score => score > 0),
        ));
    }).then((firstScore) => expect(firstScore).toBe(39984));
});

test('part 2 solution', () => {
    return testScheduler.run(async (helpers) => {
        var boards: Board[] = await firstValueFrom(readBoardInput().pipe(
            reduce((boards: Board[], board: Board) => {
                boards.push(board);
                return boards;
            }, [])
        ));
        var results = await firstValueFrom(readCallInput().pipe(
            reduce((acc: { rem: Board[], scores: number[] }, call: number) => {
                let { rem, scores } = acc;
                for (var i = 0; i < rem.length; i++) {
                    let board: Board = rem[i];
                    if (board.mark(call) && board.checkBingo()) {
                        scores.push(board.score());
                    }
                }
                return { 
                    rem: rem.filter((board) => !board.isBingo), 
                    scores: scores, 
                };
            }, { rem: boards, scores: [] }),
        ));
        return results.scores;
    }).then(scores => expect(scores[scores.length - 1]).toBe(8468));
});