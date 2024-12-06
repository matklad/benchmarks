If keys are pareto-distributed, can copying to a small hash map be competitive with a big hash map?

```
λ ./zig/zig run -OReleaseFast main.zig
big   size = 12500000KiB
small size = 1000Kib
big hash map
elapsed=274.176ms checksum=1314929355749
elapsed=192.381ms checksum=1314929355749
elapsed=202.44ms checksum=1314929355749
elapsed=210.108ms checksum=1314929355749
elapsed=235.042ms checksum=1314929355749
big hash map / small hash map
elapsed=212.691ms checksum=1314929355749
elapsed=209.148ms checksum=1314929355749
elapsed=189.581ms checksum=1314929355749
elapsed=237.615ms checksum=1314929355749
elapsed=209.686ms checksum=1314929355749
```
