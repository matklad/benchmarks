Verifying when page faulting actually happens between mmap, mlockall, and actually poking the memory:

```
λ build-exe -OReleaseSafe main.zig

λ ./main
mmap 16GiB:      4.138us
mlockall:        55ns
page touching:   5.141s
gpa alloc:       5.919s

λ ./main --lock-current
mmap 16GiB:      15.478us
mlockall:        2.88s
page touching:   65.691ms
gpa alloc:       6.17s

λ ./main --lock-current --lock-future
mmap 16GiB:      4.047us
mlockall:        2.877s
page touching:   65.629ms
gpa alloc:       5.097s

λ ./main --lock-current --lock-on-fault
mmap 16GiB:      3.79us
mlockall:        33.263us
page touching:   4.928s
gpa alloc:       6.104s
```
