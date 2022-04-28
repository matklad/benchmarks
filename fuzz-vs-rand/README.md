# fuzz-vs-rand

Comparing effectiveness of coverage guided fuzzing versus pure randomized testing.

Function under test:

```rust
pub fn panicky(x: u64) -> u64 {
    x + 1000
}
```

The problem here is that it overflows if `x` is near `u64::MAX`.

To run pure randomized tests:

```console
$ cargo test --release -- rand_test
```

To run fuzzer-based tests:

```console
$ cargo install cargo-fuzz
$ cd fuzz
$ RUSTC_BOOTSTRAP=1 cargo fuzz run f
```
