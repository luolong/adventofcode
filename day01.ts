import { readLines } from "./util.ts";

export function part1(lines: string[]) {
  const left: number[] = [];
  const right: number[] = [];

  for (const line of lines) {
    const [a, b] = line.split(/ +/, 2);
    left.push(parseInt(a));
    right.push(parseInt(b));
  }

  left.sort();
  right.sort();

  let totalDistance = 0;
  for (const [a, b] of zip(left, right)) {
    totalDistance += Math.abs(a - b);
  }

  return totalDistance;
}

export function part2(lines: string[]) {
  const left: number[] = [];
  const right: Record<number, number> = {};

  for (const line of lines) {
    const [a, b] = line.split(/ +/, 2);
    left.push(parseInt(a));
    const num = parseInt(b);
    right[num] = (right[num] ?? 0) + 1;
  }

  let totalDistance = 0;
  for (const a of left) {
    totalDistance += a * (right[a] ?? 0);
  }
  return totalDistance;
}

function* zip(a: number[], b: number[]) {
  const length = Math.min(a.length, b.length);
  for (let i = 0; i < length; i++) {
    yield [a[i], b[i]];
  }
}

if (import.meta.main) {
  const lines = await readLines("day01.txt");
  console.log("Day 1, Part 1:", part1(lines));
  console.log("Day 1, Part 2:", part2(lines));
}
