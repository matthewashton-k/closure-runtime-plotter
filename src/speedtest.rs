use std::fmt::format;
use std::time::Instant;
use anyhow::Error;
use rerun::{Position2D, BarChart, LineStrips2D, SeriesLine};
use std::hint::black_box;
use std::marker::PhantomData;
use std::mem::transmute;

pub struct SpeedTest<U: Clone, T: FnMut(u128, &mut U)+Copy> {
    execute: T,
    plot: Vec<(u128, u128)>,
    _marker: PhantomData<U>,
    rec: rerun::RecordingStream
}
impl<U: Clone, T: FnMut(u128, &mut U) +Copy> SpeedTest<U, T> {
    pub fn new(execute: T, rec: rerun::RecordingStream) -> Self {
        Self{
            execute,
            plot: vec![],
            _marker: PhantomData,
            rec
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
        self.rec.log_static("chart", &SeriesLine::new().with_name("Runtime").with_color([255, 0, 0]));
        for i in 0..iterations {
            // execute eval_before to get an object
            let mut pre_time_val = eval_before(nstart+(i*nincrements));

            // start timer
            let start_time = Instant::now();
            // call the execute closure and pass it the pre time val as well as the
            for _ in 0..noise_reduction {
                black_box((self.execute)((nstart+(i*nincrements)), &mut pre_time_val));
            };

            let end = start_time.elapsed().as_nanos();
            let (x,y)= (nstart+(i*nincrements),((end))/noise_reduction);
            self.plot.push((x,y));
            self.rec.log("chart", &rerun::Scalar::new(y as f64));
            // I included a println because its nice to see the progress of the run
            println!("{}\t{}",nstart+(i*nincrements),((end))/noise_reduction);

        }
        println!("{}",self.to_string());
    }
    pub fn apply_moving_average(&mut self, window_size: usize) {
        self.plot = self.plot.windows(window_size)
            .map(|window| {
                let sum: u128 = window.iter().map(|&(_, y)| y).sum();
                let avg = sum / window_size as u128;
                (window[0].0, avg)
            })
            .collect();
    }
    fn to_string(&self) -> String {
        let mut returnme = String::new();
        for p in &self.plot {
            returnme.push_str(&format!("{}\t{}\n",p.0,p.1));
        }
        returnme
    }
}
