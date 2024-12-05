import static aoc.Trace.Part.Part1;
import static aoc.Trace.Part.Part2;

import aoc.Flyweight;
import aoc.Input;
import aoc.Trace;
import aoc.Trace.Compute;
import aoc.Trace.Parse;

public void main(String[] args) throws IOException, ParseException {
  var input = Input.from(getClass(), args);

  try (var trace = new Trace()) {
    var rules = new HashMap<Flyweight, Set<Flyweight>>();

    var parse = Parse.start();
    var lines = Flyweight.lines(input.path());
    while (lines.hasNext()) {
      var line = lines.next();
      if (line.isBlank())
        break;

      var split = line.splitOnce('|');
      rules
          .computeIfAbsent(split.left(), k -> new HashSet<>())
          .add(split.right());
    }
    parse.commit();

    int part1 = 0;
    int part2 = 0;

    updates:
    while (lines.hasNext()) {
      var line = lines.next();
      if (!line.isBlank()) {
        var pages = line.splitToList(',');

        var p1 = Compute.start(Part1);
        var it = pages.listIterator(pages.size());
        while (it.hasPrevious()) {
          var index = it.previousIndex();
          var mustComeAfter = rules.getOrDefault(
              it.previous(),
              Set.of()
          );

          while (it.hasPrevious()) {
            var previous = it.previous();
            if (mustComeAfter.contains(previous)) {
              continue updates;
            }
          }

          it = pages.listIterator(index);
        }

        part1 += pages.get(pages.size() / 2).parseInt();
        p1.commit();
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

private boolean isPagesInRightOrder(
    Flyweight line,
    Map<Flyweight, Set<Flyweight>> rules
) {
  return true;
}
