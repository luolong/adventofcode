package day03

import "core:strings"
import "core:strconv"


day03 :: proc(input: ^string) -> (part1: u64 = 0, part2: u64 = 0) {
	for line in strings.split_lines_iterator(input) {
		bank := string(line)
		max_joltage := 0
		for window in sliding_window_iterator(&bank, 2) {
			current_joltage := strconv.parse_int(window) or_continue
			max_joltage = max(max_joltage, current_joltage)
		}
		part1 += u64(max_joltage)
	}
	return
}
