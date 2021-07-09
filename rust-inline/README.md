**Q:** if the `pub` non-generic function `foo` calls a private non-generic function `bar`, is `#[inline]` only on `foo` sufficient?\
**A:** no, you need inline on `bar` as well.


```bash
13:31:29|~/projects/rust-inline/main|HEAD⚡?
λ cargo build --release && lldb target/release/main -o 'disassemble -n main::main' -o 'exit'
    Finished release [optimized + debuginfo] target(s) in 0.00s
(lldb) target create "target/release/main"
Current executable set to '/home/matklad/projects/rust-inline/main/target/release/main' (x86_64).
(lldb) disassemble -n main::main
main`main::main::h85f39f4d54c62e00:
main[0x10a70] <+0>:  sub    rsp, 0x8
main[0x10a74] <+4>:  call   qword ptr [rip + 0x334ee]
main[0x10a7a] <+10>: mov    edi, eax
main[0x10a7c] <+12>: call   qword ptr [rip + 0x334ee]
main[0x10a82] <+18>: ud2
(lldb) exit

13:31:50|~/projects/rust-inline/main|HEAD⚡?
λ cargo build --release --features 'inline-foo' && lldb target/release/main -o 'disassemble -n main::main' -o 'exit'
    Finished release [optimized + debuginfo] target(s) in 0.00s
(lldb) target create "target/release/main"
Current executable set to '/home/matklad/projects/rust-inline/main/target/release/main' (x86_64).
(lldb) disassemble -n main::main
main`main::main::hf033084b4e6bd99c:
main[0x10a70] <+0>:  sub    rsp, 0x8
main[0x10a74] <+4>:  call   0x10a60                   ; dep::foo::h395258e65a1bb365 at lib.rs:5:5
main[0x10a79] <+9>:  mov    edi, eax
main[0x10a7b] <+11>: call   qword ptr [rip + 0x334ff]
main[0x10a81] <+17>: ud2
(lldb) exit

13:32:06|~/projects/rust-inline/main|HEAD⚡?
λ cargo build --release --features 'inline-foo,inline-bar' && lldb target/release/main -o 'disassemble -n main::main' -o 'exit'
    Finished release [optimized + debuginfo] target(s) in 0.00s
(lldb) target create "target/release/main"
Current executable set to '/home/matklad/projects/rust-inline/main/target/release/main' (x86_64).
(lldb) disassemble -n main::main
main`main::main::h974c06c04f78752b:
main[0x10a50] <+0>:  sub    rsp, 0x8
main[0x10a54] <+4>:  mov    edi, 0x5c
main[0x10a59] <+9>:  call   qword ptr [rip + 0x334d9]
main[0x10a5f] <+15>: ud2
(lldb) exit
```
