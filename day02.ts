import { readLines } from "./util.ts";

export function part1(reports: string[]) {
  let safeReports = 0;
  for (const report of reports) {
    const levels = report.split(" ").map((l) => parseInt(l));
    if (isSafe(levels)) {
      safeReports += 1;
    }
  }
  return safeReports;
}

export function part2(reports: string[]) {
  let safeReports = 0;
  for (const report of reports) {
    const levels = report.split(" ").map((l) => parseInt(l));

    if (!isSafe(levels)) {
      dampener: for (let i = 0; i < levels.length; i++) {
        if (isSafe(levels.toSpliced(i, 1))) {
          safeReports += 1;
          break dampener;
        }
      }
      continue;
    }

    safeReports += 1;
  }
  return safeReports;
}

function isSafe(levels: number[]): boolean {
  const diffs = Array.from(window(levels, 2)).map(([a, b]) => b - a);
  return (
    (diffs.every((diff) => diff > 0) || diffs.every((diff) => diff < 0)) &&
    diffs.map((diff) => Math.abs(diff)).every((d) => 1 <= d && d <= 3)
  );
}

function* window(inputArray: any[], size: number) {
  for (let index = 0; index + size <= inputArray.length; index++) {
    yield inputArray.slice(index, index + size);
  }
}

if (import.meta.main) {
  const lines = await readLines("day02.txt");
  console.log("Day 2, Part 1:", part1(lines));
  console.log("Day 2, Part 2:", part2(lines));
}
