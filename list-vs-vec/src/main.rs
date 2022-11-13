use std::{
    collections::LinkedList,
    time::{Duration, Instant},
};

fn main() {
    let n = 10u32.pow(5);
    let mut list = LinkedList::new();
    let mut vec = Vec::new();

    let mut vec_total = Duration::ZERO;
    let mut list_total = Duration::ZERO;
    for i in 0..=n {
        let t = Instant::now();
        let pos = vec.len() / 2;
        vec.insert(pos, i);
        vec_total += t.elapsed();
        if is_power_of_ten(i) {
            eprintln!("{i} vec  {vec_total:.2?}")
        }

        let t = Instant::now();
        let mut rest = list.split_off(pos);
        list.push_back(i);
        list.append(&mut rest);
        list_total += t.elapsed();
        if is_power_of_ten(i) {
            eprintln!("{i} list {list_total:.2?}")
        }
    }
    assert!(list.into_iter().eq(vec.into_iter()));
}

fn is_power_of_ten(mut n: u32) -> bool {
    if n == 0 {
        return false;
    }
    loop {
        if n == 1 {
            return true;
        }
        if n % 10 != 0 {
            return false;
        }
        n /= 10
    }
}
