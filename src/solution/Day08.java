package solution;

import static java.util.stream.Collectors.joining;
import static java.util.stream.Collectors.toMap;
import static solution.Ansi.dim;

import module java.base;

import solution.Day08.Result.*;
import solution.Test.TestUnit;

/**
 * Solutions for Advent of Code 2025 Day 8
 */
public class Day08 implements Part1, Part2, Test {

    sealed interface Result {
        value record Merged(Pair pair, Circuit cirquit) implements Result {}
        enum Unchanged implements Result {
            INSTANCE;
        }

        default boolean isMerged() {
            return this instanceof Merged;
        }

        default Circuit circuit() {
            return switch(this) {
                case Merged(var _, Circuit circuit) -> circuit;
                case Unchanged.INSTANCE -> throw new NoSuchElementException("Unchanged result has no circuit");
            };
        }

        default Pair pair() {
            return switch(this) {
                case Merged(Pair pair, var _) -> pair;
                case Unchanged.INSTANCE -> throw new NoSuchElementException("Unchanged result has no junction boxes.");
            };
        }
    }
    static class Playground {
        private final Map<Position, Circuit> circuits;
        private int distinctCircuits;

        public static Playground parse(String input) {
            return new Playground(input.lines()
                .map(Position::parse)
                .collect(toMap( Function.identity(), Circuit::of)));
        }

        private Playground(Map<Position, Circuit> circuits) {
            this.circuits = new HashMap<>(circuits);
            this.distinctCircuits = circuits.size();
        }

        public void addAll(List<Position> junctionBoxes) {
            junctionBoxes.stream().collect(toMap(
                Function.identity(),
                Circuit::of,
                (a, b) -> a,
                () -> circuits
            ));
        }

        public Queue<Pair> pairs() {
            return pairs(Integer.MAX_VALUE);
        }

        public Queue<Pair> pairs(int limit) {
            var junctionBoxes = List.copyOf(circuits.keySet());
            int totalPairs = junctionBoxes.size() * (junctionBoxes.size() - 1) / 2;

			var pairs = limit >= totalPairs
			    ? new PriorityQueue<Pair>(totalPairs, Comparator.comparingLong(Pair::magnitude))
				: new PriorityQueue<Pair>(limit + 1, Comparator.comparingLong(Pair::magnitude).reversed());

			var it = junctionBoxes.listIterator();
			while (it.hasNext()) {
                var a = it.next();
                var others = junctionBoxes.listIterator(it.nextIndex());
                while (others.hasNext()) {
                    var b = others.next();
                    pairs.offer(Pair.of(a, b));

                    if (limit < totalPairs && pairs.size() > limit) {
                        pairs.poll(); // remove largest if over limit
                    }                }
            }

            if (limit < totalPairs) {
                var minHeap = new PriorityQueue<Pair>(limit, Comparator.comparingLong(Pair::magnitude));
                minHeap.addAll(pairs);
                return minHeap;
            }

            return pairs;
        }

        public int size() {
            return distinctCircuits;
        }

        public Circuit get(Position junctionBox) {
            return circuits.get(junctionBox);
        }

        public Stream<Circuit> distinctCircuits() {
            return circuits.values().stream().distinct();
        }

        public Stream<Circuit> circuitsOfSize(int ofSize) {
            return distinctCircuits().filter(c -> c.size() == ofSize);
        }

        public Result connect(Pair pair) {
            if (pair instanceof Pair(var a, var b)) {
                assert circuits.containsKey(a) && circuits.containsKey(b);
                var ca = circuits.get(a);
                var cb = circuits.get(b);
                if (ca == cb) return Unchanged.INSTANCE;

                distinctCircuits -= 1;

                var circuit = circuits.compute(a, (_, c) -> c.connect(circuits.get(b)));
                circuit.junctionBoxes().forEach(j -> circuits.put(j, circuit));
                return new Merged(pair, circuit);
            }

            throw new IllegalArgumentException("Invalid pair");
        }
    }

    value record Position(int x, int y, int z) implements Comparable<Position> {
        static Position parse(String s) {
            var parts = s.trim().split(",");
            if (parts.length != 3) {
                throw new IllegalArgumentException("Invalid position format");
            }
            return new Position(Integer.parseInt(parts[0]), Integer.parseInt(parts[1]), Integer.parseInt(parts[2]));
        }

        double distance(Position b) {
            return Math.sqrt(this.minus(b).magnitude());
        }

        Position minus(Position b) {
            return new Position(x - b.x, y - b.y, z - b.z);
        }

        public long magnitude() {
            return (long) x * x + (long) y * y + (long) z * z;
        }

        @Override
        public final String toString() {
            return String.format("(%d,%d,%d)", x, y, z);
	    }

		@Override
		public int compareTo(Position o) {
			return Long.compare(this.magnitude(), o.magnitude());
		}
    }


    record Circuit(NavigableSet<Position> junctionBoxes) {
        public static Circuit of(Position junctionBox) {
            var set = new TreeSet<Position>(List.of(junctionBox));
            return new Circuit(set);
        }

        Circuit connect(Circuit other) {
            junctionBoxes.addAll(other.junctionBoxes());
            return this;
        }

        double distance(Position b) {
            return junctionBoxes.stream().mapToDouble(p -> p.distance(b)).min().orElse(Double.POSITIVE_INFINITY);
        }

        int size() {
            return junctionBoxes.size();
        }

        public final String toDebugString() {
            return junctionBoxes.stream().map(Position::toString).collect(joining(", ", "{", "}"));
        }

        @Override
        public final String toString() {
            //return junctionBoxes.stream().map(Position::toString).collect(joining(", ", "{", "}"));
            return switch(junctionBoxes.size()) {
                case 0 -> "{empty circuit}";
                case 1 -> "{" + junctionBoxes.first() + "}";
                case 2 -> "{" + junctionBoxes.first() + ", " + junctionBoxes.last() + "}";
                case 3 -> "{" + junctionBoxes.first() + ", ..., " + junctionBoxes.last() + "}";
                default -> "{" + junctionBoxes.first() + ", ...(" + (junctionBoxes.size() - 2) + " more), " + junctionBoxes.last() + "}";
            };
        }
    }

    value record Pair(Position a, Position b) {
        public static Pair of(Position a, Position b) {
            var comparison = Comparator.comparingLong(Position::magnitude).compare(a, b);
            if (comparison < 0) {
                return new Pair(a, b);
            }
            else {
                return new Pair(b, a);
            }
        }

        public double distance() {
            return Math.sqrt(magnitude());
        }

        public long magnitude() {
            return a.minus(b).magnitude();
        }

        @Override
        public final String toString() {
            return String.format("%s and %s", a, b);
        }
    }

    @Override
    public String part1(String input) {
        Playground playground = Playground.parse(input);
        var pairs = playground.pairs(1000);
        pairs.forEach(pair -> playground.connect(pair));

        var value = playground.distinctCircuits()
            .sorted(Comparator.comparingInt(Circuit::size).reversed())
            .limit(3)
            .mapToInt(Circuit::size)
            .reduce(1, (a, b) -> a * b);

        return String.valueOf(value);
    }

    @Override
    public String part2(String input) {
        Playground playground = Playground.parse(input);

        Pair lastMerge = null;
        var pairs = playground.pairs();
        while (playground.size() > 1) {
            var next = pairs.poll();
            if (next == null)
                break;

            var result = playground.connect(next);

            if (result instanceof Merged) {
                lastMerge = next;
            }
        }

        assert lastMerge != null;
        return String.valueOf((long) lastMerge.a().x() * (long) lastMerge.b().x());
    }

    private static final String INPUT;
    static {
        try {
            INPUT = Files.readString(Path.of("day08_test.txt"));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public List<TestUnit> tests() {
        return List.of(
            test("Part 1", () -> {
                var playground = Playground.parse(INPUT);
                var pairs = playground.pairs(10);
                return List.of(
                    ok("Initially we should have 20 circuits", () -> assertEquals(20, playground.size())),
                    test("After closest pair of junction boxes have been connected", () -> {
                        var pair = pairs.poll();
                        var circuit = playground.connect(pair).circuit();
                        return List.of(
                            ok("Closest pair of junction boxes should be 162,817,812 and 425,690,689",
                                () -> assertEquals(Pair.of(new Position(162, 817, 812), new Position(425, 690, 689)), pair)),
                            ok("Circuit should have 2 junction boxes", () -> assertEquals(2, circuit.size())),
                            ok("Circuit should have connected 162,817,812 and 425,690,689",
                                () -> assertEquals(Set.of(new Position(162, 817, 812), new Position(425, 690, 689)), circuit.junctionBoxes())),
                            test("Both junction boxes should be included in same circuit",
                                circuit.junctionBoxes().stream().map(jb ->
                                    ok("Junction box " + jb + " should be included in circuit " + circuit, () -> assertSame(circuit, playground.get(jb)))).toList()
                            ),
                            ok("After connecting them, there is a single circuit which contains two junction boxes", () -> assertEquals(1L, playground.circuitsOfSize(2).count())),
                            ok("Remaining 18 junction boxes remain in their own individual circuits", () -> assertEquals(18L, playground.circuitsOfSize(1).count())),
                            ok("Total number of circuits should be 19", () -> assertEquals((int) playground.distinctCircuits().count(), playground.size()))
                        );
                    }),
                    test("Connecting second closest pair", () -> {
                        var pair = pairs.poll();
                        var circuit = playground.connect(pair).circuit();
                        return List.of(
                            ok("Two closest junction boxes that are not already connected are 162,817,812 and 431,825,9889",
                                () -> assertEquals(Pair.of(new Position(162, 817, 812), new Position(431, 825, 988)), pair)),
                            ok("After connecting them, new circuit should have 3 junction boxes", () -> assertEquals(3, circuit.size())),
                            test("Three junction boxes should be included in same circuit", circuit.junctionBoxes().stream()
                                .map(jb ->
                                    ok("Junction box " + jb + " should be included in circuit " + circuit,
                                        () -> assertSame(circuit, playground.get(jb))))
                                .toList()
                            ),
                            ok("After connecting them, there is now a single circuit which contains three junction boxes", () -> assertEquals(1L, playground.circuitsOfSize(3).count())),
                            ok("And an additional 17 circuits contain one junction box each", () -> assertEquals(17L, playground.circuitsOfSize(1).count())),
                            ok("Total number of circuits should be 18", () -> assertEquals((int) playground.distinctCircuits().count(), playground.size()))
                        );
                    }),
                    test("Connecting third closest pair of junction boxes", () -> {
                        var pair = pairs.poll();
                        var circuit = playground.connect(pair).circuit();
                        return List.of(
                            ok("The next two junction boxes to connect are 906,360,560 and 805,96,715",
                                () -> assertEquals(Pair.of(new Position(906, 360, 560), new Position(805, 96, 715)), pair)),
                            ok("New circuit should have 2 junction boxes", () -> assertEquals(2, circuit.size())),
                            test("Both junction boxes should be included in same circuit",
                                circuit.junctionBoxes().stream()
                                .map(jb ->
                                    ok("Junction box " + jb + " should be included in circuit " + circuit, () -> assertSame(circuit, playground.get(jb)))
                                )
                                .toList()
                            ),
                            ok("After connecting them, there is a circuit containing 3 junction boxes", () -> assertEquals(1L, playground.circuitsOfSize(3).count())),
                            ok("A circuit containing 2 junction boxes", () -> assertEquals(1L, playground.circuitsOfSize(2).count())),
                            ok("and 15 circuits which contain one junction box each", () -> assertEquals(15L, playground.circuitsOfSize(1).count()))
                        );
                    }),
                    test("Connecting fourth closest pair of junction boxes", () -> {
                        var pair = pairs.poll();
                        var a = playground.get(pair.a());
                        var b = playground.get(pair.b());
                        var circuit = playground.connect(pair);
                        return List.of(
                            ok("The next two junction boxes are 431,825,988 and 425,690,689",
                                () -> assertEquals(Pair.of(new Position(431,825,988), new Position(425,690,689)), pair)),
                            test("Because these two junction boxes were already in the same circuit, nothing happens", () -> {
                                return List.of(
                                    ok("The two junction boxes were already in the same circuit", () -> assertSame(a, b)),
                                    ok("Connecting them changes nothing", () -> assertEquals(Unchanged.INSTANCE, circuit))
                                );
                            })
                        );
                    }),
                    test("After making the ten shortest connections", () -> {
                        pairs.forEach(playground::connect);

                        return List.of(
                            ok("There are 11 circuits", () -> assertEquals(11, playground.size())),
                            ok("one circuit which contains 5 junction boxes", () -> assertEquals(1L, playground.circuitsOfSize(5).count())),
                            ok("one circuit which contains 4 junction boxes", () -> assertEquals(1L, playground.circuitsOfSize(4).count())),
                            ok("two circuits which contain 2 junction boxes each", () -> assertEquals(2L, playground.circuitsOfSize(2).count())),
                            ok("and seven circuits which each contain a single junction box", () -> assertEquals(7L, playground.circuitsOfSize(1).count())),
                            ok("Multiplying together the sizes of the three largest circuits produces 40",
                                () -> assertEquals(40, playground.distinctCircuits()
                                                                 .sorted(Comparator.comparingInt(Circuit::size).reversed())
                                                                 .limit(3)
                                                                 .mapToInt(Circuit::size)
                                                                 .reduce(1, (a, b) -> a * b)))
                        );
                    })
                );
            }),

            test("Part 2", () -> {
                var result = part2(INPUT);
                return List.of(
                    ok("The response to part2 based on test input should be 25272", () -> assertEquals("25272", result))
                );
            })
        );
    }
}
