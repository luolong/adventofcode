package day01

import "core:fmt"
import "core:strings"

day01 :: proc(input: ^string) -> (part1: u64 = 0, part2: u64 = 0) {
    position: uint = 50
    zeroed: u64 = 0
    for line in strings.split_lines_iterator(input) {
        // process line
        rot, ok := rotation_parse(line)
        if !ok {
            fmt.eprintfln("ERROR: Failed to parse rotation %s", line)
            return
        }

        full_rotations := rot.distance / 100
        zeroed += u64(full_rotations)

        remaining_distance := rot.distance % 100
        switch rot.dir {
        case .L:
            zeroed += 1 if position > 0 && remaining_distance >= position else 0
            position = position + 100 - remaining_distance
        case .R:
            zeroed += 1 if position + remaining_distance >= 100 else 0
            position = position + 100 + remaining_distance
        }

        position = position % 100

        if position == 0 {
            part1 += 1
        }
    }

    part2 = zeroed
    return
}
