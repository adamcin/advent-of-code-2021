import { map, Observable } from "rxjs";
import { readTestInputLines } from "./util";

export type Diagnostic = boolean[];

function parseDiagnostic(line: string): Diagnostic {
    var diag: Diagnostic = [];
    for (var i = 0; i < 12; i++) {
        if (line.length > i) {
            diag.push('1' == line.charAt(i));
        }
    }
    return diag;
}

function filterBits(input: Diagnostic[], index: number): { ones: Diagnostic[], zeros: Diagnostic[] } {
    var ones = input.filter((diag: Diagnostic) => diag[index]);
    var zeros = input.filter((diag: Diagnostic) => !diag[index]);
    return { ones: ones, zeros: zeros };
}

export type ChooserFn = (initLen: number, ones: Diagnostic[], zeros: Diagnostic[]) => Diagnostic[];

function listPowers(): number[] {
    return [11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
}

function diagnosticToNumber(diag: Diagnostic): number {
    var powers = listPowers();
    return powers.reduce((acc, power, index) => acc + ((diag[index] ? 1 : 0) * Math.pow(2, power)), 0);
}

export const searchDiagnostics = (original: Diagnostic[], chooser: ChooserFn): number => {
    var powers: number[] = listPowers();
    var reduced: Diagnostic[] = powers.reduce((init, power, index) => {
        if (init.length <= 1) {
            return init;
        }
        var { ones, zeros } = filterBits(init, index);
        return chooser(init.length, ones, zeros);
    }, original);
    return diagnosticToNumber(reduced[0]);
};

export const readInput = (): Observable<Diagnostic> => readTestInputLines('day-03').pipe(
    map(parseDiagnostic));