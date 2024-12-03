import logging

import numpy as np
from scipy.stats import chi2

from rapidhash import rapidhash


def chi_square_test(sample_size, num_buckets):
    observed = np.zeros(num_buckets, dtype=int)

    for i in range(sample_size):
        key = i.to_bytes(8, byteorder="little")
        hash_value = rapidhash(key)
        bucket = hash_value % num_buckets
        observed[bucket] += 1

    expected = sample_size / num_buckets
    chi_square = ((observed - expected) ** 2 / expected).sum()
    return chi_square


def chi_square_critical_value(confidence_level, degrees_of_freedom):
    return chi2.ppf(confidence_level, degrees_of_freedom)


def test_distribution():
    sample_size = 10000
    num_buckets = 256
    chi_square = chi_square_test(sample_size, num_buckets)
    degrees_of_freedom = num_buckets - 1
    confidence_level = 0.90
    critical_value = chi_square_critical_value(confidence_level, degrees_of_freedom)

    logging.info(f"Chi-square: {chi_square}")
    logging.info(f"Critical value: {critical_value}")

    assert (
        chi_square < critical_value
    ), f"Chi-square test failed: chi_square = {chi_square}, critical_value = {critical_value}"
