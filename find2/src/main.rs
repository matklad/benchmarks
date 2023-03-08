fn main() {
    let mut haystack = "all the work and no play made Jack a dull boy".repeat(10_000_000);
    haystack.push('"');
    haystack.push_str("once again!");

    let t = std::time::Instant::now();
    let r_slow = find2_slow(haystack.as_bytes(), [b'"', b'\\']);
    let t_slow = t.elapsed();

    let t = std::time::Instant::now();
    let r_loop = find2_loop(haystack.as_bytes(), [b'"', b'\\']);
    let t_loop = t.elapsed();

    let t = std::time::Instant::now();
    let r_fast = find2_fast(haystack.as_bytes(), [b'"', b'\\']);
    let t_fast = t.elapsed();

    assert_eq!(r_slow, r_loop);
    assert_eq!(r_slow, r_fast);
    eprintln!("slow = {t_slow:0.2?}");
    eprintln!("loop = {t_loop:0.2?}");
    eprintln!("fast = {t_fast:0.2?}");
}

#[inline(never)]
fn find2_slow(haystack: &[u8], needle: [u8; 2]) -> Option<usize> {
    haystack
        .iter()
        .position(|&b| b == needle[0] || b == needle[1])
}

#[inline(never)]
fn find2_fast(haystack: &[u8], needle: [u8; 2]) -> Option<usize> {
    find2_chunk::<32>(haystack, needle)
}

fn find2_chunk<const N: usize>(haystack: &[u8], needle: [u8; 2]) -> Option<usize> {
    let mut chunks = haystack.chunks_exact(N);
    let chunks_len = chunks.len();
    let chunk_index = chunks
        .position(|chunk| {
            // Manually write everything to make sure there's no short circuiting.
            // This makes the code branchless and auto-vectorizable.
            let mut has_first = false;
            for i in 0..N {
                has_first |= chunk[i] == needle[0];
            };
            let mut has_second = false;
            for i in 0..N {
                has_second |= chunk[i] == needle[1];
            };
            has_first || has_second
        })
        .unwrap_or(chunks_len);
    let offset = chunk_index * N;
    haystack[offset..]
        .iter()
        .position(|&b| b == needle[0] || b == needle[1])
        .map(|it| it + offset)
}

#[inline(never)]
fn find2_loop(haystack: &[u8], needle: [u8; 2]) -> Option<usize> {
    assert!(haystack.len() % 4 == 0); // Too lazy to handle leftover.
    unsafe {
        let mut i = 0usize;
        while i < haystack.len() {
            let c = *haystack.get_unchecked(i + 0);
            if c == needle[0] || c == needle[1] { return  Some(i + 0);}

            let c = *haystack.get_unchecked(i + 1);
            if c == needle[0] || c == needle[1] { return  Some(i + 1);}

            let c = *haystack.get_unchecked(i + 2);
            if c == needle[0] || c == needle[1] { return  Some(i + 2);}

            let c = *haystack.get_unchecked(i + 3);
            if c == needle[0] || c == needle[1] { return  Some(i + 3);}

            i += 4;
        }
    }
    None
}
