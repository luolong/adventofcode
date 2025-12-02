package solution;

import static solution.Ansi.*;

import java.util.List;

/**
 * Solutions for Advent of Code 2025 Day 3
 */
public class Day03 implements Part1, Part2, Test {

    @Override
    public String part1(String input) {
        return String.valueOf(input.lines().mapToLong(this::maxJolatage).sum());
    }

    @Override
    public String part2(String input) {
        return String.valueOf(
            input.lines().mapToLong(this::twelveJolatage).sum()
        );
    }

    @Override
    public List<TestUnit> tests() {
        return List.of(
            test(
                "Part 1: Max joltage of 2",
                List.of(
                    ok("Max joltage of 987654321111111 is 98L", () -> assertMaxJoltage("987654321111111", 98L)),
                    ok("Max joltage of 811111111111119 is 89L", () -> assertMaxJoltage("811111111111119", 89L)),
                    ok("Max joltage of 891111111111111 is 91L", () -> assertMaxJoltage("891111111111111", 91L)),
                    ok("Max joltage of 234234234234278 is 78L", () -> assertMaxJoltage("234234234234278", 78L)),
                    ok("Max joltage of 818181911112111 is 92L", () -> assertMaxJoltage("818181911112111", 92L))
                )
            ),
            test(
                "Part 2: Max joltage of 12",
                List.of(
                    ok("Max joltage of 987654321111111 is 89L", () -> assertTwelveJoltage("987654321111111", 987654321111L)),
                    ok("Max joltage of 811111111111119 is 91L", () -> assertTwelveJoltage("811111111111119", 811111111119L)),
                    ok("Max joltage of 234234234234278 is 78L", () -> assertTwelveJoltage("234234234234278", 434234234278L)),
                    ok("Max joltage of 818181911112111 is 92L", () -> assertTwelveJoltage("818181911112111", 888911112111L))
                )
            )
        );
    }

    private long twelveJolatage(String bank) {
        return calculateJolatage(bank, 12);
    }

    private long maxJolatage(String bank) {
        return calculateJolatage(bank, 2);
        /*
        // prettier-ignore
        var digits = bank.chars().map(c -> c - (int) '0').toArray();

        int lastIndex = digits.length - 1;
        int tensIndex = 0;
        int tensDigit = digits[tensIndex];

        int onesIndex = tensIndex;
        int onesDigit = digits[onesIndex];

        for (int i = onesIndex + 1; i < digits.length; i++) {
            int d = digits[i];
            if (i < lastIndex && d > tensDigit) {
                tensDigit = d;
                tensIndex = i;
                continue;
            }
            if (tensIndex >= onesIndex || d > onesDigit) {
                onesDigit = d;
                onesIndex = i;
            }
        }

        return 10 * tensDigit + digits[onesIndex];
        */
    }

    private long calculateJolatage(String bank, int switches) {
        // prettier-ignore
        int[] batteries = bank.chars().map(c -> c - '0').toArray();

        int[] digits = new int[switches];
        int dc = 0; // digit cursor

        for (int i = 0; i < batteries.length; i++) {
            int b = batteries[i];
            while (
                dc > 0 &&
                i + switches - dc < batteries.length &&
                b > digits[dc - 1]
            ) {
                if (dc < switches) {
                    digits[dc] = 0;
                }
                dc -= 1;
            }

            if (dc < switches && b > digits[dc]) {
                digits[dc++] = b;
            }
        }

        // Calculate the Joltage
        long joltage = 0L;
        for (int i = 0; i < switches; i++) {
            joltage += digits[i] * Math.pow(10, switches - i - 1);
        }

        return joltage;
    }

    private void assertMaxJoltage(String bank, long expectedJoltage) {
        long actualJoltage = maxJolatage(bank);
        if (actualJoltage != expectedJoltage) {
            throw new AssertionError(
                String.format(
                    "In %s, you, the largest joltage possible should be %s, but we got %s",
                    dim(bank),
                    green(String.valueOf(expectedJoltage)),
                    red(String.valueOf(actualJoltage))
                )
            );
        }
    }

    private void assertTwelveJoltage(String bank, long expectedJoltage) {
        long actualJoltage = twelveJolatage(bank);
        if (actualJoltage != expectedJoltage) {
            throw new AssertionError(
                String.format(
                    "In %s, you, the largest joltage possible should be %s, but we got %s",
                    dim(bank),
                    green(String.valueOf(expectedJoltage)),
                    red(String.valueOf(actualJoltage))
                )
            );
        }
    }
}
