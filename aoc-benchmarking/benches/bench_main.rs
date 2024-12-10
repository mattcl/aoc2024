use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use bridge_repair::BridgeRepair;
use ceres_search::CeresSearch;
use disk_fragmenter::DiskFragmenter;
use guard_gallivant::GuardGallivant;
use historian_hysteria::HistorianHysteria;
use hoof_it::HoofIt;
use mull_it_over::MullItOver;
use print_queue::PrintQueue;
use red_nosed_reports::RedNosedReports;
use resonant_collinearity::ResonantCollinearity;
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
    (
        day_003,
        "../day-003-mull-it-over/input.txt",
        MullItOver,
        "Combined"
    ),
    (
        day_004,
        "../day-004-ceres-search/input.txt",
        CeresSearch,
        "Part 1",
        "Part 2"
    ),
    (
        day_005,
        "../day-005-print-queue/input.txt",
        PrintQueue,
        "Part 1",
        "Part 2"
    ),
    (
        day_006,
        "../day-006-guard-gallivant/input.txt",
        GuardGallivant,
        "Combined"
    ),
    (
        day_007,
        "../day-007-bridge-repair/input.txt",
        BridgeRepair,
        "Combined"
    ),
    (
        day_008,
        "../day-008-resonant-collinearity/input.txt",
        ResonantCollinearity,
        "Part 1",
        "Part 2"
    ),
    (
        day_009,
        "../day-009-disk-fragmenter/input.txt",
        DiskFragmenter,
        "Combined"
    ),
    (
        day_010,
        "../day-010-hoof-it/input.txt",
        HoofIt,
        "Combined"
    ),
    // bench_marker
}
