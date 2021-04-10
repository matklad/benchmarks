#[no_mangle]
#[inline(never)]
pub fn skewed_sum(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_mul(i) ^ i;
    }
    sum
}
