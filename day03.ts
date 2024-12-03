import { readLines } from "./util.ts";

export function part1(input: string[]) {
  const re = /mul\((\d{1,3}),(\d{1,3})\)/g;

  return input
    .join()
    .matchAll(re)
    .map(([_, a, b]) => parseInt(a) * parseInt(b))
    .reduce((sum, current) => sum + current, 0);
}

export function part2(input: string[]) {
  const re = /(mul|do|don't)\((?:(\d{1,3}),(\d{1,3}))?\)/g;
  return input
    .join()
    .matchAll(re)
    .toArray()
    .map(([_, cmd, a, b]) => [cmd, a, b])
    .reduce(
      ({ enabled, sum }, [cmd, a, b]) => {
        switch (cmd) {
          case "do":
            return { enabled: true, sum };
          case "don't":
            return { enabled: false, sum };

          case "mul":
            if (enabled) {
              return { enabled, sum: sum + parseInt(a) * parseInt(b) };
            }
          /* falls through */
          default:
            return { enabled, sum };
        }
      },
      { enabled: true, sum: 0 },
    ).sum;
}

if (import.meta.main) {
  const lines = await readLines("day03.txt");
  console.log("Day 3, Part 1:", part1(lines));
  console.log("Day 3, Part 2:", part2(lines));
}
