import { Observable } from "rxjs";
import { map, filter } from 'rxjs/operators';
import { readTestInputLines } from "./util";

export const readInput = (): Observable<string> => readTestInputLines('day-01');