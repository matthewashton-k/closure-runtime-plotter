mod disjointset;
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

    // eval_before is meant to be ran before the closure that needs to be benchmarked, it is used to create data structures that are needed for the actual benchmark
    // you dont want to benchmark the overhead of creating the object and adding random values, etc so any setup you have to do before benchmarking is done in this closure
    // n is a number passed in for you to keep track of the current n that is being benchmarked
    let eval_before = |n: u128| -> DisjointSetHashMap<u128> {
        let mut djs = DisjointSetHashMap::new(); // create new disjoint set
        let mut elements = vec![];
        // populate the disjointset with random numbers
        for i in 0..n {
            let r = thread_rng().gen_range(0..n);
            djs.make_set(r);
            elements.push(r);
        }
        let mut rng = thread_rng();

        // create random unions between items in the disjoint set
        for i in 0..n/3 {
            djs.union(*elements.get(rng.gen_range(0..elements.len())).unwrap(), *elements.get(rng.gen_range(0..elements.len())).unwrap());
        }
        return djs;
    };
    // this closure adds unions between two lists of length n containing random indexes
    // as you can see, a mutable disjointset is passed into this closure.
    // that disjointset is created by the eval_before closure
    let djs_union = |n:u128, djs: &mut DisjointSetHashMap<u128>| {
        black_box(djs.union(thread_rng().gen_range(0..n), thread_rng().gen_range(0..n)));
    };

    // create new speedtest struct and pass it the closure to execute
    let mut djs_union_benchmark = SpeedTest::new(
        djs_union,
        rec
    );
    djs_union_benchmark.apply_moving_average(20);
    // test the speed
    djs_union_benchmark.test_speed(2000, 100, 1000, 5000, eval_before);
     
    Ok(())
}
