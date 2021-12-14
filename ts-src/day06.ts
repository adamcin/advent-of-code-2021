import { map, mergeMap, Observable, reduce } from "rxjs";
import { readTestInputLines } from "./util";

const COHORT_MAX_AGE_MATURE = 7;
export type FishCohort = { size: number, maturity: number };
export class FishPool {
    day: number = 0;
    noobs: FishCohort[] = [];
    matures: FishCohort[] = [];

    constructor(fishes: number[]) {
        for (var i = 0; i < COHORT_MAX_AGE_MATURE; i++) {
            let cohortAge: number = (COHORT_MAX_AGE_MATURE - 1 + i) % COHORT_MAX_AGE_MATURE;
            this.matures.push({ maturity: 0, size: fishes.filter(age => age === cohortAge).length });
        }
    }

    tick(): { day: number, fishes: number } {
        this.day += 1;
        let { matured, stillNoobs } = this.noobs.reduce(({ matured, stillNoobs }: { matured: number, stillNoobs: FishCohort[] }, noob) => {
            if (noob.maturity <= this.day) {
                return { matured: matured + noob.size, stillNoobs: stillNoobs };
            }
            stillNoobs.push(noob);
            return { matured: matured, stillNoobs: stillNoobs };
        }, { matured: 0, stillNoobs: [] });
        this.noobs = stillNoobs;
        this.matures[(this.day + 2) % COHORT_MAX_AGE_MATURE].size += matured;

        let babySize = this.matures[this.day % COHORT_MAX_AGE_MATURE].size;
        this.noobs.push({ size: babySize, maturity: this.day + 7 });
        return { day: this.currentDay(), fishes: this.fishCount() };
    }

    currentDay(): number {
        return this.day;
    }

    fishCount(): number {
        return this.matures.reduce((total, cohort) => total + cohort.size, 0)
            + this.noobs.reduce((total, cohort) => total + cohort.size, 0);
    }
};

export const readInput = (): Observable<FishPool> => readTestInputLines('day-06').pipe(
    mergeMap(line => line.split(',')),
    map(fish => { return parseInt(fish.trim()); }),
    reduce((fishes: number[], fish) => { fishes.push(fish); return fishes; }, []),
    map((fishes: number[]) => new FishPool(fishes))
);