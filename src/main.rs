// run  := cargo build && bunzip2 -kc ../traces/fp_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/debug/bp --gshare 13
// run  := cargo build && bunzip2 -kc ../traces/fp_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/debug/bp --custom
// run  := cargo build && bunzip2 -kc ../traces/fp_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/debug/bp --tage
// run  := cargo build && bunzip2 -kc ../traces/fp_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/debug/bp --tournament 9 10 10
// run  := cargo build && bunzip2 -kc ../traces/fp_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/debug/bp --static
// run  := cargo build && /usr/bin/cat data.in | /home/jyh/ucsd/sp25/cse240A/bp/target/debug/bp --static
// dir  := .
// kid  :=

use clap::{ArgGroup, Parser};
mod predictors;
use predictors::{BPGShare, BPStatic, BPTournament, BPCustom, BranchPredictor};
use std::io::{self, Read};

const VERBOSE: bool = false;

#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("predictor")
        .args(["bp_static", "gshare", "tournament", "gskew", "tage"])
        .multiple(false)
        .required(false)
))]
struct Cli {
    #[arg(long = "static", group = "predictor")]
    bp_static: bool,

    #[arg(long, num_args = 1, group = "predictor")]
    gshare: Option<usize>,

    #[arg(long, num_args = 3, group = "predictor")]
    tournament: Option<Vec<usize>>,

    #[arg(long, group = "predictor")]
    gskew: bool,

    #[arg(long, group = "predictor")]
    tage: bool,

    #[arg(long, group = "predictor")]
    custom: bool,
}

fn build(cli: Cli) -> Box<dyn BranchPredictor> {
    if let Some(v) = cli.gshare {
        Box::new(BPGShare::new(v, VERBOSE))
    }
    else if let Some(v) = cli.tournament {
        Box::new(BPTournament::new(v[0], v[1], v[2]))
    }
    else if cli.custom {
        Box::new(BPCustom::new(VERBOSE))
    }
    else {
        Box::new(BPStatic::new())
    }
}

fn main() {
    let cli = Cli::parse();
    let mut bp = build(cli);

    let mut buf_stdin = String::new();
    io::stdin().read_to_string(&mut buf_stdin).unwrap();
    let addrs = buf_stdin
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            (
                usize::from_str_radix(chunk[0].trim_start_matches("0x"), 16).unwrap(),
                chunk[1].parse::<u8>().unwrap() == 1,
            )
        })
        .collect::<Vec<(usize, bool)>>();

    let mispred = addrs
        .iter()
        .enumerate()
        .filter_map(|(i, (addr, outcome))| {
            let pred = bp.predict(*addr);
            bp.update(i, *addr, *outcome, pred);
            if pred != *outcome {
                Some((addr, outcome, pred))
            }
            else {
                None
            }
        })
        .count();

    println!("Branches:        {:>10}", addrs.len());
    println!("Incorrect:       {:>10}", mispred);
    println!(
        "Misprediction Rate: {:>7.3}",
        100.0 * mispred as f64 / addrs.len() as f64
    );
    // println!("{:.3}", 100.0 * mispred as f64 / addrs.len() as f64);
}
