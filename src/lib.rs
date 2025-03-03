mod rapidhash;

use rapidhash::{rapidhash, rapidhash_file, RAPID_SECRET, RAPID_SEED};
use pyo3::prelude::*;

#[pyfunction(name = "rs_rapidhash")]
#[pyo3(signature = (key, seed=None))]
fn rs_rapidhash(key: &[u8], seed: Option<u64>) -> u64 {
    let seed = seed.unwrap_or(RAPID_SEED);
    rapidhash(key, seed, &RAPID_SECRET)
}

#[pyfunction(name = "rs_rapidhash_file")]
#[pyo3(signature = (filename,seed=None))]
fn rs_rapidhash_file(filename: String, seed: Option<u64>) -> u64 {
    let seed = seed.unwrap_or(RAPID_SEED);
    rapidhash_file(filename,seed,&RAPID_SECRET)
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_rapidhash, m)?)?;
    m.add_function(wrap_pyfunction!(rs_rapidhash_file, m)?)?;
    Ok(())
}
