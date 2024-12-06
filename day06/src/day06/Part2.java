package day06;

import java.util.*;
import java.util.function.Consumer;
import java.util.function.Supplier;

@SuppressWarnings("LocalVariableHidesMemberVariable")
public class Part2 implements Consumer<String>, Supplier<Integer> {
    private static final byte HOME     = 0b0000001;
    private static final byte UP       = 0b0000010;
    private static final byte DOWN     = 0b0000100;
    private static final byte LEFT     = 0b0001000;
    private static final byte RIGHT    = 0b0010000;
    private static final byte VIABLE   = 0b0100000;
    private static final byte OBSTACLE = 0b1000000;

    private final class Grid {
        private final int width;
        private final int height;

        private final byte[][] grid;

        private Position currentPosition;
        private Direction currentDirection;
        private int counter = 0;

        public Grid(int width, int height) {
            this.width = width;
            this.height = height;

            this.grid = new byte[height][width];
        }

        public void putObstacles(List<Position> positions) {
            for (Position p : positions) {
                grid[p.y()][p.x()] |= OBSTACLE;
            }
        }

        public void putHome(Position p) {
            grid[p.y()][p.x()] |= HOME | UP;
            currentPosition = p;
            currentDirection = Direction.Up;
        }

        private boolean contains(int x, int y) {
            return 0 <= x && x < width && 0 <= y && y < height;
        }

        public int countViableObstructions() {
            int counter = 0;
            int viableObstructions = 0;
            while (currentPosition instanceof Position(int x, int y) && contains(x, y)) {
                counter++;
                var next = currentPosition.moveIn(currentDirection);
                if (next instanceof Position(int nextX, int nextY) && contains(nextX, nextY)) {
                    if (isSet(nextX, nextY, OBSTACLE)) {
                        set(x, y, currentDirection.asByte());
                        currentDirection = currentDirection.turn();
                        continue;
                    }

                    if (isNeverVisitedBefore(nextX, nextY)) {
                        var alternateReality = this.deepClone(viableObstructions + 1);
                        if (alternateReality.wouldLoopIfObstructionPlacedAt(nextX, nextY)) {
                            set(nextX, nextY, VIABLE);
                            viableObstructions++;
                        }
                    }

                    set(x, y, currentDirection.asByte());
                }
                currentPosition = next;
            }
            return viableObstructions;
        }

        public boolean wouldLoopIfObstructionPlacedAt(int obstructionsX, int obstructionsY) {
            set(obstructionsX, obstructionsY, (byte)(OBSTACLE|VIABLE));
            while (currentPosition instanceof Position(int x, int y) && contains(x, y)) {
                var next = currentPosition.moveIn(currentDirection);
                if (next instanceof Position(int nextX, int nextY) && contains(nextX, nextY)) {
                    if (isSet(nextX, nextY, OBSTACLE)) {
                        set(x, y, currentDirection.asByte());
                        currentDirection = currentDirection.turn();
                        continue;
                    }

                    if (isSet(nextX, nextY, currentDirection.asByte())) {
                        return true;
                    }

                    set(x, y, currentDirection.asByte());
                }
                currentPosition = next;
            }
            return false;
        }

        private boolean isSet(int x, int y, byte value) {
            return (0 <= y && y < height)
                && (0 <= x && x < width)
                && ((grid[y][x] & value) == value);
        }

        private boolean isNeverVisitedBefore(int x, int y) {
            return y < 0 || y >= height
                || x < 0 || x >= width
                || (grid[y][x] == 0);
        }

        private void set(int x, int y, byte value) {
             grid[y][x] |= value;
        }

        private char charAt(int x, int y) {
            var value = grid[y][x];
            var position = currentPosition;
            var direction = currentDirection;
            
            if (position instanceof Position(int x1, int y1) && x == x1 && y == y1) {
                return direction.asChar();
            }

            if ((value & HOME) == HOME) {
                return '^';
            }

            if ((value & VIABLE) == VIABLE) {
                if (position.moveIn(direction)
                        instanceof Position(int nextX, int nextY)
                        && x == nextX && y == nextY
                ) {
                    return 'O';
                }
            }

            if ((value & OBSTACLE) == OBSTACLE) {
                return '#';
            }

            boolean vertical = (value & (UP | DOWN)) != 0;
            boolean horizontal = (value & (LEFT | RIGHT)) != 0;
            if (vertical && horizontal) {
                return '+';
            }

            if (vertical) {
                return '|';
            }

            if (horizontal) {
                return '-';
            }

            return '.';
        }

        @Override
        public String toString() {
            var out = new StringBuilder(height * (width + 1));
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < width; x++) {
                    out.append(charAt(x, y));
                }
                out.append('\n');
            }
            return out.toString();
        }

        Grid deepClone(int counter) {
            var clone = new Grid(width, height);
            for (int y = 0; y < height; y++) {
                clone.grid[y] = Arrays.copyOf(grid[y], width);
            }
            clone.currentPosition = currentPosition;
            clone.currentDirection = currentDirection;
            clone.counter = counter;
            return clone;
        }

        private void printGrid() {
            System.out.print("┌");
            System.out.print("─".repeat(width));
            System.out.println("┐");
            for (int y = 0; y < height; y++) {
                System.out.print('│');
                for (int x = 0; x < width; x++) {
                    System.out.print(charAt(x, y));
                }
                System.out.println('│');
            }
            System.out.print("└");
            System.out.print("─".repeat(width));
            System.out.println("┘");
        }
    }

    enum Direction {
        Up, Right, Down, Left;

        int y() {
            return switch (this) {
                case Up -> -1;
                case Down -> 1;
                case Right, Left -> 0;
            };
        }

        int x() {
            return switch (this) {
                case Up, Down -> 0;
                case Right -> 1;
                case Left -> -1;
            };
        }

        Direction turn() {
            return switch (this) {
                case Up -> Right;
                case Right -> Down;
                case Down -> Left;
                case Left -> Up;
            };
        }

        byte asByte() {
            return switch (this) {
                case Up -> UP;
                case Right -> RIGHT;
                case Down -> DOWN;
                case Left -> LEFT;
            };
        }

        private char asChar() {
            return switch (this) {
                case Up -> '^';
                case Right -> '>';
                case Down -> 'v';
                case Left -> '<';
            };
        }
    }

    record Position(int x, int y) {
        @Override
        public String toString() {
            return new StringJoiner(",", "(", ")")
                    .add(String.valueOf(x))
                    .add(String.valueOf(y))
                    .toString();
        }

        public Position moveIn(Direction direction) {
            return new Position(x + direction.x(), y + direction.y());
        }

        private boolean equals(int x, int y) {
            return this.x == x && this.y == y;
        }
    }

    private final List<Position> obstructions = new ArrayList<>();

    private Position home = new Position(0, 0);
    private int width = 0;
    private int height = 0;

    @Override
    public void accept(String s) {
        width = s.length();

        int y = height;
        for (int x = 0; x < width; x++) {
            switch (s.charAt(x)) {
                case '^' -> home = new Position(x, y);
                case '#' -> obstructions.add(new Position(x, y));
            }
        }
        height += 1;
    }


    @Override
    public Integer get() {
        var grid = new Grid(width, height);
        grid.putObstacles(obstructions);
        grid.putHome(home);


        return grid.countViableObstructions();
    }

    @Override
    public String toString() {
        return String.valueOf(get());
    }
}
