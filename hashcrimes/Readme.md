```
$ rustc --version
rustc 1.70.0 (90c541806 2023-05-31)

$ pushd rust && t cargo run -r && popd
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/hashcrimes`
12762
12402
1249999975000000

real 27.04s
cpu  27.04s (25.70s user + 1.34s sys)
rss  4065.76mb

$ go version
go version go1.20.5 linux/amd64

$ pushd go && t go run . && popd
13131
8303
1249999975000000

real 21.68s
cpu  26.00s (24.72s user + 1.28s sys)
rss  3780.03mb

$ java -version
openjdk version "19.0.2" 2023-01-17
OpenJDK Runtime Environment (build 19.0.2+7-nixos)
OpenJDK 64-Bit Server VM (build 19.0.2+7-nixos, mixed mode, sharing)

$ pushd java && t java Main.java && popd
4155
2852
1249999975000000

real 7.49s
cpu  32.63s (29.56s user + 3.07s sys)
rss  8932.71mb

$ pushd 'C#' && t dotnet run && popd
9140
3863
1249999975000000

real 13.86s
cpu  15.09s (13.49s user + 1.60s sys)
rss  5133.25mb

$ pushd rust-fx && t cargo r -r && popd
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/hashcrimes`
101007
54131
1249999975000000

real 156.88s
cpu  156.88s (155.28s user + 1.60s sys)
rss  4065.71mb
```
