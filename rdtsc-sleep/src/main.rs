fn main() {
    for i in 0..10 {
        let d = std::time::Duration::from_millis(100 * i);
        let t1 = rdtsc();
        std::thread::sleep(d);
        let t2 = rdtsc();
        eprintln!("{:?}: {}", d, t2 - t1);
    }
}

fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}
