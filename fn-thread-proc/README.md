Benchmark how long does it take to run a trivial computation 10_000 times when the computation is cast as a
a) function call b) computing stuff in a thread pool c) computing stuff in a freshly-spawned thread d) reading stuff from disk e) reading stuff from another process.

```console
$ cargo test --release -- --nocapture
function 9.18µs
pool     45.98ms
thread   124.64ms
disk     217.38ms
process  5.38s
```
