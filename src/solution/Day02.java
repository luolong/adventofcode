package solution;

import module java.base;

import static java.util.Comparator.comparingLong;
import static java.util.stream.Collectors.joining;
import static solution.Ansi.*;

/**
 * Solutions for Advent of Code 2025 Day 2
 */
public class Day02 implements Part1, Part2, Test {
    private static final String[] DIGITS = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

    private static boolean DEBUG = false;

    value record Range(long start, long end) {
        public static Range parse(String s) {
            String[] parts = s.split("-", 2);
            return new Range(Long.parseLong(parts[0].trim()), Long.parseLong(parts[1].trim()));
        }

        public Range merge(Range other) {
            return new Range(Math.min(start, other.start), Math.max(end, other.end));
        }

        @Override
        public String toString() {
            return start + "-" + end;
        }
    }

    @Override
    public String part1(String input) {
        var result = parseRanges(input.trim()).stream()
            .flatMapToLong(range -> generateDoubledIds(range.start, range.end))
            .sum();

        return String.valueOf(result);
    }

    @Override
    public String part2(String input) {
        var result = parseRanges(input.trim()).stream()
            .flatMapToLong(range -> generateAllInvalidIds(range.start, range.end))
            .sum();

        return String.valueOf(result);
    }

    private LongStream generateDoubledIds(long start, long end) {
        return generateInvalidIdsBetween(start, end, 2).stream().mapToLong(Long::valueOf);
    }

    private LongStream generateAllInvalidIds(long start, long end) {
        var maxlen = (long)Math.log10(end) + 1;
        debug("generateAllInvalidIds between " + start + " and " + end + ":");
        debug(dim("  maxlen=" + maxlen));
        return LongStream.iterate(2, m -> m <= maxlen, m -> m + 1)
            .mapToObj(m -> generateInvalidIdsBetween(start, end, (int)m))
            .takeWhile(ids -> ids.isEmpty() || ids.getFirst() <= end)
            .flatMapToLong(ids -> ids.stream().mapToLong(Long::valueOf))
            .sorted()
            .distinct();
    }

    /**
     * Generate IDs that are patterns of m repetitions of same numerical sequence.
     */
    private List<Long> generateInvalidIdsBetween(long start, long end, int m) {
        debug(dim("  > Generating repeating patterns of size " + m + " between " + start + " and " + end));
        int kmax = (int) (Math.log10(end) + 1);

        var invalidIds = new ArrayList<Long>();
        for (int k = 1; k <= kmax; k++) {
            long c = geometricSum(m, k);
            //debug(dim("   > k=" + k + "; kmax=" + kmax + "; c=" + c));

            long xstart = Math.max(
                start / c,
                (long)(Math.pow(10, k-1))
            );

            long xend = Math.min(
                (long)(end / c),
                (long)(Math.pow(10, k) - 1)
            );

            for (long x = xstart; x <= xend; x++) {
                long n = x * c;
                //debug(dim("    > x=" + x + "; xstart=" + xstart + "; xend=" + xend + "; n=" + n));
                if (start <= n && n <= end) {
                    debug(bold("   >>> " + underline(String.valueOf(n))));
                    invalidIds.add(n);
                }
            }
        }
        return invalidIds;
    }

    private long geometricSum(int m, int k) {
        return LongStream.range(0, m).map(i -> (long) Math.pow(10, i * k)).sum();
    }

	@Override
    public List<TestUnit> tests() throws AssertionError {
        return List.of(
            test("Part 1 - Two repetitions", List.of(
                ok("Range 11-22 contasins two invalid ids: 11 and 22", () -> assertInvalidIds(this::generateDoubledIds, "11-22", List.of(11L, 22L))),
                ok("Range 95-115 contains one invalid id: 99", () -> assertInvalidIds(this::generateDoubledIds, "95-115", List.of(99L))),
                ok("Range 998-1012 contains one invalid id: 1010", () -> assertInvalidIds(this::generateDoubledIds, "998-1012", List.of(1010L))),
                ok("Range 1188511880-1188511890 contains one invalid id: 1188511885", () -> assertInvalidIds(this::generateDoubledIds, "1188511880-1188511890", List.of(1188511885L))),
                ok("Range 222220-222224 contains one invalid id: 222222", () -> assertInvalidIds(this::generateDoubledIds, "222220-222224", List.of(222222L))),
                ok("Range 1698522-1698528 contains no invalid ids", () -> assertInvalidIds(this::generateDoubledIds, "1698522-1698528", List.of())),
                ok("Range 446443-446449 contains one invalid id: 446446", () -> assertInvalidIds(this::generateDoubledIds, "446443-446449", List.of(446446L))),
                ok("Range 38593856-38593862 contains one invalid id: 38593859", () -> assertInvalidIds(this::generateDoubledIds, "38593856-38593862", List.of(38593859L))),
                ok("Range 565653-565659 contains one invalid id: 565656", () -> assertInvalidIds(this::generateDoubledIds, "565653-565659", List.of(565656L))),
                ok("Range 824824821-824824827 contains one invalid id: 824824824", () -> assertInvalidIds(this::generateDoubledIds, "824824821-824824827", List.of(824824824L))),
                ok("Range 2121212118-2121212124 contains one invalid id: 2121212121", () -> assertInvalidIds(this::generateDoubledIds, "2121212118-2121212124", List.of(2121212121L)))
            )),
            test("Part 2 - Any number of repetitions", List.of(
                ok("Range 11-22 contains two invalid ids: 11 and 22", () -> assertInvalidIds(this::generateAllInvalidIds, "11-22", List.of(11L, 22L))),
                ok("Range 95-115 contains two invalid ids: 99 and 111", () -> assertInvalidIds(this::generateAllInvalidIds, "95-115", List.of(99L, 111L))),
                ok("Range 998-1012 contains two invalid ids: 999 and 1010", () -> assertInvalidIds(this::generateAllInvalidIds, "998-1012", List.of(999L, 1010L))),
                ok("Range 1188511880-1188511890 contains one invalid id: 1188511885", () -> assertInvalidIds(this::generateAllInvalidIds, "1188511880-1188511890", List.of(1188511885L))),
                ok("Range 222220-222224 contains one invalid id: 222222", () -> assertInvalidIds(this::generateAllInvalidIds, "222220-222224", List.of(222222L))),
                ok("Range 1698522-1698528 contains no invalid ids", () -> assertInvalidIds(this::generateAllInvalidIds, "1698522-1698528", List.of())),
                ok("Range 446443-446449 contains one invalid id: 446446", () -> assertInvalidIds(this::generateAllInvalidIds, "446443-446449", List.of(446446L))),
                ok("Range 38593856-38593862 contains one invalid id: 38593859", () -> assertInvalidIds(this::generateAllInvalidIds, "38593856-38593862", List.of(38593859L))),
                ok("Range 565653-565659 contains one invalid id: 565656", () -> assertInvalidIds(this::generateAllInvalidIds, "565653-565659", List.of(565656L))),
                ok("Range 824824821-824824827 contains one invalid id: 824824824", () -> assertInvalidIds(this::generateAllInvalidIds, "824824821-824824827", List.of(824824824L))),
                ok("Range 2121212118-2121212124 contains one invalid id: 2121212121", () -> assertInvalidIds(this::generateAllInvalidIds, "2121212118-2121212124", List.of(2121212121L)))
            ))
        );
    }

    private void assertInvalidIds(BiFunction<Long, Long, LongStream> generator, String input, List<Long> expected) {
        var range = Range.parse(input);
        var invalidIds = generator.apply(range.start, range.end).boxed().toList();
        if (expected.isEmpty() && !invalidIds.isEmpty()) {
            throw new AssertionError(expectationMessage(input, expected) + actualMessage(invalidIds));
        } else if (invalidIds.isEmpty()){
            return;
        }

        if (!expected.isEmpty() && invalidIds.isEmpty()) {
            throw new AssertionError(expectationMessage(input, expected) + actualMessage(invalidIds));
        }
        if (!expected.equals(invalidIds)) {
            throw new AssertionError("Expected invalid IDs " + expected + " but found " + invalidIds);
        }
    }

    private String actualMessage(List<Long> invalidIds) {
        return switch (invalidIds.size()) {
            case 0 -> ", but found none!";
            case 1 -> "but found " + strikethrough(invalidIds.getFirst());
            default -> {
                var head = invalidIds.subList(0, invalidIds.size() - 1)
                    .stream()
                    .map(String::valueOf)
                    .map(Ansi::strikethrough)
                    .collect(joining(", "));
                var tail = strikethrough(invalidIds.getLast());
				yield "but found " + head + " and " + tail;
            }
        };
    }

    private String expectationMessage(String range, List<Long> expected) {
        return switch (expected.size()) {
            case 0 -> "Expected range " + dim(underline(range)) + " to have no invalid IDs";
            case 1 -> "Expected range " + dim(underline(range)) + " to have one invalid ID, " + bold(expected.getFirst());
            default ->  {
                String count = "" + (expected.size() < 10 ? DIGITS[expected.size()] : expected.size());
                String head = expected.subList(0, expected.size() - 1).stream()
                    .map(String::valueOf)
                    .map(Ansi::bold)
                    .collect(joining(", "));
                String tail = bold(expected.getLast());
				yield "Expected range " + dim(underline(range)) + " to have " + count + " invalid IDs, " + head + " and " + tail;
            }
        };
    }

    void debug(String message) {
        if (DEBUG) {
            System.out.println("🪳 " + message);
        }
    }

	private List<Range> parseRanges(String input) {
        return Arrays.stream(input.split(","))
            .map(Range::parse)
            .sorted(comparingLong(Range::start)
                .thenComparingLong(Range::end))
            .toList();
    }

}
