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
    rec: rerun::RecordingStream,
    moving_avg: Option<usize>
}
impl<U: Clone, T: FnMut(u128, &mut U) +Copy> SpeedTest<U, T> {
    pub fn new(execute: T, rec: rerun::RecordingStream) -> Self {
        Self{
            execute,
            plot: vec![],
            _marker: PhantomData,
            rec,
            moving_avg: None
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
    pub fn test_speed<V: FnMut(u128) -> U>(
        &mut self,
        nstart: u128,
        nincrements: u128,
        iterations: u128,
        noise_reduction: u128,
        mut eval_before: V,
    ) {
        self.plot.clear();
        self.rec.log_static(
            "runtimes/chart",
            &SeriesLine::new()
                .with_name("Runtime")
                .with_color([255, 0, 0]),
        );
        self.rec.log_static(
            "runtimes/chart_moving_avg",
            &SeriesLine::new()
                .with_name("Runtime Moving Avg")
                .with_color([0, 0, 255]),
        );
    
        let mut moving_avg_buffer = std::collections::VecDeque::new();
    
        for i in 0..iterations {
            // execute eval_before to get an object
            let mut pre_time_val = eval_before(nstart + (i * nincrements));
    
            // start timer
            let start_time = Instant::now();
            // call the execute closure
            for _ in 0..noise_reduction {
                black_box((self.execute)(
                    (nstart + (i * nincrements)),
                    &mut pre_time_val,
                ));
            }
    
            let end = start_time.elapsed().as_nanos();
            let (x, y) = (nstart + (i * nincrements), end / noise_reduction);
            self.plot.push((x, y));
            self.rec.log("runtimes/chart", &rerun::Scalar::new(y as f64));
    
            // Calculate and log moving average if enabled
            if let Some(window_size) = self.moving_avg {
                moving_avg_buffer.push_back(y);
                // Maintain buffer size
                while moving_avg_buffer.len() > window_size {
                    moving_avg_buffer.pop_front();
                }
                // Log average only when buffer is full
                if moving_avg_buffer.len() == window_size {
                    let avg = moving_avg_buffer.iter().sum::<u128>() / window_size as u128;
                    self.rec.log(
                        "runtimes/chart_moving_avg",
                        &rerun::Scalar::new(avg as f64),
                    );
                }
            }
        }
    }
    
    pub fn apply_moving_average(&mut self, window_size: usize) {
        self.moving_avg = Some(window_size);
        // self.plot = self.plot.windows(window_size)
        //     .map(|window| {
        //         let sum: u128 = window.iter().map(|&(_, y)| y).sum();
        //         let avg = sum / window_size as u128;
        //         (window[0].0, avg)
        //     })
        //     .collect();
    }
    fn to_string(&self) -> String {
        let mut returnme = String::new();
        for p in &self.plot {
            returnme.push_str(&format!("{}\t{}\n",p.0,p.1));
        }
        returnme
    }
}
