import { count, filter, firstValueFrom, map, reduce, ReplaySubject, skip, zip } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { Diagnostic, readInput } from "./day03";

const testScheduler = new TestScheduler((actual, expected) => {
    // asserting the two objects are equal - required
    // for TestScheduler assertions to work via your test framework
    // e.g. using chai.
    expect(actual).toEqual(expected);
});

test('readInput emits n items', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(count()));
    }).then(n => expect(n).toBe(1000));
});

test('part 1 solution', () => {
    return testScheduler.run(async (helpers) => {
        // we have to read it all in to memory in this challenge
        return firstValueFrom(readInput().pipe(
            reduce((acc: Diagnostic[], diag: Diagnostic) => { acc.push(diag); return acc }, [])
        ));
    }).then(diags => {
        expect(diags.length).toBe(1000);

        var gamma = 0;
        var epsilon = 0;
        for (var i = 0; i < 12; i++) {
            var multi = Math.pow(2, 11 - i);
            var ones = diags.filter((diag) => diag[i]);
            if (ones.length >= diags.length / 2) {
                gamma += multi;
            } else {
                epsilon += multi;
            }
        }

        expect(gamma * epsilon).toBe(738234);
    })
});

test('part 2 solution', () => {
    return testScheduler.run(async (helpers) => {
        // we have to read it all in to memory in this challenge
        return firstValueFrom(readInput().pipe(
            reduce((acc: Diagnostic[], diag: Diagnostic) => { acc.push(diag); return acc }, [])
        ))
    }).then(diags => {
        expect(diags.length).toBe(1000);

        var gamma = 0;
        var epsilon = 0;
        for (var i = 0; i < 12; i++) {
            var multi = Math.pow(2, 11 - i);
            var ones = diags.filter((diag) => diag[i]);
            if (ones.length >= diags.length / 2) {
                gamma += multi;
            } else {
                epsilon += multi;
            }
        }

        expect(gamma * epsilon).toBe(738234);
    });
});