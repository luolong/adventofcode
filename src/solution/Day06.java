package solution;

import static java.util.stream.Collectors.joining;

import module java.base;

/**
 * Solutions for Advent of Code 2025 Day 6
 */
public class Day06 implements Part1, Part2 {

    public String part1(String input) {
        var lines = input.split("\n");
        var ops = lines[lines.length - 1].trim().split("\s+");

        var results = new long[ops.length];
        var fn = new LongBinaryOperator[ops.length];
        for (int i = 0; i < ops.length; i++) {
            var op = ops[i].trim().charAt(0);
			results[i] = unitValue(op);
			fn[i] = function(op);
        }

        for (int i = 0; i < lines.length - 1; i++) {
            var values = lines[i].trim().split("\s+");
            for (int j = 0; j < ops.length; j++) {
                results[j] = fn[j].applyAsLong(
                    results[j],
                    Long.parseLong(values[j])
                );
            }
        }

        return String.valueOf(LongStream.of(results).sum());
    }

	private LongBinaryOperator function(char op) {
		return switch (op) {
		    case '+' -> (a, b) -> a + b;
		    case '*' -> (a, b) -> a * b;
		    default -> throw new IllegalArgumentException(
		        "Invalid operation: '" + op + "'"
		    );
		};
	}

	private long unitValue(char op) {
		return switch (op) {
		    case '+' -> 0L;
		    case '*' -> 1L;
		    default -> throw new IllegalArgumentException(
		        "Invalid operation: '" + op + "'"
		    );
		};
	}

    @Override
    public String part2(String input) {
        var lines = input.split("\n");
        var lastLine = lines.length - 1;

        var operatorsLine = lines[lastLine];
        var columnIndexes = new ArrayList<Integer>();
        for (int i = 0; i < operatorsLine.length(); i++) {
            var c = operatorsLine.charAt(i);
            switch (c) {
                case '+', '*':
                    columnIndexes.add(i);
            }
        }

        // Iterate problems from right to left
        int previousStartIndex = Arrays.stream(lines).mapToInt(String::length).max().orElseThrow();
        long totalSum = 0;
        for (int problemStartIndex: columnIndexes.reversed()) {
            var op = operatorsLine.charAt(problemStartIndex);
            var result = unitValue(op);
            var fn = function(op);
            for (int c = previousStartIndex - 1; c >= problemStartIndex; c--) {
                var value = 0;
                int digits = 0;
                for (int l = 0; l < lastLine; l++) {
                    var digit = lines[l].charAt(c);
                    if (Character.isDigit(digit)) {
                        value = value * 10 + Character.getNumericValue(digit);
                        digits++;
                    }
                }

                if (digits > 0) {
                    result = fn.applyAsLong(result, value);
                }
            }
            totalSum += result;
            previousStartIndex = problemStartIndex;
        }

        return String.valueOf(totalSum);
    }

}
