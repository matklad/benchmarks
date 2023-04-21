# sorting in zig and rust


```console
$ rustc main.rs && ./main
```

```
Rust + unstable:
fingerprint=321502070426280560397710517997363019623
time = 415.60ms

Rust + cached-key
fingerprint=321502070426280560397710517997363019623
time = 447.20ms

Rust + stable:
fingerprint=321502070426280560397710517997363019623
time = 777.04ms

Zig + ReleaseFast
fingerprint=321502070426280560397710517997363019623
time  = 1.13s

Zig + ReleaseSafe
fingerprint=321502070426280560397710517997363019623
time  = 1.123s
```

```
$ rustc --version
rustc 1.70.0-beta.1 (1b7dd2252 2023-04-19)
$ zig version
0.11.0-dev.2685+fac120bc3
```
