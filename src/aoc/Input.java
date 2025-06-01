package aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public record Input(String day, Path path) {
    public static Input from(Class<?> context, String[] args)
        throws IOException {
        var day = context.getSimpleName().toLowerCase();
        var displayName =
            "Day %d".formatted(Integer.parseInt(day.substring(3)));
        return new Input(displayName, getInputPath(day, args));
    }

    public static Path getInputPath(String dayName, String[] args)
        throws IOException {
        for (var arg : args) {
            Path path = Path.of(arg);
            if (Files.exists(path) && Files.isRegularFile(path)) {
                return path;
            }
        }

        var candidates = List.of(
            dayName + ".txt",
            dayName + "test.txt",
            "input.txt",
            "test.txt"
        );

        for (var candidate : candidates) {
            var path = Path.of(candidate);
            if (Files.exists(path) && Files.isRegularFile(path)) {
                return path;
            }
        }

        throw new IOException(
            "Could not find puzzle input for " + dayName + "!"
        );
    }
}
