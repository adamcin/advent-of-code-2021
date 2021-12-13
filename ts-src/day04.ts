import { bufferCount, filter, map, mergeMap, Observable, skip, take } from "rxjs";
import { readTestInputLines } from "./util";

export class Board {
    cells: number[][];
    marks: boolean[][];
    lastMark: number;
    isBingo: boolean;
    constructor(cells: number[][]) {
        this.cells = cells;
        this.marks = [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ];
        this.lastMark = 0;
        this.isBingo = false;
    }

    mark(value: number): boolean {
        let found = this.find(value);
        if (found != undefined) {
            let { row, col } = found;
            if (this.marks[row][col] === false) {
                this.marks[row][col] = true;
                this.lastMark = value;
                return true;
            }
        }
        return false;
    }

    find(value: number): { row: number, col: number } | undefined {
        for (var row = 0; row < 5; row++) {
            for (var col = 0; col < 5; col++) {
                if (this.cells[row][col] === value) {
                    return { row: row, col: col };
                }
            }
        }
    }

    score(): number {
        var total = 0;
        for (var row = 0; row < 5; row++) {
            for (var col = 0; col < 5; col++) {
                if (this.marks[row][col] === false) {
                    total += this.cells[row][col];
                }
            }
        }
        return total * this.lastMark;
    }

    checkBingo(): boolean {
        if (this.isBingo === true) {
            return true;
        }
        for (var row = 0; row < 5; row++) {
            if (this.marks[row][0] === true
                && this.marks[row][1] === true
                && this.marks[row][2] === true
                && this.marks[row][3] === true
                && this.marks[row][4] === true) {
                this.isBingo = true;
            }
        }
        for (var col = 0; col < 5; col++) {
            if (this.marks[0][col] === true
                && this.marks[1][col] === true
                && this.marks[2][col] === true
                && this.marks[3][col] === true
                && this.marks[4][col] === true) {
                this.isBingo = true;
            }
        }
        return this.isBingo;
    }
}

export const readCallInput = (): Observable<number> => {
    return readTestInputLines('day-04').pipe(
        take(1), // only use the first line
        mergeMap((line: string) => line.split(',').map(e => e.trim())),
        map(e => parseInt(e)),
    );
};

export const readBoardInput = (): Observable<Board> => {
    return readTestInputLines('day-04').pipe(
        skip(1), // skip the first line of calls
        filter((line: string) => line.trim().length > 0), // exclude empty lines
        map((line: string) => line.trim().split(/\s+/).map(e => parseInt(e))),
        bufferCount(5), // collect 5 lines into an array
        map(cells => new Board(cells)),
    );
};