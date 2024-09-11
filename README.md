# MH370
better logger for python

### Performance of existing packages

```shell
Setup:
Computer - Apple Macbook air M1
Operating system - iOS Sonama 14.6.1
Python version - 3.12.6
```

#### 1000 iterations
| Package| time taken|
|---|---|
| without any logger | 0.001s|
| logging| 2.11s|
|loguru|15.19s|
|structlog | 9.81s |
|lo3 v0.2 | 2.84s|
|lo3 v0.3 | 0.41s|

#### 10,000 iterations
| Package| time taken|
|---|---|
| without any logger |0.01s |
| logging| 21.14s|
|loguru|154.53s|
|structlog | 62.62s |
|lo3 v0.2 | 28.59s|
|lo3 v0.3 | 3.98s|

#### 250,000 iterations
| Package| time taken|
|---|---|
| logging | 522.20 |
| lo3 v3 | 101.39|

Without a doubt every logger has it's own set of overhead in the code performance. ~~Let's see whether I can improve this.~~ `RustLogger` gave a whopping ~80% performance improvement in both high and low volume testing.

---
### Lo3 v0.3.0

Optimized rust code a bit to reduce redundant unwanted operations,
- Instead of opening file for every entry, now it is being done for once, at instantiation of the Logger
- Replaced `+=` with `push_str` to append the logs
- `BufWriter` to reduce number of system calls
- built the package with `maturin develop --release`
  
#### Primliminary scope (No change)
- Adds log to file
- Provides info, warn, debug and error methods to log. 

#### Performance
- Without parameters
  - 1000 iterations - 0.41s
  - 10,000 iterations - 3.98s
- With four parameters
  - 1000 iterations - 0.95s
  - 10,000 iterations - 9.53s

Yay! :tada:! Reached the level of python native logger. Will be working on adding features and optimising code in a better way. :nerd_face:

--- 
### Lo3 v0.2.0

I know, bad naming. But primary version of unoptimised logger written with Rust. 

#### Primliminary scope
- Adds log to file
- Provides info, warn, debug and error methods to log. 

#### Performance
- 1000 iterations - 14.24s
- 10,000 iterations - 161.64s

Not as expected, but have to explore opportunities in optimising rust code, as the current implementation is very crude. 