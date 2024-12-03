import { readLines } from "./util.ts";
import * as day1 from "./day01.ts";
import * as day2 from "./day02.ts";
import * as day3 from "./day03.ts";

let day = 0;
for (const { part1, part2 } of [day1, day2, day3]) {
  const dayStr = (++day).toString().padStart(2, "0");
  console.group("Day", dayStr);

  const inputFile = `day${dayStr}.txt`;
  console.time(inputFile);
  const lines = await readLines(inputFile);
  console.timeEnd(inputFile);

  console.time("Part 1");
  console.log(part1(lines));
  console.timeEnd("Part 1");

  console.time("Part 2");
  console.log(part2(lines));
  console.timeEnd("Part 2");
  console.groupEnd();
}
