use getrandom::getrandom;

/// Returns a random i64 in the range `[start, end)`
#[inline]
pub fn rand_range(start: usize, end: usize) -> usize {
    let mut bytes = [0u8; 8]; // Hopefully we're running on 64-bit
    getrandom(&mut bytes).unwrap_or_default();

    let rnd: usize = unsafe { std::mem::transmute(bytes) };

    rnd % (end - start) + start
}
