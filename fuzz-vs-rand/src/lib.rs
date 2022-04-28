pub fn panicky(x: u64) -> u64 {
    x + 1000
}

#[test]
fn rand_test() {
    use rand::Rng;

    let mut r = rand::thread_rng();
    loop {
        let x = r.gen();
        panicky(x);
    }
}

#[test]
fn manual_test() {
    panicky(u64::MAX - 10);
}
