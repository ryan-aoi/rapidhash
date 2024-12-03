from rapidhash._core import rs_rapidhash


def rapidhash(key: bytes, seed: int | None = None) -> int:
    """
    RapidHash function to compute a hash value for a given key and optional seed.

    Args:
        key (bytes): The input key for which the hash is to be computed. Must be of type bytes.
        seed (int | None, optional): An optional seed value to influence the hash computation.
                                     Must be an integer or None. Defaults to None.

    Returns:
        int: The computed hash value as an integer.

    Raises:
        TypeError: If the key is not of type bytes or the seed is not an integer or None.
        ValueError: If the seed is a negative integer or exceeds 64-bit integer range.
    """
    try:
        return rs_rapidhash(key, seed)
    except TypeError as e:
        if not isinstance(key, bytes):
            err = "key must be bytes"
            raise TypeError(err) from e
        if not isinstance(seed, int | None):
            err = "seed must be an integer or None"
            raise TypeError(err) from e

        err = "invalid argument"
        raise TypeError(err) from e
    except OverflowError as e:
        if seed:
            if seed < 0:
                err = "seed must be a non-negative integer"
                raise ValueError(err) from e
            if seed > 0xFFFFFFFFFFFFFFFF:  # noqa
                err = "seed must be a 64-bit integer"
                raise ValueError(err) from e
        err = "invalid argument"
        raise ValueError(err) from e
