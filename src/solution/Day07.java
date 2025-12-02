package solution;

import module java.base;

/**
 * Solutions for Advent of Code 2025 Day 7
 */
public class Day07 implements Part1, Part2 {
    private class TachyonManifold {
        private final BitSet activeTachyons;
        private int splitCount;

        public TachyonManifold(int width) {
            activeTachyons = new BitSet(width);
            activeTachyons.set(width / 2);
        }

        private void activate(int position) {
            activeTachyons.set(position);
        }

        public Iterator<Integer> activeTachyons() {
            return activeTachyons.stream().iterator();
        }

        private void deactivate(int position) {
            activeTachyons.clear(position);
        }

        private void split(int position) {
            activate(position - 1);
            activate(position + 1);
            deactivate(position);
            splitCount++;
        }

        public int splitCount() {
            return splitCount;
        }
    }

    private class QuantumTachyonManifold {
        private final long[] timelines;

        public QuantumTachyonManifold(int width) {
            timelines = new long[width];
            timelines[width / 2] = 1;
        }

        private void activate(int position, long timelines) {
            this.timelines[position] += timelines;
        }

        private void deactivate(int position, long timelines) {
            this.timelines[position] -= timelines;
        }

        public Iterator<Integer> activeTachyons() {
            return IntStream.range(0, timelines.length)
                .filter(i -> timelines[i] > 0)
                .iterator();
        }

        private void split(int position) {
            long n = timelines[position];
            activate(position - 1, n);
            activate(position + 1, n);
            deactivate(position, n);
        }

        public long timelinesCount() {
            return Arrays.stream(timelines).sum();
        }
    }

    @Override
    public String part1(String input) {
        var lines = input.lines()
            .gather(Gatherers.windowFixed(2))
            .map(l -> l.get(0))
            .iterator();

        var tachyonManifold = new TachyonManifold(lines.next().length());

        while (lines.hasNext()) {
            String line = lines.next();

            var it = tachyonManifold.activeTachyons();
            while (it.hasNext()) {
                int tachyon = it.next();
                char c = line.charAt(tachyon);

                if (c == '^') {
                    tachyonManifold.split(tachyon);
                }
            }
        }
        return String.valueOf(tachyonManifold.splitCount());
    }

    @Override
    public String part2(String input) {
        var lines = input.lines()
            .gather(Gatherers.windowFixed(2))
            .map(l -> l.get(0))
            .iterator();

        var tachyonManifold = new QuantumTachyonManifold(lines.next().length());

        while (lines.hasNext()) {
            String line = lines.next();

            var it = tachyonManifold.activeTachyons();
            while (it.hasNext()) {
                int tachyon = it.next();
                char c = line.charAt(tachyon);

                if (c == '^') {
                    tachyonManifold.split(tachyon);
                }
            }
        }
        return String.valueOf(tachyonManifold.timelinesCount());
    }
}
