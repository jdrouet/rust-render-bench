# Rust render benchmark

I was wondering, for a website doing server side rendered pages, what was the fastest, so I implemented a benchmark.

```
cargo bench
```

I get the following result

```
askama async/10         time:   [610.95 ns 613.29 ns 615.59 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

askama sync/10          time:   [592.76 ns 599.39 ns 605.69 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

askama async/1000       time:   [47.293 µs 47.370 µs 47.460 µs]
Found 21 outliers among 100 measurements (21.00%)
  5 (5.00%) high mild
  16 (16.00%) high severe

askama sync/1000        time:   [47.393 µs 47.466 µs 47.552 µs]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe

     Running benches/concat.rs (target/release/deps/concat-7dd34b8a00c8527b)
concat async/10         time:   [932.14 ns 934.30 ns 936.75 ns]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low severe
  2 (2.00%) high mild
  7 (7.00%) high severe

concat sync/10          time:   [937.69 ns 946.29 ns 955.55 ns]
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

concat async/1000       time:   [61.325 µs 61.691 µs 62.052 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

concat sync/1000        time:   [61.387 µs 61.650 µs 61.922 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

     Running benches/sailfish.rs (target/release/deps/sailfish-5db016be27ab1b3e)
sailfish async/10       time:   [669.14 ns 672.95 ns 676.88 ns]

sailfish sync/10        time:   [660.53 ns 661.52 ns 662.51 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

sailfish async/1000     time:   [55.480 µs 55.630 µs 55.802 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

sailfish sync/1000      time:   [55.484 µs 55.599 µs 55.737 µs]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

     Running benches/yew.rs (target/release/deps/yew-da7471e2e3e8a052)
yew multi/10            time:   [28.130 µs 28.510 µs 28.927 µs]
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe

yew local/10            time:   [22.480 µs 22.592 µs 22.713 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

yew multi/1000          time:   [2.0328 ms 2.0401 ms 2.0480 ms]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

yew local/1000          time:   [2.0400 ms 2.0482 ms 2.0567 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```