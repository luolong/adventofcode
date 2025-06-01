import static aoc.Trace.Part.Part1;
import static aoc.Trace.Part.Part2;

import aoc.Input;
import aoc.Trace;
import aoc.Trace.Collect;
import aoc.Trace.Compute;
import aoc.Trace.Parse;

record Pair(int a, int b) {
    public static Pair parse(String s) {
        var parts = s.split(" +", 2);
        return new Pair(Integer.parseInt(parts[0]), Integer.parseInt(parts[1]));
    }
}

public void main(String[] args) throws IOException {
    var input = Input.from(getClass(), args);

    var lineCount = 0;
    try (var lines = Files.lines(input.path())) {
        var parse = Parse.start();
        lineCount = (int) lines.count();
        parse.commit();
    }

    try (var trace = new Trace();
        var lines = Files.lines(input.path())) {
        int[] a = new int[lineCount];
        int[] b = new int[lineCount];
        int[] c = new int[lineCount];
        int[] d = new int[lineCount];
        int clen = 0;

        int i = 0;
        var it = lines.iterator();
        while (it.hasNext()) {
            var parse = Parse.start();
            var p = Pair.parse(it.next());
            parse.commit();

            var cc = Collect.start();
            a[i] = p.a();
            b[i] = p.b();
            cc.commit();

            var c2 = Collect.start(Part2);
            if (i == 0) {
                c[clen] = p.b;
                d[clen++] = 1;
            } else {
                var index = Arrays.binarySearch(c, 0, clen, p.b());
                if (index < 0) {
                    index = (index * -1) - 1;
                    System.arraycopy(c, index, c, index + 1, clen - index);
                    System.arraycopy(d, index, d, index + 1, clen - index);
                    c[index] = p.b();
                    d[index] = 1;
                    clen += 1;
                } else {
                    d[index] += 1;
                }
            }
            c2.commit();

            i++;
        }

        var cc = Collect.start();
        Arrays.sort(a);
        Arrays.sort(b);
        cc.commit();

        var c2 = Collect.start(Part2);
        c = Arrays.copyOf(c, clen);
        c2.commit();

        var part1 = 0L;
        var part2 = 0L;
        for (int line = 0; line < a.length; line++) {
            var p1 = Compute.start(Part1);
            part1 += Math.abs(a[line] - b[line]);
            p1.commit();

            var p2 = Compute.start(Part2);
            int index = Arrays.binarySearch(c, a[line]);
            long count = index < 0 ? 0 : d[index];
            part2 += a[line] * count;
            p2.commit();
        }

        var stats = trace.stop();
        System.out.println(stats.formatResult(Part1, part1));
        System.out.println(stats.formatResult(Part2, part2));
        System.out.printf("%s solved in %s%n", input.day(), stats.totalTime());
    } catch (IOException e) {
        System.err.println("Failed to read file " + e.getMessage());
        System.exit(1);
    }
}
