use std::str::FromStr;
use pyo3::prelude::*;
use not_br_lib::not_br;


#[pyfunction]
fn process_text(input: String, frequency: u32, bold_percentage: f64, output_type: String) -> PyResult<String> {
    let output_type = match not_br::OutputType::from_str(output_type.as_str()) {
        Ok(output_type) => { output_type }
        Err(_) => { return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid output type.")); }
    };

    let bold_percentage =
        if bold_percentage < 0. {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("'bold_percentage has to be in range [0,1] or (1,100]'"));
        } else if bold_percentage > 1. {
            bold_percentage / 100.
        } else { bold_percentage };

    not_br::process(input.as_str(), frequency as u64, bold_percentage, output_type)
        .map_or(Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("'Internal Error. This is probably a bug.'")),
                |s| Ok(s))
}

/// A Python module implemented in Rust.
#[pymodule]
fn not_br_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_text, m)?)?;
    Ok(())
}
