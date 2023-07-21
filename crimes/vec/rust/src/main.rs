use std::time::Instant;

const N: u64 = 50_000_000;

fn main() {
    let mut v: Vec<String> = Vec::new();

    let t = Instant::now();
    for i in 0..N {
        v.push(i.to_string());
    }
    println!("{}", t.elapsed().as_millis());

    let t = Instant::now();
    v.sort();
    println!("{}", t.elapsed().as_millis());

    let t = Instant::now();
    let mut total: u64 = 0;
    for i in 0..N {
        total += v.binary_search(&i.to_string()).is_ok() as u64;
    }
    println!("{}", t.elapsed().as_millis());

    println!("{}", total);
}
