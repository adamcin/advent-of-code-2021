import { map, Observable } from "rxjs";
import { readTestInputLines } from "./util";

export enum CmdType {
    Up, Down, Forward,
};

function cmdTypeForName(cmdName: string): CmdType | undefined {
    if (cmdName == 'up') {
        return CmdType.Up;
    } else if (cmdName == 'down') {
        return CmdType.Down;
    } else if (cmdName == 'forward') {
        return CmdType.Forward;
    }
}

export type SubCommand = { type?: CmdType, scale: number };

export type Coord = { x: number, y: number };
export type AimCoord = { x: number, y: number, aim: number };

export const readInput = (): Observable<SubCommand> => readTestInputLines('day-02').pipe(
    map(value => {
        var [cmdName, scaleString] = value.split(" ", 2);
        var cmdType = cmdTypeForName(cmdName);
        return { type: cmdType, scale: parseInt(scaleString) || 0 };
    }));