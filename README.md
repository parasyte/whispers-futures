# Telephonic Whispsers

Uses `futures-preview = 0.3` to implement the Telephonic Whispers concurrency example.

Background: http://thornydev.blogspot.com/2014/01/chinese-whispers-in-rust.html

# Benchmarks

_Entirely unscientific benchmarks below, take with a grain of salt._

----

The go-lang example on my machine, built with `go version go1.12.5 darwin/amd64`:

```
$ go build -o whispers main.go && time ./whispers
100001

real    0m0.320s
user    0m0.860s
sys     0m0.221s
```

----

Initial commit, built with `rustc 1.37.0-nightly (04a3dd8a8 2019-06-18)`:

```
$ cargo build --release && time ./target/release/whispers-futures
    Finished release [optimized] target(s) in 0.01s
100001

real    0m0.103s
user    0m0.083s
sys     0m0.014s
```

----

After switching to a sorted `Vec`:

```
$ cargo build --release && time ./target/release/whispers-futures
    Finished release [optimized] target(s) in 0.01s
100001

real    0m0.047s
user    0m0.034s
sys     0m0.009s
```
