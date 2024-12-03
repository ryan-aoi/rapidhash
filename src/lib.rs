mod rapidhash;

use rapidhash::{rapidhash, RAPID_SEED, RAPID_SECRET};
use pyo3::prelude::*;

#[pyfunction(name = "rs_rapidhash")]
#[pyo3(signature = (key, seed=None))]
fn rs_rapidhash(key: &[u8], seed: Option<u64>) -> u64 {
    let seed = seed.unwrap_or(RAPID_SEED);
    rapidhash(key, seed, &RAPID_SECRET)
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_rapidhash, m)?)?;
    Ok(())
}
