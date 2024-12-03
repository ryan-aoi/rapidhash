# ruff: noqa: S324
import logging
import random
import string
from hashlib import md5, sha1, sha256
from timeit import timeit

import mmh3
import numpy as np
import pandas as pd
import seaborn as sns
from matplotlib import pyplot as plt

from rapidhash import rapidhash

sns.set_theme(style="darkgrid")
logging.basicConfig(level=logging.INFO)


def random_text(length: int) -> str:
    return "".join(random.choices(string.ascii_letters + string.digits, k=length))  # noqa: S311


def benchmark(fn, text_length, test_num):
    text = random_text(text_length).encode()

    def run_func():
        fn(text)

    logging.info("======== {fn.__name__}({text_length} bytes / {test_num} times) ========")
    t = timeit(run_func, number=test_num)
    logging.info(f"{fn.__name__}: {t * (1000 ** 2) / test_num:.2f} ns")

    return t


def benchmark_curve(fn, test_length, test_num):
    test_times = []

    for length in test_length:
        test_times.append(benchmark(fn, length, test_num))

    return np.array(test_times) / test_num


def main():
    test_num = 1000
    test_length = 2 ** np.linspace(0, 18, 18, dtype=int)

    def md5_hash(text):
        md5(text).hexdigest()

    def sha1_hash(text):
        sha1(text).hexdigest()

    def sha256_hash(text):
        sha256(text).hexdigest()

    def mmh3_hash64(text):
        mmh3.hash64(text)

    def mmh3_hash128(text):
        mmh3.hash128(text)

    def mmh3_hash_bytes(text):
        mmh3.hash(text)

    def rapidhash_hash(text):
        rapidhash(text)

    df_arr = []
    hash_functions = [
        md5_hash,
        sha1_hash,
        sha256_hash,
        mmh3_hash64,
        mmh3_hash128,
        mmh3_hash_bytes,
        rapidhash_hash,
    ]

    for fn in hash_functions:
        time_ns = benchmark_curve(fn, test_length, test_num) * (1000**2)
        df_arr.append(pd.DataFrame({"hash": fn.__name__, "length": test_length, "time[ns]": time_ns}))

    df = pd.concat(df_arr)

    sns.lineplot(data=df, x="length", y="time[ns]", hue="hash")
    sns.scatterplot(data=df, x="length", y="time[ns]", hue="hash")

    plt.legend(loc="upper left", borderaxespad=0, fontsize=8)
    plt.xscale("log")
    plt.yscale("log")
    plt.title("Hash Function Benchmark")
    plt.savefig("./docs/assets/benchmark.jpg")


if __name__ == "__main__":
    main()
