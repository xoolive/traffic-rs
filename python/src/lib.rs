use pyo3::prelude::*;

pub mod intervals;
#[cfg(any(feature = "openblas", feature = "netlib"))]
pub mod kalman;

#[pymodule]
#[pyo3(name = "core")]
fn thrust(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add submodules
    let interval_mod = intervals::init(py)?;
    m.add_submodule(&interval_mod)?;

    #[cfg(any(feature = "openblas", feature = "netlib"))]
    {
        let kalman_mod = kalman::init(py)?;
        m.add_submodule(&kalman_mod)?;
    }

    Ok(())
}
