import * as fs from 'fs';
import * as path from 'path';
import * as Rx from 'rxjs';
import { map, scan, concatWith, concatMap } from 'rxjs/operators';
import { Observable } from 'rxjs';


export const readTestInput = (name: string): Observable<Buffer> => {
    var rsEmitter = fs.createReadStream(path.join('data', name, 'input.txt'), 'utf-8');
    return Rx.from(rsEmitter);
};

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
