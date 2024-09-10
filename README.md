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

#### 1000 iterations
| Package| time taken|
|---|---|
| without any logger |0.01s |
| logging| 21.14s|
|loguru|154.53s|
|structlog | 62.62s |

Without a doubt every logger has it's own set of overhead in the code performance. Let's see whether I can improve this. 