# Another Autovectorisation benchmark

Comparing naive loop vs unrolled unsafe loop vs "use chunks_exact to nudge compiler to auto-vectorize".

```console
$ RUSTFLAGS="-C target-cpu=native" cargo r -q -r
slow = 156.40ms
loop = 112.20ms
fast = 21.17ms
```

Source: https://lobste.rs/s/9q6rnm/fast_byte_searching_with_simd_without_ton
