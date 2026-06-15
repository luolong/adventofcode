package day03

sliding_window_iterator :: proc(rem: ^string, window_size: int) -> (window: string, ok: bool) {
	if len(rem^) < window_size {
        return "", false
    }

     window = rem^[:window_size]
     rem^ = rem^[1:]

     return window, true
}
