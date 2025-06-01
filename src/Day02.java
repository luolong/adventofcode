import static aoc.Trace.Part.Part1;
import static aoc.Trace.Part.Part2;

import aoc.Input;
import aoc.Trace;
import aoc.Trace.Compute;
import aoc.Trace.Parse;

public void main(String[] args) throws IOException {
  var input = Input.from(getClass(), args);

  try (var trace = new Trace();
      var lines = Files.lines(input.path())) {
    int part1 = 0;
    int part2 = 0;
    var it = lines.iterator();
    while (it.hasNext()) {
      var s = it.next();

      var parse = Parse.start();
      var levels = Arrays.stream(s.split(" "))
          .mapToInt(Integer::parseInt)
          .toArray();
      parse.commit();

      var p1 = Compute.start();
      var fault = findFault(levels);
      if (fault == -1) {
        part1 += 1;
        part2 += 1;
      }
      p1.commit();

      if (fault != -1) {
        var p2 = Compute.start(Part2);
        for (int i = 0; i < levels.length; i++) {
          var arr = copyWithout(levels, i);
          if (findFault(arr) < 0) {
            part2 += 1;
            break;
          }
        }
        p2.commit();
      }
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

@SuppressWarnings("NonAsciiCharacters")
int findFault(int[] levels) {
  for (int i = 1; i < levels.length; i++) {
    var a = levels[i];
    var b = levels[i - 1];
    var Δ = Math.abs(a - b);
    if (1 > Δ || Δ > 3) {
      return i;
    }
    if (i > 1) {
      var c = levels[i - 2];
      if ((a - b) * (b - c) < 0) {
        return i;
      }
    }
  }

  return -1;
}

private int[] copyWithout(int[] levels, int i) {
  var arr = new int[levels.length - 1];
  if (i > 0) {
    System.arraycopy(levels, 0, arr, 0, i);
  }
  if (i < levels.length - 1) {
    System.arraycopy(levels, i + 1, arr, i, arr.length - i);
  }
  return arr;
}
