# Closure Runtime Plotter tool


### Features

* Creating a graphical representation of how long it took a closure to run with increasing values of n over a certain range
* Evaluating "setup" closures before the closure you want to benchmark, allowing for easier and more usable data
* This project also provides a DisJointSet struct that is there to use as an example of how to benchmark a library or a data structure

### Usage Example

Let's say you want to find the runtime complexity of the pop() method on a vector.
This tool allows you to make a graph with n (number of elements in the vector) on the x axis,
and runtime on the y axis. 

First create a closure like this:

```rust
let eval_before_vec = |n: u128| -> Vec<u128> {
    let mut vec_of_len_n = Vec::new();
    for i in 0..n {
        vec_of_len_n.push(i);
    }
    return vec_of_len_n;
};
```

The value returned by that closure will be passed to another closure, which will actually be executed and benchmarked:

```rust
    let benchmark_pop =  |n:u128, mut vector: Vec<u128>| {
        for i in 0..n {
            black_box(vector.pop());
        }
    };
```

That closure takes a value of n, and a vector (which is created by the eval_before closure)
and calls pop n times. 
the black_box method prevents the compiler from doing certain optimizations which can obscure the actual runtime complexity of your algorithm sometimes.


then use this code to benchmark and plot the points:
```rust
    let mut speedtest_vec_pop = SpeedTest::new(benchmark_pop);
    speedtest_vec_pop.test_speed(10000,1000,100,1000, eval_before_vec);
    let plotter = plotter::Plotter::new(speedtest_vec_pop.get_plot());
    plotter.generate_image("vec_pop.png");
```


as you can see it makes a pretty graph with not too much noise! ![alt text](https://gitlab.com/matthewashton_k/closure-runtime-plotter/-/raw/master/vec_pop.png?ref_type=heads&inline=false)


