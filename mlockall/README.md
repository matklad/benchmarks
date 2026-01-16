Verifying when page faulting actually happens between mmap, mlockall, and actually poking the memory:

```

$./main
mmap 16GiB:      4.264us
mlockall:        56ns
page touching:   4.948s

$./main --lock-current
mmap 16GiB:      8.575us
mlockall:        2.873s
page touching:   65.776ms

$./main --lock-current --lock-future
mmap 16GiB:      3.71us
mlockall:        2.887s
page touching:   65.689ms

$./main --lock-current --lock-on-fault
mmap 16GiB:      4.313us
mlockall:        35.864us
page touching:   4.987s
```
