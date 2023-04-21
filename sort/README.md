# sorting in zig and rust


```console
$ rustc main.rs && ./main
```

```
Rust + unstable:
fingerprint=321502070426280560397710517997363019623
time = 424.10ms
64MiB round = 42.41ms

Rust + cached-key
fingerprint=321502070426280560397710517997363019623
time = 456.57ms
64MiB round = 45.66ms

Rust + stable:
fingerprint=321502070426280560397710517997363019623
time = 765.45ms
64MiB round = 76.55ms

Zig + ReleaseFast
fingerprint=321502070426280560397710517997363019623
time  = 1.147s
64MiB round = 114.714ms

Zig + ReleaseSafe
fingerprint=321502070426280560397710517997363019623
time  = 1.137s
64MiB round = 113.774ms
```

```
$ rustc --version
rustc 1.70.0-beta.1 (1b7dd2252 2023-04-19)
$ zig version
0.11.0-dev.2685+fac120bc3
```
