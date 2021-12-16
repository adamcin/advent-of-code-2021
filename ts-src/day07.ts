import { map, mergeMap, Observable } from "rxjs";
import { readTestInputLines } from "./util";

export type costFn = (diff: number) => number;
export const alignTo = (crabs: number[], position: number, fncost: costFn): number => {
    return crabs.reduce((cost, crab) => cost + fncost(Math.abs(position - crab)), 0);
};
export const readInput = (): Observable<number> => readTestInputLines('day-07').pipe(
    mergeMap(line => line.split(',')),
    map(crab => { return parseInt(crab.trim()); }),
);