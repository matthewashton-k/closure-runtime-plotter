use std::fmt::format;
use std::time::Instant;
use anyhow::Error;
use std::hint::black_box;
use std::marker::PhantomData;
use std::mem::transmute;

pub struct SpeedTest<U: Clone, T: FnMut(u128, U)+Copy> {
    execute: T,
    plot: Vec<(u128, u128)>,
    _marker: PhantomData<U>
}
impl<U: Clone, T: FnMut(u128, U) +Copy> SpeedTest<U, T> {
    pub fn new(execute: T) -> Self {
        Self{
            execute,
            plot: vec![],
            _marker: PhantomData
        }
    }

    pub fn get_plot(&self) -> &Vec<(u128, u128)> {
        return &self.plot;
    }

    /// whatever is returned from eval_before is passed as the second parameter to the closure
    /// nstart is the starting value of n to benchmark
    /// nincrements is by how much you want n to increment by in each iteration of the benchmark
    /// iterations is the number of times you want to loop (and the number of data points you get)
    /// noise reduction is the number of times you want self.execute to run and be timed and averaged out to hopefully reduce noise
    /// eval before is passed in the current value of n being tested, and can be used to compute or create anything needed to setup your benchmark
    /// for example, if you want to test the time complexity of removing elements from a vector, you can use eval_before to create a vector of n size, then that vector is passed into self.execute, where you can call pop() n different times
    pub fn test_speed<V: FnMut(u128) -> U>(&mut self, nstart: u128, nincrements: u128, iterations: u128, noise_reduction: u128, mut eval_before: V){
        self.plot.clear();
        for i in 0..iterations {
            // execute eval_before to get an object
            let pre_time_val = eval_before(nstart+(i*nincrements));

            // start timer
            let start_time = Instant::now();
            // call the execute closure and pass it the pre time val as well as the
            for _ in 0..noise_reduction {
                black_box(self.execute)(black_box(nstart+(i*nincrements)), black_box(pre_time_val.clone()));
            };
            let middle_time = start_time.elapsed().as_nanos(); // keep track of how long the loop above this took

            // this loop just calls clone on pre_time_vall noise_reduction times, then that time will be subtracted from the total, that corrects for the error of accidentally timing .clone() a bunch of times in the loop above this
            // yes I thought about useing an Rc<Refcell<U>> to not have to clone every time, but the problem with that is refcell enforces borrow rules at runtime and because of that it wouldnt actually be any better for benchmarking than just a simple clone
            // also I thought about passing in a raw pointer to evade borrow checking rules but that also seemed more complicated and unsafe than just doing this
            for _  in 0..noise_reduction {
                black_box(pre_time_val.clone());
            }
            let end = start_time.elapsed().as_nanos();
            self.plot.push((nstart+(i*nincrements),((end)-(end-middle_time))/noise_reduction));

            // I included a println because its nice to see the progress of the run
            println!("{}\t{}",nstart+(i*nincrements),((end)-(end-middle_time))/noise_reduction);

        }
        println!("{}",self.to_string());
    }
    fn to_string(&self) -> String {
        let mut returnme = String::new();
        for p in &self.plot {
            returnme.push_str(&format!("{}\t{}\n",p.0,p.1));
        }
        returnme
    }
}
