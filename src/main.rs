mod plotter;
mod speedtest;
mod disjointset;

use std::collections::HashMap;
use std::hint::black_box;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rand::{random, Rng, thread_rng};
use speedtest::SpeedTest;
use crate::disjointset::DisjointSetHashMap;

fn main() {

    // this is supposed to make sure the thread is all spun up before benchmarking
    sleep(Duration::from_secs(1));

    // eval_before is meant to be ran before the closure that needs to be benchmarked, it is used to create data structures that are needed for the actual benchmark
    // you dont want to benchmark the overhead of creating the object and adding random values, etc so any setup you have to do before benchmarking is done in this closure
    // n is a number passed in for you to keep track of the current n that is being benchmarked
    let eval_before = |n: u128| -> DisjointSetHashMap<u128> {
        let mut djs = DisjointSetHashMap::new(); // create new disjoint set
        let mut elements = vec![];
        // populate the disjointset with random numbers
        for i in 0..n {
            let r = rand::random::<u128>();
            djs.make_set(r);
            elements.push(r);
        }
        let mut rng = thread_rng();

        // create random unions between items in the disjoint set
        for i in 0..n/2 {
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


    let eval_before_hash_get = |n: u128| -> HashMap<String,u128> {
        let mut hashmap = HashMap::new();
        for i in 0..n {
            hashmap.insert(format!("index{}",i),i);
        }
        return hashmap;
    };

    let benchmark_get =  |n:u128, vector: &mut HashMap<String,u128>| {
            black_box(vector.get(&format!("index{}",n-100)));
    };

    let mut hashmap_get = SpeedTest::new(benchmark_get);
    hashmap_get.test_speed(100, 100, 100, 5000000, eval_before_hash_get);
    let plotter = plotter::Plotter::new(hashmap_get.get_plot());
    plotter.generate_image("vec_pop.png");
    println!("finished plotting vector pop runtimes");

    // create new speedtest struct and pass it the closure to execute
    let mut speedtester_bigo_n = SpeedTest::new(
        djs_union
    );

    // test the speed
    speedtester_bigo_n.test_speed(1000,10000,100, 1000,eval_before);

    // generates a plot of the image and saves it to a png
    let plotter = plotter::Plotter::new(speedtester_bigo_n.get_plot());
    let generated = plotter.generate_image("djs.png");


}