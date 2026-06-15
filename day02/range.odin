package day02

import "core:math"
import "core:strconv"
import "core:strings"

MERGE_RANGES :: #config(MERGE_RANGES, false)

Range :: struct {
    min: uint,
    max: uint,
}

parse_range :: proc(line: string) -> (result: Range, ok: bool) {
    parts := strings.split_n(line, "-", 2)
    if len(parts) != 2 {
        return
    }
    defer delete(parts)

    min := strconv.parse_uint(strings.trim_space(parts[0])) or_return
    max := strconv.parse_uint(strings.trim_space(parts[1])) or_return

    result = Range{min, max}
    ok = true
    return
}

is_range_overlap :: proc(r1, r2: Range) -> bool {
    return r1.min <= r2.max && r1.max >= r2.min
}

is_connected :: proc(r1, r2: Range) -> bool {
    return r1.min == r2.max + 1 || r2.min == r1.max + 1
}

is_ordered :: proc(a, b: Range) -> bool {
    return a.max <= b.max if a.min == b.min else a.min <= b.min
}

merge_ranges :: proc(r1, r2: Range) -> Range {
    min := math.min(r1.min, r2.min)
    max := math.max(r1.max, r2.max)
    return Range{min, max}
}

add_range :: proc(ranges: ^[dynamic]Range, r: Range) {
    if len(ranges) > 0 {
        for r2, i in ranges {
            when MERGE_RANGES {
                if is_range_overlap(r, r2) || is_connected(r, r2) {
                    ranges[i] = merge_ranges(r, r2)
                    return
                }
            } else {
                if is_range_overlap(r, r2) {
                    inject_at(ranges, i, r)
                    return
                }
            }
        }
    }

    append(ranges, r)
}
