# Lo3: High-Performance Python Logging with Rust (Alpha Version)

## Overview

Lo3 is an extremely fast, Rust-based logging solution for Python applications. It provides a simple Python interface while leveraging Rust's performance to deliver logging capabilities up to 80 times faster than Python's native logging module in high-volume scenarios.

**Please Note: This is an alpha version with limited features. Currently, RustPyLogger only supports logging to files. Additional features are in active development.**

## Current Features

- High-performance logging to files (up to 80x faster than Python's native logging)
- Python-friendly API with familiar log levels (debug, info, warning, error)
- Thread-safe operations
- Efficient handling of high-volume logs
- Support for additional context (kwargs) in log messages
- Minimal CPU and memory overhead

## Planned Features (In Development)

- Support for all handlers provided in [logging](https://docs.python.org/3/library/logging.handlers.html)
- Structured logging support
- Asynchronous logging
- Log rotation and archiving
- Configurable log formatting

## Installation

- via pypi:
   ```
   pip install lo3
   ```

## Usage

Here's a simple example of how to use Lo3 in its current alpha state:

```python
from lo3 import RustLogger

# Initialize the logger (currently only supports file logging)
logger = RustLogger("app.log")

# Log messages at different levels
logger.debug("This is a debug message", {"extra": "info"})
logger.info("This is an info message")
logger.warning("This is a warning")
logger.error("This is an error", {"error_code": 500})

# Ensure all logs are written
logger.flush()
```

## Performance

In our benchmarks, Lo3 showed significant performance improvements over Python's native logging module:

- Test: Writing 1,000,000 log messages (250,000 each of debug, info, warning, and error) to a file
- Lo3: ~101.39 seconds
- Python's logging module: ~522.20 seconds

This represents an ~5x(~80%) speedup in this high-volume logging scenario.

## Benchmarking

To run the benchmarks yourself:

1. clone `https://github.com/DataEnggNerd/MH370`
2. Run the provided tests script:
   ```
   python tests/test-lo3.py
   python tests/test-logging.py
   ```

## Contributing

As this is an alpha version, we welcome contributions, especially for the features currently in development. Please feel free to submit pull requests, create issues, or suggest new features.

## Limitations

- Currently only supports logging to files
- No support yet for log rotation or size limits
- Configuration options are limited in this alpha version

## Acknowledgments

- Huge shoutout to [Michael Kefeder](https://github.com/mike-kfed) for his [Bedroom builds rust](https://github.com/bedroombuilds/python2rust) series in youtube, through which I started learning Rust, via Python.
