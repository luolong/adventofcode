package solution;

import module java.base;

import static solution.Ansi.*;

/**
 * Solutions for Advent of Code 2025 Day 4
 */
public class Day04 implements Part1, Part2, Test {

    value record Matrix(int rows, int columns, int[] data) {
        Matrix {
            Objects.requireNonNull(data);
            if (rows * columns != data.length) {
                throw new IllegalArgumentException("Invalid dimensions");
            }
        }

        Matrix(int rows, int columns) {
            this(rows, columns, new int[rows * columns]);
        }

        public static Matrix parse(String input) {
            int lines = (int) input.lines().count();
            int columns = input.indexOf('\n');
            var data = input.lines()
                 .flatMapToInt(line -> line.chars())
                 .map(ch -> switch (ch) {
                     case '.' -> 0;
                     case '@' -> 1;
                     default -> throw new IllegalArgumentException("Invalid character: '" + (char) ch + "'");
                 })
                 .toArray();

            return new Matrix(lines, columns, data);
        }

        public int get(int row, int col) {
            Objects.checkIndex(row, rows);
            Objects.checkIndex(col, columns);
            return data[row * columns + col];
        }

        public int set(int row, int col, int value) {
            int oldValue = get(row, col);
            data[row * columns + col] = value;
            return oldValue;
        }

        @Override
        public final String toString() {
            var out = new StringJoiner("\n");
            for (int row = 0; row < rows; row++) {
                int first = row * columns;
                var rowStr = IntStream.range(first, first + columns)
                        .mapToObj(i -> String.valueOf(data[i]))
                        .collect(Collectors.joining(" "));
                out.add(rowStr);
            }
            return out.toString();
        }

        private String highlight(String indent, int rowFrom, int rowTo, int colFrom, int colTo, int row, int col) {
            String out = "";
            for (int r = 0; r < rows; r++) {
                out += indent;
                for (int c = 0; c < columns; c++) {
                    var v = get(r, c);
                    if (r == row && c == col) {
                        out += underline(bold(v));
                    } else if (r < rowFrom || r >= rowTo || c < colFrom || c >= colTo) {
                        out += dim(v);
                    } else {
                        out += v;
                    }
                    out += " ";
                }
                out += "\n";
            }
            return out;
        }

        int countAdjacentRolls(int row, int col) {
            Objects.checkIndex(row, rows);
            Objects.checkIndex(col, columns);

            int minCol = col - (col > 0 ? 1 : 0);
            int minRow = row - (row > 0 ? 1 : 0);
            int colLimit = col + (col < columns - 1 ? 2 : 1);
            int rowLimit = row + (row < rows - 1 ? 2 : 1);

            int count = 0;
            for (int r = minRow; r < rowLimit; r++) {
                for (int c = minCol; c < colLimit; c++) {
                    count += get(r, c);
                }
            }

            return count - get(row, col);
        }

        public boolean canMove(int row, int col) {
            return get(row, col) == 1 && countAdjacentRolls(row, col) < 4;
        }

        public int removePaperRolls() {
            var m = new Matrix(this.rows(), this.columns());
            int count = 0;
            for (int row = 0; row < this.rows(); row++) {
                for (int col = 0; col < this.columns(); col++) {
                    if (canMove(row, col)) {
                        m.set(row, col, 0);
                        count++;
                    } else {
                        m.set(row, col, get(row, col));
                    }
                }
            }
            if (count > 0) {
                System.arraycopy(m.data, 0, this.data, 0, m.data.length);
            }
            return count;
        }
    }

    @Override
    public String part1(String input) {
        var matrix = Matrix.parse(input);

        int count = 0;
        for (int row = 0; row < matrix.rows(); row++) {
            for (int col = 0; col < matrix.columns(); col++) {
                if (matrix.canMove(row, col)) {
                    count++;
                }
            }
        }
        return String.valueOf(count);
    }

    @Override
    public String part2(String input) {
        var matrix = Matrix.parse(input);
        int totalCount = 0;
        do {
            int count = matrix.removePaperRolls();
            totalCount += count;
            if (count == 0) break;
        } while (totalCount < matrix.rows() * matrix.columns());
        return String.valueOf(totalCount);
    }

    @Override
    public List<TestUnit> tests() {
        var matrix = Matrix.parse("""
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
        """);

        return List.of(
            test("Part 1", List.of(
                ok("Tile [0,0] has 2 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 0, 2)),
                ok("Tile [0,1] has 4 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 1, 4)),
                ok("Tile [0,2] has 3 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 2, 3)),
                ok("Tile [0,3] has 3 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 3, 3)),
                ok("Tile [0,4] has 3 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 4, 3)),
                ok("Tile [0,5] has 3 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 5, 3)),
                ok("Tile [0,6] has 3 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 6, 3)),
                ok("Tile [0,7] has 4 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 7, 4)),
                ok("Tile [0,8] has 3 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 8, 3)),
                ok("Tile [0,9] has 3 adjacent rolls", () -> assertAdjacentRolls(matrix, 0, 9, 3)),

                ok("Tile [1,0] has 3 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 0, 3)),
                ok("Tile [1,1] has 6 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 1, 6)),
                ok("Tile [1,2] has 6 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 2, 6)),
                ok("Tile [1,3] has 7 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 3, 7)),
                ok("Tile [1,4] has 4 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 4, 4)),
                ok("Tile [1,5] has 6 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 5, 6)),
                ok("Tile [1,6] has 4 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 6, 4)),
                ok("Tile [1,7] has 7 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 7, 7)),
                ok("Tile [1,8] has 5 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 8, 5)),
                ok("Tile [1,9] has 4 adjacent rolls", () -> assertAdjacentRolls(matrix, 1, 9, 4))
            ))
        );
    }

    private void assertAdjacentRolls(Matrix matrix, int row, int column, int expectedCount) {
        var actualCount = matrix.countAdjacentRolls(row, column);
        if (actualCount != expectedCount) {
            throw new AssertionError("Expected " + expectedCount + " adjacent rolls at (" + row + ", " + column + "), but got " + actualCount);
        }
    }
}
