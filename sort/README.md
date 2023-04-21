# Blake3

Benchmarking Zig versus official Rust implementation of Blake3

```console
$ rustc main.rs && ./main
```

```
Rust + asm: # This uses assembly compiled with gcc
fingerprint=13800000
time =1.34s
MiB/s=4651

Rust:       # Pure rust
fingerprint=13800000
time =1.62s
MiB/s=3859

Zig + ReleaseFast
fingerprint=13800000
time  = 6.074s
MiB/S = 1028

Zig + ReleaseSafe
fingerprint=13800000
time  = 6.336s
MiB/S = 986
```

```console
$ lscpu | rg 'Model name'
Model name:                      12th Gen Intel(R) Core(TM) i7-12700H
$ rustc --version
rustc 1.69.0-beta.8 (f18236dcd 2023-04-13)
$ zig version
0.10.1
```
