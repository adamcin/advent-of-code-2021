import { map, Observable } from "rxjs";
import { readTestInputLines } from "./util";

export const readInput = (): Observable<number> => readTestInputLines('day-01').pipe(
    map(value => parseInt(value)));