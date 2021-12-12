import { count, reduce } from "rxjs";
import { CmdType, Coord, AimCoord, readInput } from "./day02";

test('readInput emits n items', done => {
    readInput().pipe(count()).subscribe(n => {
        expect(n).toBe(1000);
        done();
    });
});

test('part 1 solution', done => {
    var initial_loc: Coord = { x: 0, y: 0 };
    readInput().pipe(
        reduce((coord, { type, scale }): Coord => {
            if (type !== undefined) {
                switch (+type) {
                    case CmdType.Up:
                        return { x: coord.x, y: coord.y - scale };
                        break;
                    case CmdType.Down:
                        return { x: coord.x, y: coord.y + scale };
                        break;
                    case CmdType.Forward:
                        return { x: coord.x + scale, y: coord.y };
                        break;
                }
            }
            return coord;
        }, initial_loc)
    ).subscribe(final_loc => {
        expect(final_loc.x * final_loc.y).toBe(1480518);
        done();
    });
});

test('part 2 solution', done => {
    var initial_loc: AimCoord = { x: 0, y: 0, aim: 0 };
    readInput().pipe(
        reduce((coord, { type, scale }): AimCoord => {
            if (type !== undefined) {
                switch (+type) {
                    case CmdType.Up:
                        return { x: coord.x, y: coord.y, aim: coord.aim - scale };
                        break;
                    case CmdType.Down:
                        return { x: coord.x, y: coord.y, aim: coord.aim + scale };
                        break;
                    case CmdType.Forward:
                        return { x: coord.x + scale, y: coord.y + (coord.aim * scale), aim: coord.aim };
                        break;
                }
            }
            return coord;
        }, initial_loc)
    ).subscribe(final_loc => {
        expect(final_loc.x * final_loc.y).toBe(1282809906);
        done();
    });
});