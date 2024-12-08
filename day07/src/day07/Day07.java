import day07.Part1;
import day07.Part2;
import helper.Input;
import helper.Timer;

import java.io.IOException;
import java.nio.file.Files;

public void main(String[] args) throws IOException {
    var inputPath = Input.getInputPath(Part1.class, args);

    var part1 = Timer.of(new Part1());
    var part2 = Timer.of(new Part2());

    try (var lines = Files.lines(inputPath)) {
        lines.forEach(s -> {
            part1.accept(s);
            part2.accept(s);
        });
    } catch (IOException e) {
        System.err.println("Failed to read file " + e.getMessage());
        System.exit(1);
    }

    System.out.println("Day 7, Part 1: " + part1);
    System.out.println("Day 7, Part 2: " + part2);
}