import { Observable } from "rxjs";
import { map, filter } from 'rxjs/operators';
import { readTestInputLines } from "./util";

export const readInput = (): Observable<number> => readTestInputLines('day-01').pipe(
    map(value => parseInt(value)));