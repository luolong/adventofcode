import static aoc.Trace.Part.Part1;
import static aoc.Trace.Part.Part2;

import aoc.Input;
import aoc.Trace;
import aoc.Trace.Compute;
import aoc.Trace.Parse;

private static final byte LF = '\n';
private static final byte[] XMAS = "XMAS".getBytes();
private static final byte[] SAMX = "SAMX".getBytes();

private static final byte S = SAMX[0];
private static final byte A = SAMX[1];
private static final byte M = SAMX[2];

record Haystack(int rows, int columns, byte[][] letters) {

  public static Haystack of(byte[] bytes) {
    int columns = 0;
    while (columns < bytes.length) {
      if (bytes[columns] == LF) {
        int rows = bytes.length / (columns + 1);
        var letters = new byte[rows][];
        for (int r = 0; r < rows; r++) {
          int start = r * (columns + 1);
          int end = start + columns;
          letters[r] = Arrays.copyOfRange(bytes, start, end);
        }
        return new Haystack(rows, columns, letters);
      }
      columns++;
    }
    return null;
  }

  int countWordsAt(int row, int col) {
    return switch ((char) byteAt(row, col)) {
      case 'X' -> countMatches(row, col, XMAS);
      case 'S' -> countMatches(row, col, SAMX);
      default -> 0;
    };
  }

  int countXMasAt(int row, int col) {
    if (row == 0 || col == 0)
      return 0;
    if (row == rows - 1 || col == columns - 1)
      return 0;
    if (byteAt(row, col) == A) {
      var ul = byteAt(row - 1, col - 1);
      var ur = byteAt(row - 1, col + 1);
      var bl = byteAt(row + 1, col - 1);
      var br = byteAt(row + 1, col + 1);

      var ud = (ul == S && br == M) || (ul == M && br == S);
      var du = (bl == S && ur == M) || (bl == M && ur == S);

      return ud && du ? 1 : 0;
    }

    return 0;
  }

  private int countMatches(int row, int col, byte[] needle) {
    int[] counters = new int[]{1, 1, 1, 1}; // directions: {/, -, \, |}
    for (int offset = needle.length - 1; offset > 0; offset--) {
      int expectedCount = needle.length - offset;
      if (
          Arrays.stream(counters).noneMatch(
              counter -> counter == expectedCount
          )
      ) {
        break;
      }

      var expectedByte = needle[offset];
      int matches = 0;
      // check vertical
      if (
          rows > row + offset &&
              counters[3] == expectedCount &&
              byteAt(row + offset, col) == expectedByte
      ) {
        //System.out.print("|");
        counters[3]++;
        matches++;
      }
      if (columns > col + offset) {
        if (
            rows > row + offset &&
                counters[2] == expectedCount &&
                byteAt(row + offset, col + offset) == expectedByte
        ) {
          //System.out.print("\\");
          counters[2]++;
          matches++;
        }

        if (
            counters[1] == expectedCount &&
                byteAt(row, col + offset) == expectedByte
        ) {
          //System.out.print("-");
          counters[1]++;
          matches++;
        }

        if (
            row - offset >= 0 &&
                counters[0] == expectedCount &&
                byteAt(row - offset, col + offset) == expectedByte
        ) {
          //System.out.print("/");
          counters[0]++;
          matches++;
        }
      }

      // early return
      if (matches == 0) {
        //System.out.print(" >> No matches at offset " + offset);
        break;
      }
    }

    var matches = 0;
    for (int counter : counters) {
      if (counter == needle.length) {
        matches++;
      }
    }
    //System.out.println(" >> counters: " + Arrays.toString(counters));
    return matches;
  }

  private byte byteAt(int row, int col) {
    return letters[row][col];
  }

  @Override
  public String toString() {
    var joiner = new StringJoiner("\n");
    for (int i = 0; i < rows; i++) {
      joiner.add(new String(letters[i]));
    }
    return joiner.toString();
  }
}

public void main(String[] args) throws IOException {
  var input = Input.from(getClass(), args);

  try (var trace = new Trace()) {
    var parse = Parse.start();
    var haystack = Haystack.of(Files.readAllBytes(input.path()));
    parse.commit();

    int part1 = 0;
    int part2 = 0;

    for (int row = 0; row < haystack.rows(); row++) {
      for (int col = 0; col < haystack.columns(); col++) {
        var p1 = Compute.start(Part1);
        part1 += haystack.countWordsAt(row, col);
        p1.commit();

        var p2 = Compute.start(Part2);
        part2 += haystack.countXMasAt(row, col);
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
