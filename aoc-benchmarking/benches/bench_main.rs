use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use historian_hysteria::HistorianHysteria;
use red_nosed_reports::RedNosedReports;
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
        "Combined"
    ),
    (
        day_002,
        "../day-002-red-nosed-reports/input.txt",
        RedNosedReports,
        "Combined"
    ),
    // bench_marker
}
