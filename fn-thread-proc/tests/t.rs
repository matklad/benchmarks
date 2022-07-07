use std::{path::Path, time::Instant};

#[test]
fn bench_all() {
    let n = 10_000;
    let t = Instant::now();
    let mut res = 0i32;
    for _ in 0..n {
        let x = fn_thread_proc::f();
        res = res.wrapping_add(x);
    }
    eprintln!("function {:0.2?}: {}", t.elapsed(), res);

    let t = Instant::now();
    let pool = threadpool::ThreadPool::new(4);
    let mut res = 0i32;
    for _ in 0..n {
        let x = {
            let (sender, receiver) = std::sync::mpsc::channel();
            pool.execute(move || sender.send(fn_thread_proc::f()).unwrap());
            receiver.recv().unwrap()
        };
        res = res.wrapping_add(x);
    }
    eprintln!("pool     {:0.2?}: {}", t.elapsed(), res);

    let t = Instant::now();
    let mut res = 0i32;
    for _ in 0..n {
        let x = std::thread::spawn(fn_thread_proc::f).join().unwrap();
        res = res.wrapping_add(x);
    }
    eprintln!("thread   {:0.2?}: {}", t.elapsed(), res);

    let t = Instant::now();
    let mut res = 0i32;
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("in.txt");
    for _ in 0..n {
        std::fs::write(&path, "92\n").unwrap();
        let x = std::fs::read_to_string(&path)
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        res = res.wrapping_add(x);
    }
    eprintln!("disk     {:0.2?}: {}", t.elapsed(), res);

    let t = Instant::now();
    let mut res = 0i32;
    for _ in 0..n {
        let x = std::process::Command::new(env!("CARGO_BIN_EXE_fn-thread-proc"))
            .status()
            .unwrap()
            .code()
            .unwrap();
        res = res.wrapping_add(x);
    }
    eprintln!("process  {:0.2?}: {}", t.elapsed(), res);
}
