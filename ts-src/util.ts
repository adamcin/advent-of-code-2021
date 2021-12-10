import * as fs from 'fs';
import * as path from 'path';
import * as Rx from 'rxjs';
import { map, scan, concatWith, concatMap } from 'rxjs/operators';

const _readTestInput =
    (name: string, callback: (err: NodeJS.ErrnoException | null, data: Buffer) => void): void =>
        fs.readFile(path.join('data', name, 'input.txt'), callback);



export const readTestInput = Rx.bindNodeCallback(_readTestInput);

export const readTestInputLines = (name: string) => readTestInput(name)
    .pipe(
        map(b => b.toString()),
        concatWith(Rx.of("\n")),
        scan<string, { buffer: string | undefined, items: string[] }>((acc, b) => {
            var buffer = acc.buffer || "";
            const splitted = buffer.concat(b).split("\n");
            const rest = splitted.pop();
            return { buffer: rest, items: splitted };
        }, { buffer: '', items: [] }),
        // Each item here is a pair { buffer: string, items: string[] }
        // such that buffer contains the remaining input text that has no newline
        // and items contains the lines that have been produced by the last buffer
        concatMap(({ items }) => items)
        // we flatten this into a sequence of items (strings)
    );
