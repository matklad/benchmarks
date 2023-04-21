fn main() -> std::io::Result<()> {
    let mut data = std::fs::read("./input.data")?;
    assert_eq!(data.len(), 64 * 1024 * 1024);

    let data = unsafe {
        let ptr = data.as_mut_ptr();
        let len = data.len() / 128;
        std::slice::from_raw_parts_mut::<[u128; 8]>(ptr.cast(), len)
    };

    let attempts = 10;
    let t = std::time::Instant::now();
    for i in 0..attempts {
        let k = i % 8;
        #[cfg(feature = "stable-sort")]
        data.sort_by_key(|it| it[k]);
        #[cfg(feature = "cached-key")]
        data.sort_by_cached_key(|it| it[k]);
        #[cfg(all(not(feature = "stable-sort"), not(feature = "cached-key")))]
        data.sort_unstable_by_key(|it| it[k]);
    }

    let t = t.elapsed();
    println!(
        "fingerprint={}\ntime = {t:0.2?}\n64MiB round = {:0.2?}",
        data[0][0],
        t / (attempts as u32)
    );
    Ok(())
}
