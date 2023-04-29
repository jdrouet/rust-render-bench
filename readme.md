# Rust render benchmark

I was wondering, for a website doing server side rendered pages, what was the fastest, so I implemented a benchmark.

```
cargo bench
```

I get the following result

```
     Running benches/concat.rs (target/release/deps/concat-020b7bfca056be6e)
concat async/10         time:   [942.29 ns 947.00 ns 951.72 ns]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

concat sync/10          time:   [960.30 ns 968.58 ns 977.06 ns]

concat async/1000       time:   [61.104 µs 61.306 µs 61.511 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

concat sync/1000        time:   [61.685 µs 61.849 µs 62.007 µs]
Found 17 outliers among 100 measurements (17.00%)
  11 (11.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

     Running benches/sailfish.rs (target/release/deps/sailfish-5d5630232a4c1c63)
sailfish async/10       time:   [671.21 ns 672.60 ns 673.88 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

sailfish sync/10        time:   [662.70 ns 665.73 ns 668.34 ns]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) low severe
  4 (4.00%) low mild
  2 (2.00%) high mild

sailfish async/1000     time:   [55.321 µs 55.385 µs 55.457 µs]
Found 19 outliers among 100 measurements (19.00%)
  9 (9.00%) high mild
  10 (10.00%) high severe

sailfish sync/1000      time:   [55.296 µs 55.366 µs 55.450 µs]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

     Running benches/yew.rs (target/release/deps/yew-d5d18861b00cf782)
yew multi/10            time:   [28.427 µs 28.481 µs 28.547 µs]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

yew local/10            time:   [22.287 µs 22.398 µs 22.512 µs]

yew multi/1000          time:   [2.0290 ms 2.0317 ms 2.0349 ms]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

yew local/1000          time:   [2.0290 ms 2.0322 ms 2.0356 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```