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

#### 10,000 iterations
| Package| time taken|
|---|---|
| without any logger |0.01s |
| logging| 21.14s|
|loguru|154.53s|
|structlog | 62.62s |

Without a doubt every logger has it's own set of overhead in the code performance. Let's see whether I can improve this. 

---

### Lo3

I know, bad naming. But primary version of unoptimised logger written with Rust. 

#### Primliminary scope
- Adds log to file
- Provides info, warn, debug and error methods to log. 

#### Performance
- 1000 iterations - 14.24s
- 10,000 iterations - 161.64s

Not as expected, but have to explore opportunities in optimising rust code, as the current implementation is very crude. 