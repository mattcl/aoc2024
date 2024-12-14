use std::env;

use aoc_plumbing::Problem;
use bridge_repair::BridgeRepair;
use ceres_search::CeresSearch;
use claw_contraption::ClawContraption;
use disk_fragmenter::DiskFragmenter;
use garden_groups::GardenGroups;
use guard_gallivant::GuardGallivant;
use historian_hysteria::HistorianHysteria;
use hoof_it::HoofIt;
use mull_it_over::MullItOver;
use plutonium_pebbles::PlutoniumPebbles;
use print_queue::PrintQueue;
use red_nosed_reports::RedNosedReports;
use resonant_collinearity::ResonantCollinearity;
use restroom_redoubt::RestroomRedoubt;

pub fn run() -> anyhow::Result<()> {
    let day: u8 = env::var("AOC_DAY")?.parse()?;
    let input_file = env::var("AOC_INPUT")?;
    let input = std::fs::read_to_string(&input_file)?;

    let out = match day {
        1 => serde_json::to_string(&HistorianHysteria::solve(&input)?)?,
        2 => serde_json::to_string(&RedNosedReports::solve(&input)?)?,
        3 => serde_json::to_string(&MullItOver::solve(&input)?)?,
        4 => serde_json::to_string(&CeresSearch::solve(&input)?)?,
        5 => serde_json::to_string(&PrintQueue::solve(&input)?)?,
        6 => serde_json::to_string(&GuardGallivant::solve(&input)?)?,
        7 => serde_json::to_string(&BridgeRepair::solve(&input)?)?,
        8 => serde_json::to_string(&ResonantCollinearity::solve(&input)?)?,
        9 => serde_json::to_string(&DiskFragmenter::solve(&input)?)?,
        10 => serde_json::to_string(&HoofIt::solve(&input)?)?,
        11 => serde_json::to_string(&PlutoniumPebbles::solve(&input)?)?,
        12 => serde_json::to_string(&GardenGroups::solve(&input)?)?,
        13 => serde_json::to_string(&ClawContraption::solve(&input)?)?,
        14 => serde_json::to_string(&RestroomRedoubt::solve(&input)?)?,
        _ => "\"not implemented\"".into(),
    };

    println!("{}", out);

    Ok(())
}
