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
|lo3 | 2.84s|

#### 10,000 iterations
| Package| time taken|
|---|---|
| without any logger |0.01s |
| logging| 21.14s|
|loguru|154.53s|
|structlog | 62.62s |
|lo3 | 28.59s|

Without a doubt every logger has it's own set of overhead in the code performance. Let's see whether I can improve this. 

---
### Lo3 v0.3.0

Optimized rust code a bit to reduce redundant unwanted operations,
- Instead of opening file for every entry, now it is being done for once, at instantiation of the Logger
- Replaced `+=` with `push_str` to append the logs
- `BufWriter` to reduce number of system calls

#### Primliminary scope (No change)
- Adds log to file
- Provides info, warn, debug and error methods to log. 

#### Performance
- 1000 iterations - 2.84s
- 10,000 iterations - 28.59s

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