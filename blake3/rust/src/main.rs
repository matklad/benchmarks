fn main() -> std::io::Result<()> {
    let data = std::fs::read("./input.data")?;
    assert_eq!(data.len(), 64 * 1024 * 1024);
    let mut res = 0u32;
    let attempts = 100;
    let t = std::time::Instant::now();
    for i in 0..attempts {
        let hash: [u8; 32] = if cfg!(feature = "k12") {
            kangarootwelve_xkcp::hash(data.as_slice()).into()
        } else if cfg!(feature = "threading") {
            blake3::Hasher::new()
                .update_rayon(data.as_slice())
                .finalize()
                .into()
        } else {
            blake3::hash(data.as_slice()).into()
        };
        res = res.wrapping_add(hash[i % 32] as u32);
    }

    let t = t.elapsed();
    println!(
        "fingerprint={res}\ntime ={t:0.2?}\nMiB/s={:0.2}",
        ((data.len() * attempts) as f64 / (1024.0 * 1024.0) / t.as_secs_f64()) as u32
    );
    Ok(())
}
