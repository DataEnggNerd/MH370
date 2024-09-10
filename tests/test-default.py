def logger_test_function():
    for i in range(100):
        i+=1

if __name__ == "__main__":
    import timeit
    print(timeit.timeit("logger_test_function()", setup="from __main__ import logger_test_function", number=1000))