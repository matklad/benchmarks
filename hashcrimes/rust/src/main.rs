use std::{collections::HashMap, time::Instant};

const N: u64 = 50_000_000;

fn main() {
    let mut m: HashMap<String, u64> = HashMap::new();
    let t = Instant::now();
    for i in 0..N {
        m.insert(i.to_string(), i);
    }
    println!("{}", t.elapsed().as_millis());
    let t = Instant::now();
    let mut total: u64 = 0;
    for i in 0..N {
        total += m[&i.to_string()];
    }
    println!("{}", t.elapsed().as_millis());
    println!("{}", total);
}
