package solution;

import module java.base;

public class Day01 implements Part1, Part2, Test {
    @Override
    public String part1(String input) {
        var answer = input.lines()
                .map(Rotation::parse)
                .gather(Gatherers.scan(State::initial, State::rotate))
                .filter(state -> state.position() == 0)
                .count();
       return String.valueOf(answer);
    }

    @Override
    public String part2(String input) {
        var answer = input.lines()
                .map(Rotation::parse)
                .gather(Gatherers.fold(State::initial, State::rotate))
                .findFirst()
                .orElseThrow(() -> new IllegalStateException("No solution found"));
        return String.valueOf(answer.zeroed());
    }

    enum Direction {
        L, R;

        public static Direction valueOf(char ch) {
            return switch (ch) {
                case 'L' -> L;
                case 'R' -> R;
                default -> throw new IllegalArgumentException("Invalid direction: " + ch);
            };
        }
    }

    value record Rotation(Direction dir, int distance) {
        public static Rotation parse(String str) {
            Direction dir = Direction.valueOf(str.charAt(0));
            int distance = Integer.parseInt(str.substring(1));
            return new Rotation(dir, distance);
        }

        public static Rotation right(int distance) {
            return new Rotation(Direction.R, distance);
        }

        public static Rotation left(int distance) {
            return new Rotation(Direction.L, distance);
        }

        public String toString() {
            return String.format("%s%d", dir, distance);
        }
    }

    value record State(int position, int zeroed) {
        static State initial() {
            return new State(50, 0);
        }

        State rotate(Rotation rotation) {
            if (rotation instanceof Rotation(var dir, var rotationDistance)) {
                // Capture number of full rotations
                var fullRotations = rotationDistance / 100;
                var zeroed = zeroed() + fullRotations;

                var remainingDistance = rotationDistance % 100;

                var position = position();
                zeroed += switch (dir) {
                    case L -> (position > 0 && remainingDistance >= position) ? 1 : 0;
                    case R -> (position + remainingDistance >= 100) ? 1 : 0;
                };

                position += 100 + switch (dir) {
                    case L -> -remainingDistance;
                    case R -> remainingDistance;
                };
                position = position % 100;

                // Adjust for negative positions
                return new State(position % 100, zeroed);
            }

            throw new AssertionError("Impossible rotation: " + rotation);
        }
    }


    // TESTS
    // --------------------------------------

    @Override
    public List<TestUnit> tests() throws AssertionError {
        var state = new State[] {State.initial()};
        return List.of(
            test("Testing simple rotation", List.of(
                ok("Rotating left by 1 click from 50 points the dial at 49", () -> testRotationPosition(State.initial(), Rotation.left(1), 49, 0)),
                ok("Rotating right by 1 click from 50 points the dial at 51", () -> testRotationPosition(State.initial(), Rotation.right(1), 51, 0))
            )),
            test("Test cases from AoC", List.of(
                ok("Rotate right by 8 clicks from 11 moves the dial to 19", () -> testRotationPosition(new State(11, 0), Rotation.right(8), 19, 0)),
                ok("Rotate left by 19 clicks from 19 moves the dial to 0", () -> testRotationPosition(new State(19, 0), Rotation.left(19), 0, 1))
            )),
            test("AoC sample input", List.of(
                ok("The dial is rotated L68 to point at 82.", () -> state[0] = testRotationPosition(state[0], Rotation.left(68), 82, 1)),
                ok("The dial is rotated L30 to point at 52.", () -> state[0] = testRotationPosition(state[0], Rotation.left(30), 52, 1)),
                ok("The dial is rotated R48 to point at 0.", () -> state[0] = testRotationPosition(state[0], Rotation.right(48), 0, 2)),
                ok("The dial is rotated L5 to point at 95.", () -> state[0] = testRotationPosition(state[0], Rotation.left(5), 95, 2)),
                ok("The dial is rotated R60 to point at 55.", () -> state[0] = testRotationPosition(state[0], Rotation.right(60), 55, 3)),
                ok("The dial is rotated L55 to point at 0.", () -> state[0] = testRotationPosition(state[0], Rotation.left(55), 0, 4)),
                ok("The dial is rotated L1 to point at 99.", () -> state[0] = testRotationPosition(state[0], Rotation.left(1), 99, 4)),
                ok("The dial is rotated L99 to point at 0.", () -> state[0] = testRotationPosition(state[0], Rotation.left(99), 0, 5)),
                ok("The dial is rotated R14 to point at 14.", () -> state[0] = testRotationPosition(state[0], Rotation.right(14), 14, 5)),
                ok("The dial is rotated L82 to point at 32.", () -> state[0] = testRotationPosition(state[0], Rotation.left(82), 32, 6))
            ))
        );
    }

    public State testRotationPosition(State initial, Rotation rotation, int expectedPostion, int expectedZeroedCount) {
        var state = initial.rotate(rotation);
        if (state instanceof State(var actualPosition, var actualZeroed)) {
            assert actualPosition == expectedPostion : String.format("Rotating from %d by %s should cause the dial to point at %s. Actual position: %d", initial.position(), rotation, expectedPostion, actualPosition);
            assert actualZeroed == expectedZeroedCount : String.format("Rotating from %d position by %s should %s %d. Actual zeroed count: %d",
                initial.position(), rotation, initial.zeroed() == expectedZeroedCount ? "keep zeroed count at" : "increase zeroed count to", expectedZeroedCount, actualZeroed);
        }
        return state;
    }

}
