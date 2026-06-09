package day01

import "core:strconv"
import "core:strings"

Direction :: enum {
    L = -1,
    R = 1,
}

Rotation :: struct {
    dir:      Direction,
    distance: uint,
}

rotation_parse :: proc(input: string) -> (Rotation, bool) {
    // Kontroll, et string poleks tühi või liiga lühike
    if len(input) < 2 do return {}, false

    dir: Direction
    switch input[0] {
    case 'L':
        dir = .L
    case 'R':
        dir = .R
    case:
        return {}, false // Vigane suund
    }

    distance, ok := strconv.parse_uint(input[1:])
    if !ok do return {}, false

    return Rotation{dir, distance}, true
}
