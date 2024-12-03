import logging
import random

from rapidhash import rapidhash

INPUT_SIZE = 256
NUM_TESTS = 1_000
TOLERANCE = 0.1


def test_avalanche_effect():
    total_diff_ratio = 0.0

    for _ in range(NUM_TESTS):
        input_int = random.getrandbits(INPUT_SIZE * 8)

        original_hash = rapidhash(input_int.to_bytes(INPUT_SIZE, byteorder="little"))

        for byte_idx in range(INPUT_SIZE):
            for bit_idx in range(8):
                modified_bytes = bytearray(input_int.to_bytes(INPUT_SIZE, byteorder="little"))
                modified_bytes[byte_idx] ^= 1 << bit_idx
                modified_int = int.from_bytes(modified_bytes, byteorder="little")

                modified_hash = rapidhash(modified_int.to_bytes(INPUT_SIZE, byteorder="little"))

                diff = bin(original_hash ^ modified_hash).count("1")
                total_diff_ratio += diff / 64.0

    total_bits = NUM_TESTS * INPUT_SIZE * 8
    avg_diff_ratio = total_diff_ratio / total_bits

    logging.info(f"Average bit difference ratio: {avg_diff_ratio * 100:.2f}%")

    assert abs(avg_diff_ratio - 0.5) < TOLERANCE, (
        "Avalanche effect test failed: "
        f"average bit difference ratio = {avg_diff_ratio * 100:.2f}%, expected close to 50%"
    )
