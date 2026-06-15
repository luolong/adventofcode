package day02

import "core:testing"


expect_a_range :: proc(t: ^testing.T, input: string, loc := #caller_location, value_expr := #caller_expression(input)) -> Range {
	range, ok := parse_range(input)
	testing.expect(t, ok, "expected a range, but got garbage", expr = value_expr, loc = loc)
	return range
}

expect_no_invalid_ids :: proc(t: ^testing.T, input: string, loc := #caller_location, value_expr := #caller_expression(input)) {
	range := expect_a_range(t, input, loc, value_expr)

	it := generate_invalid_ids_between(range.min, range.max, 2)

	actual, ok := next_invalid_id(&it)
	testing.expect(t, !ok, "expected no values, but got at least one", expr = value_expr, loc = loc)
	testing.expect_value(t, actual, 0, loc, value_expr)
}

expect_one_invalid_id :: proc(t: ^testing.T, input: string, expected_value: uint, loc := #caller_location, value_expr := #caller_expression(input)) {
	range := expect_a_range(t, input, loc, value_expr)

	it := generate_invalid_ids_between(range.min, range.max, 2)

	actual, ok := next_invalid_id(&it)
	testing.expect(t, ok, "expected one value, but got none", expr = value_expr, loc = loc)
	testing.expect_value(t, actual, expected_value, loc, value_expr)

	end, more := next_invalid_id(&it)
	testing.expect(t, !more, "expected just one value, but got more")
}

expect_two_invalid_ids :: proc(t: ^testing.T, input: string, first_value: uint, second_value: uint, loc := #caller_location, value_expr := #caller_expression(input)) {
	range := expect_a_range(t, input, )

	it := generate_invalid_ids_between(range.min, range.max, 2)

	actual_first, first_ok := next_invalid_id(&it)
	testing.expect(t, first_ok, "expected two values, but got none", expr = value_expr, loc = loc)
	testing.expect_value(t, actual_first, first_value, loc, value_expr)

	actual_second, second_ok := next_invalid_id(&it)
	testing.expect(t, second_ok, "expected two values, but got one", expr = value_expr, loc = loc)
	testing.expect_value(t, actual_second, second_value, loc, value_expr)

	end, more := next_invalid_id(&it)
	testing.expect(t, !more, "expected two values, but got more", expr = value_expr, loc = loc)
}

expect_invalid_ids :: proc(t: ^testing.T, input: string, expected_values: []uint, loc := #caller_location, value_expr := #caller_expression(input)) {
	range := expect_a_range(t, input, loc, value_expr)

	it := generate_invalid_ids_between(range.min, range.max, 2)

    for expected, idx in expected_values {
        actual, ok := next_invalid_id(&it)
        testing.expect(t, ok, "expected ok to be true, got false", expr = value_expr, loc = loc)
        testing.expect_value(t, actual, expected, loc, value_expr)
    }

    val, more := next_invalid_id(&it)
    testing.expect(t, !more, "expected no more values, but got more", expr = value_expr, loc = loc)
    testing.expect_value(t, val, 0, loc, value_expr)
}


@(test)
// 11-22 has two invalid IDs, 11 and 22.
part1_range_11_to_12_has_two_invalid_ids :: proc(t: ^testing.T) {
    expect_two_invalid_ids(t, "11-22", uint(11), uint(22))
}

@(test)
// 23-33 has one invalid ID, 33.
part1_range_23_to_33_has_one_invalid_id :: proc(t: ^testing.T) {
    expect_one_invalid_id(t, "23-33", uint(33))
}

@(test)
// 95-115 has one invalid ID, 99.
part1_range_95_to_115_has_one_invalid_id :: proc(t: ^testing.T) {
    expect_one_invalid_id(t, "95-115", uint(99))
}

@(test)
// 998-1012 has one invalid ID, 1010.
part1_range_998_to_1012_has_one_invalid_id :: proc(t: ^testing.T) {
    expect_one_invalid_id(t, "998-1012", uint(1010))
}

@(test)
// 1188511880-1188511890 has one invalid ID, 1188511885.
part1_range_1188511880_to_1188511890_has_one_invalid_id :: proc(t: ^testing.T) {
    expect_one_invalid_id(t, "1188511880-1188511890", uint(1188511885))
}

@(test)
// 222220-222224 has one invalid ID, 222222.
part1_range_222220_to_222224_has_one_invalid_id :: proc(t: ^testing.T) {
    expect_one_invalid_id(t, "222220-222224", uint(222222))
}

@(test)
// 1698522-1698528 contains no invalid IDs.
part1_range_1698522_to_1698528_has_one_invalid_id :: proc(t: ^testing.T) {
    expect_no_invalid_ids(t, "1698522-1698528")
}

@(test)
// 446443-446449 has one invalid ID, 446446.
part1_range_446443_to_446449_has_one_invalid_id :: proc(t: ^testing.T) {
    expect_one_invalid_id(t, "446443-446449", uint(446446))
}

@(test)
// 38593856-38593862 has one invalid ID, 38593859.
part1_range_3859385_to_38593862_has_one_invalid_id :: proc(t: ^testing.T) {
    expect_one_invalid_id(t, "38593856-38593862", uint(38593859))
}
