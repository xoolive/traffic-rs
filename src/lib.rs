use intervals::Interval;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::types::IntoPyDict;
use pyo3::{prelude::*, types::PyDict};

#[pyfunction]
fn process_numpy_datetime_array(_py: Python, array: PyReadonlyArray1<i64>) -> PyResult<i64> {
    // Process the NumPy array containing datetime objects
    let num_elements = array.len();

    // Example: Print each datetime element
    for i in 0..num_elements {
        let datetime_obj = array.get(i).unwrap();
        println!("Datetime {}: {}", i, datetime_obj);
    }

    Ok(num_elements as i64)
}

#[pyfunction]
fn interval_and(py: Python, start1: i64, stop1: i64, start2: i64, stop2: i64) -> PyResult<&PyDict> {
    let left = Interval {
        start: start1,
        stop: stop1,
    };
    let right = Interval {
        start: start2,
        stop: stop2,
    };
    let res = match left & right {
        None => [("empty", true)].into_py_dict(py),
        Some(Interval { start, stop }) => [("start", start), ("stop", stop)].into_py_dict(py),
    };
    Ok(res)
}

#[pyfunction]
fn interval_add(py: Python, start1: i64, stop1: i64, start2: i64, stop2: i64) -> PyResult<&PyDict> {
    let left = Interval {
        start: start1,
        stop: stop1,
    };
    let right = Interval {
        start: start2,
        stop: stop2,
    };
    let res = left + right;
    let start: Vec<i64> = res.elts.iter().map(|elt| elt.start).collect();
    let stop: Vec<i64> = res.elts.iter().map(|elt| elt.stop).collect();

    let wrapped_res = PyDict::new(py);
    wrapped_res.set_item("start", PyArray1::from_vec(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec(py, stop))?;
    Ok(wrapped_res)
}

#[pymodule]
#[pyo3(name = "_rust_core")]
fn traffic_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(interval_and, m)?)?;
    m.add_function(wrap_pyfunction!(interval_add, m)?)?;

    Ok(())
}

pub mod intervals;
