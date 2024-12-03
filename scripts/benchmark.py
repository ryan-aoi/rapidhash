# ruff: noqa: S324
import logging
import random
import string
import timeit
from hashlib import md5, sha1, sha256

import mmh3

from rapidhash import rapidhash

logging.basicConfig(level=logging.INFO)

TEXT_LENGTH = 1024 * (2**4)
NUM_TESTS = 10_000


def main():
    def random_text(length: int) -> str:
        return "".join(random.choices(string.ascii_letters + string.digits, k=length))  # noqa: S311

    text = random_text(TEXT_LENGTH).encode()

    def md5_hash():
        md5(text).hexdigest()

    logging.info(f"======== md5_hash({TEXT_LENGTH} bytes / {NUM_TESTS} times) ========")
    t = timeit.timeit(md5_hash, number=NUM_TESTS)
    logging.info(f"md5_hash: {t * (1000 ** 2) / NUM_TESTS:.2f} ns")

    def sha1_hash():
        sha1(text).hexdigest()

    logging.info(f"======== sha1_hash({TEXT_LENGTH} bytes / {NUM_TESTS} times) ========")
    t = timeit.timeit(sha1_hash, number=NUM_TESTS)
    logging.info(f"sha1_hash: {t * (1000 ** 2) / NUM_TESTS:.2f} ns")

    def sha256_hash():
        sha256(text).hexdigest()

    logging.info(f"======== sha256_hash({TEXT_LENGTH} bytes / {NUM_TESTS} times) ========")
    t = timeit.timeit(sha256_hash, number=NUM_TESTS)
    logging.info(f"sha256_hash: {t * (1000 ** 2) / NUM_TESTS:.2f} ns")

    def mmh3_hash64():
        mmh3.hash64(text)

    logging.info(f"======== mmh3_hash64({TEXT_LENGTH} bytes / {NUM_TESTS} times) ========")
    t = timeit.timeit(mmh3_hash64, number=NUM_TESTS)
    logging.info(f"mmh3_hash64: {t * (1000 ** 2) / NUM_TESTS:.2f} ns")

    def mmh3_hash128():
        mmh3.hash128(text)

    logging.info(f"======== mmh3_hash128({TEXT_LENGTH} bytes / {NUM_TESTS} times) ========")
    t = timeit.timeit(mmh3_hash128, number=NUM_TESTS)
    logging.info(f"mmh3_hash128: {t * (1000 ** 2) / NUM_TESTS:.2f} ns")

    def mmh3_hash_bytes():
        mmh3.hash(text)

    logging.info(f"======== mmh3_hash_bytes({TEXT_LENGTH} bytes / {NUM_TESTS} times) ========")
    t = timeit.timeit(mmh3_hash_bytes, number=NUM_TESTS)
    logging.info(f"mmh3_hash_bytes: {t * (1000 ** 2) / NUM_TESTS:.2f} ns")

    def rapidhash_hash():
        rapidhash(text)

    logging.info(f"======== rapidhash_hash({TEXT_LENGTH} bytes / {NUM_TESTS} times) ========")
    t = timeit.timeit(rapidhash_hash, number=NUM_TESTS)
    logging.info(f"rapidhash_hash: {t * (1000 ** 2) / NUM_TESTS:.2f} ns")


if __name__ == "__main__":
    main()
