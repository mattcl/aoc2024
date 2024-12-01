use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use historian_hysteria::HistorianHysteria;
// import_marker

criterion_main! {
    benches
}

aoc_benches! {
    5,
    (
        day_001,
        "../day-001-historian-hysteria/input.txt",
        HistorianHysteria,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
