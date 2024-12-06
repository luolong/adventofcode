package day06;

import java.util.*;
import java.util.function.Consumer;
import java.util.function.Supplier;

@SuppressWarnings("ALL")
public class Part1 implements Consumer<String>, Supplier<Integer> {

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
    }

    record Position(int x, int y) {
        @Override
        public String toString() {
            return new StringJoiner(",", "(", ")")
                    .add(String.valueOf(x))
                    .add(String.valueOf(y))
                    .toString();
        }

        public boolean withinGrid(int width, int height) {
            return 0 <= x && x < width && 0 <= y && y < height;
        }

        public Position moveIn(Direction direction) {
            return new Position(x + direction.x(), y + direction.y());
        }
    }

    private final List<Position> obstructions = new ArrayList<>();

    private Position position = new Position(0, 0);
    private int width = 0;
    private int height = 0;

    @Override
    public void accept(String s) {
        width = s.length();

        int y = height;
        for (int x = 0; x < width; x++) {
            switch (s.charAt(x)) {
                case '^' -> position = new Position(x, y);
                case '#' -> obstructions.add(new Position(x, y));
            }
        }
        height += 1;
    }


    @Override
    public Integer get() {
        var grid = new char[height][width];
        for (int y = 0; y < height; y++) Arrays.fill(grid[y], '.');
        obstructions.forEach(o -> grid[o.y()][o.x()] = '#');

        var position = this.position;
        var direction = Direction.Up;

        grid[position.y()][position.x()] = 'X';
        int visitedPositions = 1;

        while (position.withinGrid(width, height)) {
            var next = position.moveIn(direction);
            if (!next.withinGrid(width, height)) {
                break;
            }

            if (next instanceof Position(int x, int y)) {
                position = switch (grid[y][x]) {
                    case '#' -> {
                        direction = direction.turn();
                        yield position;
                    }
                    case 'X' -> next;
                    default -> {
                        grid[y][x] = 'X';
                        visitedPositions++;
                        yield next;
                    }
                };
            }
        }

        return visitedPositions;
    }

    @Override
    public String toString() {
        return String.valueOf(get());
    }
}
