package day07;

import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class Part1 implements Consumer<String>, Supplier<Long> {

    record Equation(Long testValue, List<Long> numbers) {
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

        public void ifSolvable(Consumer<Long> doIfSolvable) {
            if (isSolvable(testValue, numbers.getFirst(), numbers.subList(1, numbers.size()))) {
                //System.out.print("true, ");
                doIfSolvable.accept(testValue);
            //} else {
            //    System.out.print("false, ");
            }
        }

        private boolean isSolvable(long testValue, long head, List<Long> remaining) {
            if (remaining.isEmpty()) {
                return head == testValue;
            }

            var next = remaining.getFirst();
            return isSolvable(testValue, head + next, remaining.subList(1, remaining.size()))
                || isSolvable(testValue, head * next, remaining.subList(1, remaining.size()));
        }
    }

    private Long validResult = 0L;

    @Override
    public void accept(String s) {
        Equation.parse(s).ifSolvable(testValue -> validResult += testValue);
        //System.out.printf("%d, ", validResult);
    }


    @Override
    public Long get() {
        return validResult;
    }

    @Override
    public String toString() {
        return String.valueOf(get());
    }
}
