import { map, Observable, reduce } from "rxjs";
import { readTestInputLines } from "./util";

export enum SyntaxCheckResultKind {
    Incomplete,
    Corrupt,
}

export type SyntaxCheckResult = { kind: SyntaxCheckResultKind, score: number };

export enum Delim {
    LRound,
    LSquare,
    LCurly,
    LAngle,
    RRound,
    RSquare,
    RCurly,
    RAngle,
}

const parseDelim = (c: string): Delim[] => {
    switch (c) {
        case '(': return [Delim.LRound];
        case '[': return [Delim.LSquare];
        case '{': return [Delim.LCurly];
        case '<': return [Delim.LAngle];
        case ')': return [Delim.RRound];
        case ']': return [Delim.RSquare];
        case '}': return [Delim.RCurly];
        case '>': return [Delim.RAngle];
        default: return [];
    }
};

export const isOpen = (delim: Delim): boolean => {
    switch (delim) {
        case Delim.LRound:
        case Delim.LSquare:
        case Delim.LCurly:
        case Delim.LAngle: return true;
        default: return false;
    }
}

export const isClose = (delim: Delim): boolean => {
    return !isOpen(delim);
}

export const matches = (left: Delim, right: Delim): boolean => {
    return (isClose(left) && matches(right, left))
        || left == Delim.LRound && right == Delim.RRound
        || left == Delim.LSquare && right == Delim.RSquare
        || left == Delim.LCurly && right == Delim.RCurly
        || left == Delim.LAngle && right == Delim.RAngle;
}

export const toClosing = (delim: Delim): Delim => {
    switch (delim) {
        case Delim.LRound: return Delim.RRound;
        case Delim.LSquare: return Delim.RSquare;
        case Delim.LCurly: return Delim.RCurly;
        case Delim.LAngle: return Delim.RAngle;
        default: return delim;
    }
}

export const corruptScore = (delim: Delim): number => {
    switch (delim) {
        case Delim.RRound: return 3;
        case Delim.RSquare: return 57;
        case Delim.RCurly: return 1197;
        case Delim.RAngle: return 25137;
        default: return 0;
    }
}

export const closeScore = (delim: Delim): number => {
    switch (delim) {
        case Delim.RRound: return 1;
        case Delim.RSquare: return 2;
        case Delim.RCurly: return 3;
        case Delim.RAngle: return 4;
        default: return 0;
    }
}

export const checkSyntax = (line: Delim[]): SyntaxCheckResult => {
    var stack: Delim[] = [];
    for (var i = 0; i < line.length; i++) {
        var delim = line[i];
        if (isOpen(delim)) {
            stack.push(delim);
        } else {
            if (stack.length > 0) {
                if (matches(stack.slice(-1)[0], delim)) {
                    stack.pop();
                } else {
                    return { kind: SyntaxCheckResultKind.Corrupt, score: corruptScore(delim) };
                }
            }
        }
    }
    return {
        score: stack.reverse().map(delim => toClosing(delim)).reduce((acc, value) => (5 * acc) + closeScore(value), 0),
        kind: SyntaxCheckResultKind.Incomplete
    };
}

export const readInput = (): Observable<Delim[]> => readTestInputLines('day-10').pipe(
    map(line => line.split('').flatMap(c => parseDelim(c))),
);
