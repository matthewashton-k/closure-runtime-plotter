# Closure Runtime Plotter tool


### Features

* Creating a graphical representation of how long it took a closure to run with increasing values of n over a certain range
* Evaluating "setup" closures before the closure you want to benchmark, allowing for easier and more usable data
* This project also provides a DisJointSet struct that is there to use as an example of how to benchmark a library or a data structure

### Usage Example

Let's say you want to find the runtime complexity of the get() method on a hashmap.
This tool allows you to make a graph with n (number of elements in the vector) on the x axis,
and runtime on the y axis. 

First create a closure like this:

```rust
let eval_before_hash_get = |n: u128| -> HashMap<String,u128> {
    let mut hashmap = HashMap::new();
    for i in 0..n {
        hashmap.insert(format!("index{}",i),i);
    }
    return hashmap;
};
```

The value returned by that closure will be passed to another closure, which will actually be executed and benchmarked:

```rust
let benchmark_get =  |n:u128, vector: &mut HashMap<String,u128>| {
    black_box(vector.get(&format!("index{}",n-100)));
};
```

That closure takes a value of n, and a hashmap with n different elements (which is created by the eval_before closure)
and calls get on a hashmap of n size. 
the black_box method prevents the compiler from doing certain optimizations which can obscure the actual runtime complexity of your algorithm sometimes.


then use this code to benchmark and plot the points:
```rust
let mut hashmap_get = SpeedTest::new(benchmark_get);
hashmap_get.test_speed(100, 100, 100, 5000000, eval_before_hash_get);
let plotter = plotter::Plotter::new(hashmap_get.get_plot());
plotter.generate_image("hash_get.png");
println!("finished plotting hashmap get runtimes");
```


as you can see it makes a pretty graph with not too much noise! ![hashmap get plot](https://gitlab.com/matthewashton_k/closure-runtime-plotter/-/raw/master/hash_get.png?ref_type=heads&inline=false)


