fn main() -> std::io::Result<()> {
    let data = std::fs::read("./input.data")?;
    assert_eq!(data.len(), 64 * 1024);
    let mut res = 0u32;
    let attempts = 100_000;
    let t = std::time::Instant::now();
    for i in 0..attempts {
        let hash = blake3::hash(&data.as_slice());
        res = res.wrapping_add(hash.as_bytes()[i % 32] as u32);
    }

    let t = t.elapsed();
    println!(
        "fingerprint={res}\ntime ={t:0.2?}\nMiB/s={:0.2}",
        ((data.len() * attempts) as f64 / (1024.0 * 1024.0) / t.as_secs_f64()) as u32
    );
    Ok(())
}
