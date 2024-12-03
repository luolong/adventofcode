import { readLines } from "./util.ts";
import * as day1 from "./day01.ts";

let day = 0;
for (const { part1, part2 } of [day1]) {
  const dayStr = (++day).toString().padStart(2, "0");
  console.group("Day", dayStr);

  const lines = await readLines(`day${dayStr}.txt`);

  console.time("Part 1");
  console.log(part1(lines));
  console.timeEnd("Part 1");

  console.time("Part 2");
  console.log(part2(lines));
  console.timeEnd("Part 2");
  console.groupEnd();
}
