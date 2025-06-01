import static aoc.Trace.Part.Part1;
import static aoc.Trace.Part.Part2;
import static java.lang.Integer.parseInt;

import aoc.Input;
import aoc.Trace;
import aoc.Trace.Compute;
import aoc.Trace.Parse;

private static final Pattern PATTERN = Pattern.compile(
    "(mul|do|don't)\\((?:(\\d{1,3}),(\\d{1,3}))?\\)"
);

sealed interface Instruction {

  static Instruction of(MatchResult match) {
    var cmd = match.group(1);
    var arg1 = match.group(2);
    var arg2 = match.group(3);

    return switch (cmd) {
      case "mul" -> arg1 != null && arg2 != null
          ? new Mul(parseInt(arg1), parseInt(arg2))
          : null;
      case "do" -> arg1 == null && arg2 == null ? new Do() : null;
      case "don't" -> arg1 == null && arg2 == null
          ? new Dont()
          : null;
      default -> null;
    };
  }
}

record Do() implements Instruction {

}

record Dont() implements Instruction {

}

record Mul(int a, int b) implements Instruction {

}

public void main(String[] args) throws IOException {
  var input = Input.from(getClass(), args);
  long dayStart = System.nanoTime();

  try (var trace = new Trace();
      var scanner = new Scanner(Files.newBufferedReader(input.path()))) {
    int part1 = 0;
    int part2 = 0;

    var parse = Parse.start();
    var instructions = scanner
        .findAll(PATTERN)
        .map(Instruction::of)
        .toList();
    parse.commit();

    var doit = true;
    for (var instruction : instructions) {
      switch (instruction) {
        case Mul(int a, int b) -> {
          var p1 = Compute.start();
          part1 += a * b;
          p1.commit();

          var p2 = Compute.start(Part2);
          if (doit) {
            part2 += a * b;
          }
          p2.commit();
        }
        case Do _ -> {
          var p2 = Compute.start(Part2);
          doit = true;
          p2.commit();
        }
        case Dont _ -> {
          var p2 = Compute.start(Part2);
          doit = false;
          p2.commit();
        }
        case null -> {
        }
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
