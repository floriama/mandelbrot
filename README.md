# mandelbrot viewer

![full](screenshots/full.png)
![valley](screenshots/valley.png)

### Building
Install Rust from [the Rust website](https://www.rust-lang.org/)

To build, clone the repository and execute 

```
    cargo build --release
```

### Running
Execute 
```
    cargo run --release
```

### Controls
Use the arrow keys to move around. 'W' and 'S' zooms 
in and out of the fractal. 'I' and 'J' change the number 
of iterations to determine convergence of elements in the
set. Thus, higher numbers of iterations increase resolution 
but require more computational resources.
