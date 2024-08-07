use numpy::{PyArray1, PyArray2, PyArray3, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::types::IntoPyDict;
use pyo3::{prelude::*, types::PyDict};
use pyo3_polars::PyDataFrame;
use trafficrs::intervals::{Interval, IntervalCollection};
use trafficrs::kalman::kalman6d;

fn get_ic(
    start: PyReadonlyArray1<i64>,
    stop: PyReadonlyArray1<i64>,
) -> IntervalCollection<i64> {
    let size1 = start.len().unwrap();
    let size2 = stop.len().unwrap();
    let size = std::cmp::min(size1, size2);

    let mut elts = Vec::<Interval<i64>>::with_capacity(size);
    for i in 0..size {
        elts.push(Interval {
            start: *start.get(i).unwrap(),
            stop: *stop.get(i).unwrap(),
        })
    }
    IntervalCollection { elts }
}

#[pyfunction]
fn interval_and(
    py: Python,
    start1: i64,
    stop1: i64,
    start2: i64,
    stop2: i64,
) -> PyResult<Bound<PyDict>> {
    let left = Interval {
        start: start1,
        stop: stop1,
    };
    let right = Interval {
        start: start2,
        stop: stop2,
    };
    let res = match &left & &right {
        None => [("empty", true)].into_py_dict_bound(py),
        Some(Interval { start, stop }) => {
            [("start", start), ("stop", stop)].into_py_dict_bound(py)
        }
    };
    Ok(res)
}

#[pyfunction]
fn collection_and<'a>(
    py: Python<'a>,
    start1: PyReadonlyArray1<i64>,
    stop1: PyReadonlyArray1<i64>,
    start2: PyReadonlyArray1<i64>,
    stop2: PyReadonlyArray1<i64>,
) -> PyResult<Bound<'a, PyDict>> {
    let left = get_ic(start1, stop1);
    let right = get_ic(start2, stop2);
    let res = &left & &right;
    let start: Vec<i64> = res.elts.iter().map(|elt| elt.start).collect();
    let stop: Vec<i64> = res.elts.iter().map(|elt| elt.stop).collect();

    let wrapped_res = PyDict::new_bound(py);
    wrapped_res.set_item("start", PyArray1::from_vec_bound(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec_bound(py, stop))?;
    Ok(wrapped_res)
}

#[pyfunction]
fn collection_andi<'a>(
    py: Python<'a>,
    start1: PyReadonlyArray1<i64>,
    stop1: PyReadonlyArray1<i64>,
    start2: i64,
    stop2: i64,
) -> PyResult<Bound<'a, PyDict>> {
    let left = get_ic(start1, stop1);
    let right = Interval {
        start: start2,
        stop: stop2,
    };

    let res = &left & &right;
    let start: Vec<i64> = res.elts.iter().map(|elt| elt.start).collect();
    let stop: Vec<i64> = res.elts.iter().map(|elt| elt.stop).collect();

    let wrapped_res = PyDict::new_bound(py);
    wrapped_res.set_item("start", PyArray1::from_vec_bound(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec_bound(py, stop))?;
    Ok(wrapped_res)
}

#[pyfunction]
fn interval_add(
    py: Python,
    start1: i64,
    stop1: i64,
    start2: i64,
    stop2: i64,
) -> PyResult<Bound<PyDict>> {
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

    let wrapped_res = PyDict::new_bound(py);
    wrapped_res.set_item("start", PyArray1::from_vec_bound(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec_bound(py, stop))?;
    Ok(wrapped_res)
}

#[pyfunction]
fn collection_add<'a>(
    py: Python<'a>,
    start1: PyReadonlyArray1<i64>,
    stop1: PyReadonlyArray1<i64>,
    start2: PyReadonlyArray1<i64>,
    stop2: PyReadonlyArray1<i64>,
) -> PyResult<Bound<'a, PyDict>> {
    let left = get_ic(start1, stop1);
    let right = get_ic(start2, stop2);
    let res = left + right;

    let start: Vec<i64> = res.elts.iter().map(|elt| elt.start).collect();
    let stop: Vec<i64> = res.elts.iter().map(|elt| elt.stop).collect();

    let wrapped_res = PyDict::new_bound(py);
    wrapped_res.set_item("start", PyArray1::from_vec_bound(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec_bound(py, stop))?;
    Ok(wrapped_res)
}

#[pyfunction]
fn collection_addi<'a>(
    py: Python<'a>,
    start1: PyReadonlyArray1<i64>,
    stop1: PyReadonlyArray1<i64>,
    start2: i64,
    stop2: i64,
) -> PyResult<Bound<'a, PyDict>> {
    let left = get_ic(start1, stop1);
    let right = Interval {
        start: start2,
        stop: stop2,
    };
    let res = &left + &right;

    let start: Vec<i64> = res.elts.iter().map(|elt| elt.start).collect();
    let stop: Vec<i64> = res.elts.iter().map(|elt| elt.stop).collect();

    let wrapped_res = PyDict::new_bound(py);
    wrapped_res.set_item("start", PyArray1::from_vec_bound(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec_bound(py, stop))?;
    Ok(wrapped_res)
}

#[pyfunction]
fn interval_sub(
    py: Python,
    start1: i64,
    stop1: i64,
    start2: i64,
    stop2: i64,
) -> PyResult<Bound<PyDict>> {
    let left = Interval {
        start: start1,
        stop: stop1,
    };
    let right = Interval {
        start: start2,
        stop: stop2,
    };

    let res = left - right;
    let start: Vec<i64> = res.elts.iter().map(|elt| elt.start).collect();
    let stop: Vec<i64> = res.elts.iter().map(|elt| elt.stop).collect();

    let wrapped_res = PyDict::new_bound(py);
    wrapped_res.set_item("start", PyArray1::from_vec_bound(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec_bound(py, stop))?;
    Ok(wrapped_res)
}

#[pyfunction]
fn collection_sub<'a>(
    py: Python<'a>,
    start1: PyReadonlyArray1<i64>,
    stop1: PyReadonlyArray1<i64>,
    start2: PyReadonlyArray1<i64>,
    stop2: PyReadonlyArray1<i64>,
) -> PyResult<Bound<'a, PyDict>> {
    let left = get_ic(start1, stop1);
    let right = get_ic(start2, stop2);
    let res = left - right;

    let start: Vec<i64> = res.elts.iter().map(|elt| elt.start).collect();
    let stop: Vec<i64> = res.elts.iter().map(|elt| elt.stop).collect();

    let wrapped_res = PyDict::new_bound(py);
    wrapped_res.set_item("start", PyArray1::from_vec_bound(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec_bound(py, stop))?;
    Ok(wrapped_res)
}

#[pyfunction]
fn collection_subi<'a>(
    py: Python<'a>,
    start1: PyReadonlyArray1<i64>,
    stop1: PyReadonlyArray1<i64>,
    start2: i64,
    stop2: i64,
) -> PyResult<Bound<'a, PyDict>> {
    let left = get_ic(start1, stop1);
    let right = Interval {
        start: start2,
        stop: stop2,
    };
    let res = left - right;

    let start: Vec<i64> = res.elts.iter().map(|elt| elt.start).collect();
    let stop: Vec<i64> = res.elts.iter().map(|elt| elt.stop).collect();

    let wrapped_res = PyDict::new_bound(py);
    wrapped_res.set_item("start", PyArray1::from_vec_bound(py, start))?;
    wrapped_res.set_item("stop", PyArray1::from_vec_bound(py, stop))?;
    Ok(wrapped_res)
}

#[pyfunction]
fn kalman6d_rs(py: Python, pydf: PyDataFrame) -> PyResult<Bound<PyDict>> {
    kalman6d(pydf.into())
        .map(|(x_pre, x_cor, p_pre, p_cor)| {
            let wrapped_res = PyDict::new_bound(py);
            wrapped_res
                .set_item("x_pre", PyArray2::from_owned_array_bound(py, x_pre))
                .unwrap();
            wrapped_res
                .set_item("x_cor", PyArray2::from_owned_array_bound(py, x_cor))
                .unwrap();
            wrapped_res
                .set_item("p_pre", PyArray3::from_owned_array_bound(py, p_pre))
                .unwrap();
            wrapped_res
                .set_item("p_cor", PyArray3::from_owned_array_bound(py, p_cor))
                .unwrap();
            wrapped_res
        })
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pymodule]
#[pyo3(name = "core")]
fn thrust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(interval_and, m)?)?;
    m.add_function(wrap_pyfunction!(collection_and, m)?)?;
    m.add_function(wrap_pyfunction!(collection_andi, m)?)?;

    m.add_function(wrap_pyfunction!(interval_add, m)?)?;
    m.add_function(wrap_pyfunction!(collection_add, m)?)?;
    m.add_function(wrap_pyfunction!(collection_addi, m)?)?;

    m.add_function(wrap_pyfunction!(interval_sub, m)?)?;
    m.add_function(wrap_pyfunction!(collection_sub, m)?)?;
    m.add_function(wrap_pyfunction!(collection_subi, m)?)?;

    m.add_function(wrap_pyfunction!(kalman6d_rs, m)?)?;

    Ok(())
}
