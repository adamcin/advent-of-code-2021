import { count, filter, map, reduce, ReplaySubject, skip, zip } from "rxjs";
import { Diagnostic, readInput } from "./day03";

test('readInput emits n items', done => {
    readInput().pipe(count()).subscribe(n => {
        expect(n).toBe(1000);
        done();
    });
});

test('part 1 solution', done => {
    // we have to read it all in to memory in this challenge
    readInput().pipe(
        reduce((acc: Diagnostic[], diag: Diagnostic) => { acc.push(diag); return acc }, [])
    ).subscribe(diags => {
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
        done();
    });
});

test('part 2 solution', done => {
    // we have to read it all in to memory in this challenge
    readInput().pipe(
        reduce((acc: Diagnostic[], diag: Diagnostic) => { acc.push(diag); return acc }, [])
    ).subscribe(diags => {
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
        done();
    });
});