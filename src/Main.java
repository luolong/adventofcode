import java.util.concurrent.TimeUnit;
import solution.*;

class NoInputFileExcpetion extends RuntimeException {

    public NoInputFileExcpetion(int dayNumber) {
        super("No input for day " + dayNumber + "!");
    }
}

void printUsage() {
    System.out.println("Usage: java Main [options] <day number(s)>");
    System.out.println("Options:");
    System.out.println("  -h, --help      Display this help message");
    System.out.println("  -T, --test      Run tests");
}

static final Object[] SOLUTIONS = {
    new Day01(),
    new Day02(),
    new Day03(),
    new Day04(),
    new Day05(),
    new Day06(),
    new Day07(),
    new Day08(),
    null, //new Day09(),
    null, //new Day10(),
    null, //new Day11(),
    null, //new Day12(),
};

String readAsString(Path path) {
    try {
        return new String(Files.readAllBytes(path), StandardCharsets.UTF_8);
    } catch (IOException e) {
        throw new RuntimeException(e);
    }
}

Optional<String> readTestInput(int dayNumber) {
    return Optional.of("day%02d_test.txt")
        .map(f -> Path.of(String.format(f, dayNumber)))
        .filter(Files::exists)
        .map(path -> readAsString(path));
}

Optional<String> readTestOutput(int dayNumber) {
    return Optional.of("day%02d_expected.txt")
        .map(f -> Path.of(String.format(f, dayNumber)))
        .filter(Files::exists)
        .map(path -> readAsString(path));
}

Optional<String> readInput(int dayNumber) {
    return Optional.of("day%02d.txt")
        .map(f -> Path.of(String.format(f, dayNumber)))
        .filter(Files::exists)
        .map(path -> readAsString(path));
}

String humanTime(long nanoseconds) {
    String out = "";
    var days = TimeUnit.NANOSECONDS.toDays(nanoseconds);
    if (days > 0) out += String.format("%d d ", days);
    nanoseconds -= TimeUnit.DAYS.toNanos(days);

    var hours = TimeUnit.NANOSECONDS.toHours(nanoseconds);
    if (hours > 0) out += String.format("%d h ", hours);
    nanoseconds -= TimeUnit.HOURS.toNanos(hours);

    var minutes = TimeUnit.NANOSECONDS.toMinutes(nanoseconds);
    if (minutes > 0) out += String.format("%d min ", minutes);
    nanoseconds -= TimeUnit.MINUTES.toNanos(minutes);

    var seconds = TimeUnit.NANOSECONDS.toSeconds(nanoseconds);
    if (seconds > 0) out += String.format("%d s ", seconds);
    nanoseconds -= TimeUnit.SECONDS.toNanos(seconds);

    var milliseconds = TimeUnit.NANOSECONDS.toMillis(nanoseconds);
    if (milliseconds > 0) out += String.format("%d ms ", milliseconds);
    nanoseconds -= TimeUnit.MILLISECONDS.toNanos(milliseconds);

    var microseconds = TimeUnit.NANOSECONDS.toMicros(nanoseconds);
    if (microseconds > 0) out += String.format("%d µs ", microseconds);
    nanoseconds -= TimeUnit.MICROSECONDS.toNanos(microseconds);

    if (nanoseconds > 0) {
        out += String.format("%d ns", nanoseconds);
    }
    return out;
}

void runDay(boolean test, int dayNumber)
    throws AssertionError, NoInputFileExcpetion, IndexOutOfBoundsException, UnsupportedOperationException {
    Objects.checkIndex(dayNumber - 1, SOLUTIONS.length);

    var implementation = SOLUTIONS[dayNumber - 1];
    if (implementation == null) {
        throw new UnsupportedOperationException(
            "Day " + dayNumber + " not implemented!"
        );
    }

    if (test && implementation instanceof Test suite) {
        suite.test();
    }

    var maybeInput = test
        ? readTestInput(dayNumber)
        : readInput(dayNumber).or(() -> readTestInput(dayNumber));

    var input = maybeInput.orElseThrow(() ->
        new NoInputFileExcpetion(dayNumber)
    );

    var expectedOutput = test && maybeInput.isPresent()
        ? readTestOutput(dayNumber).stream().flatMap(String::lines).iterator()
        : Stream.empty().iterator();

    if (implementation instanceof Part1 solve) {
        var start = System.nanoTime();
        var answer = solve.part1(input);
        var end = System.nanoTime();
        IO.println(
            String.format(
                "Day %d Part 1: %s (finished in %s)",
                dayNumber,
                answer,
                humanTime(end - start)
            )
        );

        if (expectedOutput.hasNext()) {
            var expected = expectedOutput.next();
            if (!answer.equals(expected)) {
                IO.println("‼️ Expected: " + expected);
            }
        }
    }

    if (implementation instanceof Part2 solve) {
        var start = System.nanoTime();
        var answer = solve.part2(input);
        var end = System.nanoTime();
        IO.println(
            String.format(
                "Day %d Part 2: %s (finished in %s)",
                dayNumber,
                answer,
                humanTime(end - start)
            )
        );
    }
}

void main(String[] args) {
    if (args.length == 0) {
        printUsage();
        System.exit(1);
    }

    boolean test = false;

    var arguments = List.of(args);
    var it = arguments.listIterator();

    parseOptions: while (it.hasNext()) {
        switch (it.next()) {
            case "-h", "--help" -> {
                printUsage();
                System.exit(0);
            }
            case "-t", "--test" -> {
                IO.println("Running tests...");
                test = true;
            }
            case String s when s.startsWith("-") -> {
                IO.println("Invalid option: " + s);
                printUsage();
                System.exit(1);
            }
            default -> {
                it.previous();
                break parseOptions;
            }
        }
    }

    if (!it.hasNext()) {
        IO.println("Missing day number!");
        printUsage();
        System.exit(1);
    }

    while (it.hasNext()) {
        String day = it.next();
        if (!day.matches("\\d+")) {
            IO.println("Invalid day: " + day);
            printUsage();
            System.exit(1);
        }
        int dayNumber = Integer.parseInt(day);
        if (dayNumber < 1 || dayNumber > 25) {
            IO.println("Day must be between 1 and 25: " + day);
            printUsage();
            System.exit(1);
        }

        try {
            runDay(test, dayNumber);
        } catch (AssertionError | RuntimeException e) {
            IO.println("ERROR [" + dayNumber + "]: " + e.getMessage());
            e.printStackTrace();
        }
    }
}
