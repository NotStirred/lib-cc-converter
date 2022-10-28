pub const fn ceil_div(x: i64, y: i64) -> i64 {
    return -floor_div(-x, y);
}

pub const fn floor_div(x: i64, y: i64) -> i64 {
    let mut r = x / y;
    // if the signs are different and modulo not zero, round down
    if (x ^ y) < 0 && (r * y != x) {
        r -= 1;
    }
    return r;
}

pub const fn ceil_div_usize(x: usize, y: usize) -> usize {
    ceil_div(x as i64, y as i64) as usize
}

pub const fn floor_div_usize(x: usize, y: usize) -> usize {
    floor_div(x as i64, y as i64) as usize
}
