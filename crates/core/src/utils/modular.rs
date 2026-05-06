pub fn mod_pos(a: i32, m: i32) -> usize {
    ((a % m + m) % m) as usize
}

pub fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    let (mut old_r, mut r) = (a, m);
    let (mut old_s, mut s) = (1i32, 0i32);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }

    if old_r != 1 {
        None // no inverse exists
    } else {
        Some(((old_s % m + m) % m) as i32)
    }
}
