import { map, Observable } from "rxjs";
import { readTestInputLines } from "./util";

export enum Signal {
    A = "a", B = "b", C = "c", D = "d", E = "e", F = "f", G = "g",
};

export type Signals = { set: Signal[] };

export type SignalsInput = { sets: Signals[], outs: Signals[] };

export const SIG_ALL: Signal[] = [Signal.A, Signal.B, Signal.C, Signal.D, Signal.E, Signal.F, Signal.G];

const sigSub = (left: Signals, right: Signals): Signals => {
    return { set: SIG_ALL.filter((sig) => left.set.includes(sig) && !right.set.includes(sig)) };
};

const sigAdd = (left: Signals, right: Signals): Signals => {
    return { set: SIG_ALL.filter((sig) => left.set.includes(sig) || right.set.includes(sig)) };
};

const sigIxn = (left: Signals, right: Signals): Signals => {
    return { set: SIG_ALL.filter((sig) => left.set.includes(sig) && right.set.includes(sig)) };
};

const sigXor = (left: Signals, right: Signals): Signals => {
    return { set: SIG_ALL.filter((sig) => (left.set.includes(sig) && !right.set.includes(sig)) || (!left.set.includes(sig) && right.set.includes(sig))) };
};

const sigEqu = (left: Signals, right: Signals): boolean => {
    if (left.set.length == right.set.length) {
        for (var i = 0; i < left.set.length; i++) {
            if (left.set[i] != right.set[i]) {
                return false;
            }
        }
        return true;
    }
    return false;
};

const parseSignals = (sig_s: string): Signals => {
    return { set: SIG_ALL.filter((sig) => sig_s.includes(sig)) };
};

const parseSignalsInput = (line: string): SignalsInput => {
    var [sets_s, outs_s] = line.split(" | ", 2);
    var sets = sets_s.split(' ').map((sig_s) => parseSignals(sig_s));
    var outs = outs_s.split(' ').map((sig_s) => parseSignals(sig_s));
    return { sets: sets, outs: outs };
};

export const readInput = (): Observable<SignalsInput> => readTestInputLines('day-08').pipe(
    map((line) => parseSignalsInput(line)),
);

const sigTopRight = (enc: Signals[]): Signals => {
    return sigSub(enc[1], sigIxn(enc[1], enc[6]));
};

const sigTopLeft = (enc: Signals[]): Signals => {
    return sigSub(sigAdd(enc[3], enc[4]), enc[3]);
};

const sigMiddle = (enc: Signals[]): Signals => {
    return sigSub(sigSub(enc[4], sigTopLeft(enc)), enc[1]);
};

export const sigIs1 = (sig: Signals): boolean => { 
    return sig.set.length == 2;
};

export const sigIs7 = (sig: Signals): boolean => { 
    return sig.set.length == 3;
};

export const sigIs4 = (sig: Signals): boolean => { 
    return sig.set.length == 4;
};

export const sigIs8 = (sig: Signals): boolean => { 
    return sig.set.length == 7;
};

const sigIs6 = (enc: Signals[], sig: Signals): boolean => { 
    return sig.set.length == 6 && sigIxn(sig, enc[1]).set.length == 1;
};

const sigIs3 = (enc: Signals[], sig: Signals): boolean => { 
    return sig.set.length == 5 && sigEqu(sig, sigAdd(sig, enc[1]));
};

const sigIs2 = (enc: Signals[], sig: Signals): boolean => { 
    return sig.set.length == 5 && sigEqu(sig, sigAdd(sigXor(enc[6], enc[4]), sigMiddle(enc)));
};

const sigIs5 = (enc: Signals[], sig: Signals): boolean => { 
    return sig.set.length == 5 && sigEqu(sig, sigAdd(sigSub(enc[3], sigTopRight(enc)), sigTopLeft(enc)));
};

const sigIs0 = (enc: Signals[], sig: Signals): boolean => { 
    return sig.set.length == 6 && sigEqu(sig, sigSub(enc[8], sigMiddle(enc)));
};

const sigIs9 = (enc: Signals[], sig: Signals): boolean => { 
    return sig.set.length == 6 && sigEqu(sig, sigAdd(enc[3], sigTopLeft(enc)));
};

export class Decoder {
    sets: Signals[];
    constructor(sets: Signals[]) {
        this.sets = sets;
    }

    decodeDigit(set: Signals): number {
        for (var i = 0; i < this.sets.length; i++) {
            if (sigEqu(set, this.sets[i])) {
                return i;
            }
        }
        return -1;
    }

    decode(outs: Signals[]): number {
        return outs.reduce((acc, set, index) => { 
            return acc + (this.decodeDigit(set) * Math.pow(10, (outs.length - 1) - index));
        }, 0);
    }
}

export const makeDecoder = (sets: Signals[]): Decoder => {
    var enc: Signals[] = Array.of(...sets);

    // phase 1, no dependencies
    for (var i = 0; i < sets.length; i++) {
        if (sigIs1(sets[i])) {
            enc[1] = sets[i];
        } else if (sigIs4(sets[i])) {
            enc[4] = sets[i];
        } else if (sigIs7(sets[i])) {
            enc[7] = sets[i];
        } else if (sigIs8(sets[i])) {
            enc[8] = sets[i];
        }
    }

    // phase 2, minimally dependent
    for (var i = 0; i < sets.length; i++) {
        if (sigIs6(enc, sets[i])) {
            enc[6] = sets[i];
        } else if (sigIs3(enc, sets[i])) {
            enc[3] = sets[i];
        }
    }

    // phase 3, final deciphering
    for (var i = 0; i < sets.length; i++) {
        if (sigIs2(enc, sets[i])) {
            enc[2] = sets[i];
        } else if (sigIs5(enc, sets[i])) {
            enc[5] = sets[i];
        } else if (sigIs0(enc, sets[i])) {
            enc[0] = sets[i];
        } else if (sigIs9(enc, sets[i])) {
            enc[9] = sets[i];
        }
    }

    return new Decoder(enc);
};
