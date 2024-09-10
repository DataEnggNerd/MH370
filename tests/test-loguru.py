from loguru import logger

logger.remove()
logger.add("loguru_testing.log")

def logger_test_function():
    for i in range(100):
        logger.info("Info level log")
        logger.error("Error level log")
        logger.warning("Warn level log")
        logger.debug("Debug level log")
        i+=1

if __name__ == "__main__":
    import timeit
    print(timeit.timeit("logger_test_function()", setup="from __main__ import logger_test_function", number=10000))