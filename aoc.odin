package aoc

import "core:flags"
import "core:fmt"
import "core:os"
import "core:time"

import "day01"
import "day02"

Solution :: proc(str_ptr: ^string) -> (val1, val2: u64)
SOLUTIONS :: [13]Solution {
    1 = day01.day01,
}

run :: proc(day: int, solution: Solution) {
    filename := fmt.tprintf("day%02d.txt", day)
    data, err := os.read_entire_file(filename, context.allocator)
    if err != os.ERROR_NONE {
        fmt.printf("Faili %s ei leitud või ei saanud lugeda.\n", filename)
        return
    }
    defer delete(data)

    input := string(data)
    start := time.tick_now()
    part1, part2 := solution(&input)
    duration := time.tick_since(start)

    fmt.printfln("  Part 1: %d", part1)
    fmt.printfln("  Part 2: %d", part2)
    fmt.printfln("  Completed in %d", duration)
}

main :: proc() {
    Options :: struct {
        verbose: bool `usage:"Show verbose output."`,
        day:     int `usage:"Run single day"`,
        file:    ^os.File `usage:"Input file to run against."`,
    }

    opt: Options
    style: flags.Parsing_Style = .Unix
    flags.parse_or_exit(&opt, os.args, style)

    // yeah
    fmt.println("AoC 2025 (Odin)")

    switch opt.day {
    case 0:
        fmt.println("Running all days")
        for solution, day in SOLUTIONS {
            if day == 0 || solution == nil {
                continue
            }
            if day > 1 {
                fmt.println("---")
            }

            fmt.printfln("Day %02d:", day)
            run(day, solution)
        }
    case 1 ..= 12:
        solutions := SOLUTIONS
        if solutions[opt.day] == nil {
            fmt.eprintfln("ERROR: Day %d of AoC 2025 is not implemented yet!", opt.day)
            os.exit(1)
        }

        fmt.printfln("Day %02d:", opt.day)
        run(opt.day, solutions[opt.day])
    case:
        fmt.eprintfln("ERROR: AoC 2025 does not have day %d!", opt.day)
        os.exit(1)
    }
}
