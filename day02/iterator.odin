package day02

Invalid_Id_Iterator :: struct {
    start: uint,
    end:   uint,
    m:     uint,
    kmax:  uint,
    k:     uint,
    x:     uint,
    xend:  uint,
    c:     uint,
}

generate_invalid_ids_between :: proc(start, end, m: uint) -> Invalid_Id_Iterator {
    kmax := num_digits(end)
    return Invalid_Id_Iterator{start = start, end = end, m = m, kmax = kmax, k = 1, x = 0, xend = 0, c = 0}
}

next_invalid_id :: proc(it: ^Invalid_Id_Iterator) -> (id: uint, ok: bool) {
    for it.k <= it.kmax {
        if it.m * it.k > it.kmax {
            return 0, false
        }

        if it.c == 0 {
            it.c = geometric_sum(it.m, it.k)

            xstart := max(it.start / it.c, pow10(it.k - 1))
            it.xend = min(it.end / it.c, pow10(it.k) - 1)

            it.x = xstart
        }

        for it.x <= it.xend {
            n := it.x * it.c
            it.x += 1

            if it.start <= n && n <= it.end {
                return n, true
            }
        }

        it.k += 1
        it.c = 0
    }

    return 0, false
}
