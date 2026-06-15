package day02

import "core:fmt"
import "core:slice"
import "core:strings"

parse_ranges :: proc(input: ^string) -> (value: [dynamic]Range, ok: bool = false) {
    value = make([dynamic]Range)

    for slice in strings.split_iterator(input, ",") {
        range := parse_range(slice) or_return
        add_range(&value, range)
    }

    ok = true
    return
}

day02 :: proc(input: ^string) -> (part1: u64 = 0, part2: u64 = 0) {
    when DAY2_MERGE_RANGES {
        fmt.printfln("Running with DAY2_MERGE_RANGES enabled")
    }

    all_ranges, ok := parse_ranges(input)
    if !ok {
        return
    }
    defer delete(all_ranges)

    unique := make([dynamic]uint)
    defer delete(unique)

    for range in all_ranges {
        min, max := range.min, range.max
        maxlen := num_digits(max)

        for m in 2 ..= maxlen {
            it := generate_invalid_ids_between(range.min, range.max, uint(m))
            for id in next_invalid_id(&it) {
                if id <= range.max {
                    if m == 2 {
                        part1 += u64(id)
                    }
                    append(&unique, id)
                }
            }
        }
    }

    slice.sort(unique[:])
    if len(unique) > 0 {
        part2 = u64(unique[0])
        for i in 1 ..< len(unique) {
            if unique[i] != unique[i - 1] {
                part2 += u64(unique[i])
            }
        }
    }

    return
}
