use super::unit::zz::py::py_zz;
use pyo3::prelude::*;
use std::process;

#[pymodule]
#[pyo3(name = "amalie")]
fn amalie(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ctrlc::set_handler(move || {
        process::exit(130);
    })
    .expect("Error setting Ctrl+C handler");

    let _ = py_zz(&m);
    Ok(())
}
