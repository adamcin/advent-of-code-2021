import { map, Observable, reduce } from "rxjs";
import { readTestInputLines } from "./util";

export const CAVE_SIZE = 100;
export type Point = { row: number, col: number };

export class PointSet {
    rows: boolean[][];
    constructor() {
        var rows: boolean[][] = [];
        for (var irow = 0; irow < CAVE_SIZE; irow++) {
            rows.push(Array.from({ length: CAVE_SIZE }, () => false));
        }
        this.rows = rows;
    }

    add(point: Point) {
        var { row, col } = point;
        this.rows[row][col] = true;
    }

    has(point: Point): boolean {
        var { row, col } = point;
        return this.rows[row][col];
    }

    values(caveMap: CaveMap): Point[] {
        return caveMap.allPoints().filter(p => this.has(p));
    }
}

export class CaveMap {
    rows: number[][];
    constructor(rows: number[][]) {
        this.rows = rows;
    }

    height(point: Point): number {
        var { row, col } = point;
        return this.rows[row][col];
    }

    riskLevel(point: Point): number {
        return this.height(point) + 1;
    }

    adjacents(point: Point): Point[] {
        var adjs: Point[] = [];
        var { row, col } = point;
        if (row <= CAVE_SIZE && col <= CAVE_SIZE) {
            if (row > 0) {
                adjs.push({ row: row - 1, col: col });
            }
            if (col < CAVE_SIZE - 1) {
                adjs.push({ row: row, col: col + 1 });
            }
            if (row < CAVE_SIZE - 1) {
                adjs.push({ row: row + 1, col: col });
            }
            if (col > 0) {
                adjs.push({ row: row, col: col - 1 });
            }
        }
        return adjs;
    }

    allPoints(): Point[] {
        return Array.from({ length: CAVE_SIZE }, (vr, ir) => ir)
            .flatMap((row: number) => {
                return Array.from({ length: CAVE_SIZE }, (vc, ic) => ic)
                    .map((col: number) => { return { row: row, col: col }; });
            });
    }

    lowPoints(): Point[] {
        return this.allPoints().filter((c) => {
            var height = this.height(c);
            var lower = this.adjacents(c).filter((adj) => this.height(adj) <= height);
            return lower.length == 0;
        });
    }

    _expandBasin(basinPoints: PointSet, points: Point[]): Point[] {
        var toInsert = points.filter(p => this.height(p) < 9 && !basinPoints.has(p))
        for (var i = 0; i < toInsert.length; i++) {
            if (toInsert[i]) {
                basinPoints.add(toInsert[i]);
            }
        }
        return toInsert.flatMap(p => this.adjacents(p));
    }

    expandBasin(lowPoint: Point): Point[] {
        var basinPoints = new PointSet();
        var expansion = this._expandBasin(basinPoints, [lowPoint]);
        var expandedBasin = expansion.length > 0;
        while (expandedBasin) {
            expansion = this._expandBasin(basinPoints, expansion);
            expandedBasin = expansion.length > 0;
        }
        return basinPoints.values(this);
    }
}

export const readInput = (): Observable<CaveMap> => readTestInputLines('day-09').pipe(
    map(line => line.split('').map(c => parseInt(c))),
    reduce((acc: number[][], row: number[]) => { acc.push(row); return acc; }, []),
    map(rows => new CaveMap(rows)),
);
