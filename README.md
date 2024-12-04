# rapidhash
Very fast, high quality, platform-independent hashing algorithm in Python.

This project implements **rapidhash** in Rust and exposes it to Python through the pyo3 interface. 
It maintains compatibility with the original C implementation ([Nicoshev/rapidhash](https://github.com/Nicoshev/rapidhash)) and follows the BSD 2-Clause license.

Thanks to **Nicoshev** for their excellent work.

This `README.md` is still a work in progress—I’ll finish it when time permits!

## Install

Use the following command to install.

```sh
pip install rapidhash
```

## Usage

```python
from rapidhash import rapidhash

key = "hello world"
print(rapidhash(key.encode()))

key = "hello world"
seed = 42
print(rapidhash(key.encode(), seed))
```
