import { bufferCount, count, firstValueFrom } from "rxjs";
import { TestScheduler } from "rxjs/testing";
import { alignTo, readInput } from "./day07";

const testScheduler = new TestScheduler((actual, expected) => {
    expect(actual).toEqual(expected);
});

test('readInput emits n items', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(count()));
    }).then(n => expect(n).toBe(1000));
});

test('part 1 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(bufferCount(1000))).then(crabs => {
            let costs = crabs.map(pos => {
                return { pos: pos, cost: alignTo(crabs, pos, cost => cost) };
            });
            let { cost: initCost } = costs[0];
            return costs.reduce((accCost, { pos, cost }) => {
                if (cost < accCost) {
                    return cost;
                } else {
                    return accCost;
                }
            }, initCost);
        });
    }).then(n => expect(n).toBe(355150));
});

test('part 2 solution', () => {
    return testScheduler.run(async () => {
        return firstValueFrom(readInput().pipe(bufferCount(1000))).then(crabs => {
            let costs = crabs.map(pos => {
                return {
                    pos: pos, cost: alignTo(crabs, pos, diff => {
                        var cost = 0;
                        for (var d = 0; d <= diff; d++) {
                            cost += d;
                        }
                        return cost;
                    })
                };
            });
            let { cost: initCost } = costs[0];
            return costs.reduce((accCost, { cost }) => {
                if (cost < accCost) {
                    return cost;
                } else {
                    return accCost;
                }
            }, initCost);
        });
    }).then(n => expect(n).toBe(98368490));
});
