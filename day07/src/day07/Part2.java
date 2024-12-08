package day07;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.Scanner;
import java.util.function.Consumer;
import java.util.function.LongConsumer;
import java.util.function.LongSupplier;

public class Part2 implements Consumer<String>, LongSupplier {
    private static long[] POWER_OF_10 = {
            1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
            10000000000L, 100000000000L, 1000000000000L, 10000000000000L, 100000000000000L,
            1000000000000000L, 10000000000000000L, 100000000000000000L, 1000000000000000000L
    };

    record Result(long value/*, String string*/) {
        public static Result of(long value) {
            return new Result(value/*, String.valueOf(value)*/);
        }

        public Result plus(long value) {
            return new Result(this.value + value);
        }

        public Result times(long value) {
            return new Result(this.value * value);
        }

        public Result concat(long value) {
            if (this.value == 0) {
                return new Result(value);
            }
            if (value == 0) {
                return new Result(this.value * 10);
            }

            long magnitude = (long) (StrictMath.log10(value) + 1);
            long multiplier = POWER_OF_10[(int) magnitude];
            return new Result(this.value * multiplier + value);
        }
    }

    record Equation(long testValue, List<Long> numbers) {
        @SuppressWarnings("DuplicatedCode")
        public static Equation parse(String input) {
            try (var scanner = new Scanner(input)) {
                scanner.useDelimiter(": ");
                var testValue = scanner.nextLong();
                scanner.skip(": ").useDelimiter(" ");
                var numbers = new ArrayList<Long>();
                while (scanner.hasNextLong()) {
                    numbers.add(scanner.nextLong());
                }
                return new Equation(testValue, List.copyOf(numbers));
            }
        }

        public void ifSolvable(LongConsumer doIfSolvable) {
            if (isSolvable(Result.of(numbers.getFirst()), numbers.subList(1, numbers.size()))) {
                doIfSolvable.accept(testValue);
            }
        }

        private boolean isSolvable(Result result, List<Long> remaining) {
            if (remaining.isEmpty()) {
                if (Objects.equals(result.value, testValue)) {
                    //System.out.print("\"" + result + "\", ");
                    return true;
                }

                return false;
            }

            var next = remaining.getFirst();
            return isSolvable(result.plus(next), remaining.subList(1, remaining.size()))
                    || isSolvable(result.times(next), remaining.subList(1, remaining.size()))
                    || isSolvable(result.concat(next), remaining.subList(1, remaining.size()));
        }
    }

    @Override
    public void accept(String s) {
        Equation.parse(s).ifSolvable(testValue -> validResult += testValue);
        //System.out.printf("%d", validResult);
    }

    long validResult = 0;

    @Override
    public long getAsLong() {
        return validResult;
    }

    @Override
    public String toString() {
        return String.valueOf(getAsLong());
    }
}
