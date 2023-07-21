```
$ rustc --version && pushd rust && t cargo r -r && popd
rustc 1.70.0 (90c541806 2023-05-31)
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/veccrimes`
1646
677
5791
50000000

real 8.47s
cpu  8.47s (7.83s user + 646.02ms sys)
rss  2900.80mb

$ go version && pushd go && t go run . && popd
go version go1.20.5 linux/amd64
2167
8312
7033
50000000

real 17.61s
cpu  20.77s (20.32s user + 447.17ms sys)
rss  2766.49mb

$ java -version && pushd java && t java Main.java && popd
openjdk version "19.0.2" 2023-01-17
OpenJDK Runtime Environment (build 19.0.2+7-nixos)
OpenJDK 64-Bit Server VM (build 19.0.2+7-nixos, mixed mode, sharing)
1420
601
6630
50000000

real 9.01s
cpu  16.40s (15.00s user + 1.40s sys)
rss  4350.33mb

$ dotnet --version && pushd 'C#' && t dotnet run && popd
7.0.305
3196
7762
8070
50000000

real 19.81s
cpu  20.38s (19.43s user + 943.08ms sys)
rss  2845.38mb
```
