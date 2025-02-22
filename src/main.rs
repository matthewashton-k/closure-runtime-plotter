mod disjointset;
mod plotter;
mod speedtest;

use crate::disjointset::DisjointSetHashMap;
use rand::{random, thread_rng, Rng};
use speedtest::SpeedTest;
use std::collections::HashMap;
use std::hint::black_box;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_app").connect_tcp()?;    
    
    let eval_before = |n| -> u128 {0};
    let quadratic = |n, before: &mut u128| {
        for i in 0..n {
            for i in 0..n {
                black_box(3+3);
            }
        }
    };
    let mut bench = SpeedTest::new(quadratic, rec);
    // test the speed
    bench.test_speed(10, 1, 300, 600, eval_before);
    Ok(())
}
