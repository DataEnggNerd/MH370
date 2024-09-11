from lo3 import RustLogger

logger = RustLogger("rust_test.log")

def logger_test_function():
    for i in range(100):
        # logger.info("Info level log", {"test":"test_1", "test_2":"test_2", "test_3":"test_3", "test_4":"test_4"})
        # logger.error("Error level log", {"test":"test_1", "test_2":"test_2", "test_3":"test_3", "test_4":"test_4"})
        # logger.warning("Warn level log", {"test":"test_1", "test_2":"test_2", "test_3":"test_3", "test_4":"test_4"})
        # logger.debug("Debug level log", {"test":"test_1", "test_2":"test_2", "test_3":"test_3", "test_4":"test_4"})
        logger.info("Info level log")
        logger.error("Error level log")
        logger.warning("Warn level log")
        logger.debug("Debug level log")
        i+=1

if __name__ == "__main__":
    import timeit
    print(timeit.timeit("logger_test_function()", setup="from __main__ import logger_test_function", number=250_000))