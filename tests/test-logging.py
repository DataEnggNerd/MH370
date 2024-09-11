from logging import getLogger, FileHandler, DEBUG

logger = getLogger()
handler = FileHandler("builtin_logging_test.log")
handler.setLevel(DEBUG)
logger.addHandler(handler)

def logger_test_function():
    for i in range(100):
        logger.info("Info level log")
        logger.error("Error level log")
        logger.warn("Warn level log")
        logger.debug("Debug level log")
        i+=1

if __name__ == "__main__":
    import timeit
    print(timeit.timeit("logger_test_function()", setup="from __main__ import logger_test_function", number=250_000))