import { count } from "rxjs";
import { readInput } from "./day01";

test('readInput emits n items', done => {
    readInput().pipe(count()).subscribe(n => {
        expect(n).toBe(2000);
        done();
    });
});