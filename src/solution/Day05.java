package solution;

import module java.base;

/**
 * Solutions for Advent of Code 2025 Day 5
 */
public class Day05 implements Part1, Part2, Test {

    record Range(long start, long end) {
        Range {
            if (start > end) {
                throw new IllegalArgumentException("Invalid range: " + start + " > " + end);
            }
        }

        public static Range parse(String input) {
            var parts = input.split("-");
            return new Range(Long.parseUnsignedLong(parts[0]), Long.parseUnsignedLong(parts[1]));
        }

        public boolean overlaps(Range other) {
            return start <= other.end && end >= other.start;
        }

        public Range merge(Range other) {
            return new Range(Math.min(start, other.start), Math.max(end, other.end));
        }

        public long size() {
            return end - start + 1;
        }

        public String toString() {
            return start + "-" + end;
        }
    }

    private static class Database {
        // Sorted range list: An ordered list of non-overlapping range endpoints
        // Each even index (0, 2, 4, ...) represents the start of a range
        // Each odd index (1, 3, 5, ...) represents the end of a range
        // Adding a range to the database will merge it with existing overlapping range.
        // Adjacent ranges continuous ranges will be merged on insertion.
        private final List<Long> ranges = new ArrayList<>();

        public Range add(Range range) {
            if (range instanceof Range(long start, long end)) {
                if (ranges.isEmpty()) {
                    ranges.addAll(List.of(start, end));
                    return range;
                } else {
                    int startIndex = Collections.binarySearch(ranges, start, Long::compare);
                    if (startIndex < 0) {
                        // no range neither starts nor ends with given range start
                        startIndex = -startIndex - 1;
                        if (startIndex % 2 == 0 && startIndex > 0) {
                            int prevIndex = startIndex - 1;
                            long prev = ranges.get(prevIndex);
                            if (start == prev + 1) {
                                // new range start is immediately adjacent to previous range end
                                startIndex -= 2;
                                start = ranges.get(startIndex);
                            }
                        }
                    } else if (startIndex % 2 == 1) {
                        // given range starts at an existing range endpoint
                        startIndex -= 1;
                        start = ranges.get(startIndex);
                    }

                    int endIndex = Collections.binarySearch(ranges, end, Long::compare);
                    if (endIndex < 0) {
                        // no range neither starts nor ends with given range end
                        endIndex = -endIndex - 1;
                        if (endIndex % 2 == 0 && endIndex + 1 < ranges.size()) {
                            long next = ranges.get(endIndex);
                            if (end == next) {
                                // given range ends at an existing range endpoint
                                endIndex += 1;
                                end = next;
                            } else if (end + 1 == next) {
                                // given range ends immediately adjacent to an existing range start
                                endIndex += 1;
                                end = ranges.get(endIndex);
                            }
                        }
                    } else if (endIndex % 2 == 0) {
                        endIndex += 1;
                        end = ranges.get(endIndex);
                    }

                    if (startIndex == endIndex) {
                        if (startIndex % 2 == 0) {
                            ranges.addAll(startIndex, List.of(start, end));
                            return new Range(start, end);
                        } else {
                            start = ranges.get(startIndex - 1);
                            end = ranges.get(endIndex);
                            return new Range(start, end);
                        }
                    }

                    if (startIndex % 2 == 1) {
                        startIndex -= 1;
                        start = ranges.get(startIndex);
                    }

                    if (endIndex % 2 == 1) {
                        end = ranges.get(endIndex);
                    } else if (startIndex < endIndex) {
                        endIndex -= 1;
                    }

                    var segment = ranges.subList(startIndex, endIndex + 1);
                    segment.clear();
                    segment.addAll(List.of(start, end));

                    return new Range(start, end);
                }
            }

            return range;
        }

        public Optional<Range> findRange(long id) {
            if (ranges.isEmpty() || ranges.getFirst() > id || ranges.getLast() < id) {
                return Optional.empty();
            }

            int index = Collections.binarySearch(ranges, id, Long::compare);
            // We matched one of the range endpoints, so lets construct a matching range
            if (index >= 0) {
                if (index % 2 == 0) {
                    // even index means id equals range start ({0}..1, {2}..3, ...)
                    return Optional.of(new Range(ranges.get(index), ranges.get(index + 1)));
                }

                // odd index means id equals range start (0..{1}, 2..{3}, ...)
                return Optional.of(new Range(ranges.get(index - 1), ranges.get(index)));
            }

            // negative index means, id is "between" range endpoints
            int insertionPoint = -index - 1;
            if (insertionPoint % 2 == 1) {
                // Odd insertion point means id is between range start and end (0{..}1, 2{..}3, ...)
                return Optional.of(new Range(ranges.get(insertionPoint-1), ranges.get(insertionPoint)));
            }

            return Optional.empty();
        }

        public int size() {
            return ranges.size() / 2;
        }

        public boolean isFresh(long id) {
            return findRange(id).isPresent();
        }

        public Stream<Range> ranges() {
            return ranges.stream()
                .gather(Gatherers.windowFixed(2))
                .map(pair -> new Range(pair.get(0), pair.get(1)));
        }

        @Override
        public String toString() {
            String out = "[";
            for (int i = 0; i + 1 < ranges.size(); i += 2) {
                out += ranges.get(i) + "-" + ranges.get(i + 1);
                if (i + 2 < ranges.size()) {
                    out += ", ";
                }
            }
            return out + "]";
        }
    }

    @Override
    public String part1(String input) {
        var it = input.lines().toList().listIterator();
        var database = new Database();
        while (it.hasNext()) {
            var line = it.next();
            if (line.isEmpty()) {
                break;
            }
            database.add(Range.parse(line));
        }

        int fresh = 0;
        while (it.hasNext()) {
            var id = Long.parseLong(it.next());
            if (database.isFresh(id)) {
                fresh++;
            }
        }

        return String.valueOf(fresh);
    }

    @Override
    public String part2(String input) {
        var it = input.lines().toList().listIterator();
        var database = new Database();
        while (it.hasNext()) {
            var line = it.next();
            if (line.isEmpty()) {
                break;
            }
            database.add(Range.parse(line));
        }

        return String.valueOf(database.ranges()
            .mapToLong(Range::size)
            .sum());
    }

    @Override
    public List<TestUnit> tests() {
        var db1 = new Database();
        var db2 = new Database();
        return List.of(
            test("Range operations", List.of(
                test("simple range operations", List.of(
                    ok("[]             + [30-40] -> [30-40]", () -> assertRangeAdd(db1, new Range(30, 40), new Range(30, 40), 1)),
                    ok("[30-40]        + [10-20] -> [10-20, 30-40]", () -> assertRangeAdd(db1, new Range(10, 20), new Range(10, 20), 2)),
                    ok("[10-20, 30-40] + [50-60] -> [10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(50, 60), new Range(50, 60), 3))
                )),
                test("adding fully enclosed ranges", List.of(
                    ok("[10-20, 30-40, 50-60] + [30-35] -> [10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(30, 35), new Range(30, 40), 3)),
                    ok("[10-20, 30-40, 50-60] + [35-35] -> [10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(35, 35), new Range(30, 40), 3)),
                    ok("[10-20, 30-40, 50-60] + [35-40] -> [10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(35, 40), new Range(30, 40), 3)),
                    ok("[10-20, 30-40, 50-60] + [33-37] -> [10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(33, 37), new Range(30, 40), 3)),
                    ok("[10-20, 30-40, 50-60] + [40-40] -> [10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(40, 40), new Range(30, 40), 3)),
                    ok("[10-20, 30-40, 50-60] + [30-40] -> [10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(30, 40), new Range(30, 40), 3))
                )),
                test("adding partially overlapping ranges", List.of(
                    ok("[10-20, 30-40, 50-60]      + [5-5]  -> [5-5, 10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(5, 5), new Range(5, 5), 4)),
                    ok("[5-5, 10-20, 30-40, 50-60] + [3-5]  -> [3-5, 10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(3, 5), new Range(3, 5), 4)),
                    ok("[3-5, 10-20, 30-40, 50-60] + [5-7]  -> [3-7, 10-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(5, 7), new Range(3, 7), 4)),
                    ok("[3-7, 10-20, 30-40, 50-60] + [7-10] -> [3-20, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(7, 10), new Range(3, 20), 3))
                )),
                test("adding adjacent ranges", List.of(
                    ok("[3-20, 30-40, 50-60]        + [25-25] -> [3-20, 25-25, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(25, 25), new Range(25, 25), 4)),
                    ok("[3-20, 25-25, 30-40, 50-60] + [24-24] -> [3-20, 24-25, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(24, 24), new Range(24, 25), 4)),
                    ok("[3-20, 24-25, 30-40, 50-60] + [26-26] -> [3-20, 24-26, 30-40, 50-60]", () -> assertRangeAdd(db1, new Range(26, 26), new Range(24, 26), 4)),
                    ok("[3-20, 24-26, 30-40, 50-60] + [27-29] -> [3-20, 24-40, 50-60]",        () -> assertRangeAdd(db1, new Range(27, 29), new Range(24, 40), 3)),
                    ok("[3-20, 24-40, 50-60]        + [1-2] -> [1-20, 24-40, 50-60]",          () -> assertRangeAdd(db1, new Range(1, 2), new Range(1, 20), 3)),
                    ok("[1-20, 24-40, 50-60]        + [61-80] -> [1-20, 24-40, 50-80]",        () -> assertRangeAdd(db1, new Range(61, 80), new Range(50, 80), 3))
                ))
            )),
            test("Part 1", List.of(
                test("Build fresh ingredient ranges", List.of(
                    ok("[3-5]", () -> assertRangeAdd(db2, new Range(3, 5), new Range(3, 5), 1)),
                    ok("[10-14]", () -> assertRangeAdd(db2, new Range(10, 14), new Range(10, 14), 2)),
                    ok("[16-20]", () -> assertRangeAdd(db2, new Range(16, 20), new Range(16, 20), 3)),
                    ok("[12-18]", () -> assertRangeAdd(db2, new Range(12, 18), new Range(10, 20), 2))
                )),
                test("Fresh or Spoiled", List.of(
                    ok("1 is spoiled", () -> assertIsSpoiledIngredient(db2, 1)),
                    ok("5 is fresh", () -> assertIsFreshIngredient(db2, 5, "3-5")),
                    ok("8 is spoiled", () -> assertIsSpoiledIngredient(db2, 8)),
                    ok("11 is fresh", () -> assertIsFreshIngredient(db2, 11, "10-14")),
                    ok("17 is fresh", () -> assertIsFreshIngredient(db2, 17, "16-20")),
                    ok("32 is spoiled", () -> assertIsSpoiledIngredient(db2, 32))
                ))
            ))
        );
    }

    /*
    @Override
    public void test() throws AssertionError {


        db = new Database();
        assertRangeAdd(db, new Range(3, 5), new Range(3, 5), 1);
        assertRangeAdd(db, new Range(10, 14), new Range(10, 14), 2);
        assertRangeAdd(db, new Range(16, 20), new Range(16, 20), 3);
        assertRangeAdd(db, new Range(12, 18), new Range(10, 20), 2);

        assertIsSpoiledIngredient(db, 1);
        assertIsFreshIngredient(db, 5, "3-5");
        assertIsSpoiledIngredient(db, 8);
        assertIsFreshIngredient(db, 11, "10-14");
        assertIsFreshIngredient(db, 17, "16-20");
        assertIsSpoiledIngredient(db, 32);
    }
    */

    private void assertRangeAdd(Database db, Range range, Range expected, int expectedSize) {
        assertEquals(expected, db.add(range), (e, a) -> "Expected db.add(" + range + ") to return " + e + ", but got " + a);
        assertEquals(expectedSize, db.size(), (e, a) -> "Expected db.size() after db.add(" + range + ") to be " + e + ", but got " + a);
    }

    private void assertIsSpoiledIngredient(Database db, long id) {
        if (db.isFresh(id)) {
            throw new AssertionError("Expected ingredient ID " + id + " is spoiled because it does not fall into any range, but it was found in the database.");
        }
    }

    private void assertIsFreshIngredient(Database db, long id, String expectedRange) {
        if (!db.isFresh(id)) {
            throw new AssertionError("Expected ingredient ID " + id + " to be fresh because it falls into range " + expectedRange + ", but it was not found in the database.");
        }
    }
}
