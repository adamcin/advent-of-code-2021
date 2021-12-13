import { count, firstValueFrom, reduce } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { CmdType, Coord, AimCoord, readInput } from "./day02";

const testScheduler = new TestScheduler((actual, expected) => {
    // asserting the two objects are equal - required
    // for TestScheduler assertions to work via your test framework
    // e.g. using chai.
    expect(actual).toEqual(expected);
});

test('readInput emits n items', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(count()));
    }).then(n => expect(n).toBe(1000));
});

test('part 1 solution', () => {
    return testScheduler.run(async () => {
        var initial_loc: Coord = { x: 0, y: 0 };
        return firstValueFrom(readInput().pipe(
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
        ));
    }).then(final_loc => expect(final_loc.x * final_loc.y).toBe(1480518));
});

test('part 2 solution', () => {
    return testScheduler.run(async () => {
        var initial_loc: AimCoord = { x: 0, y: 0, aim: 0 };
        return firstValueFrom(readInput().pipe(
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
        ));
    }).then(final_loc => expect(final_loc.x * final_loc.y).toBe(1282809906));
});