use pyo3::prelude::*;
use pyo3_file::PyFileLikeObject;
use std::io::Read;

/// Reads from file-like object and prints results.
#[pyfunction]
fn read_from_file(obj: PyObject) -> PyResult<()> {
    let mut filelike =
        PyFileLikeObject::with_requirements(obj, true, false, false)?;
    let mut buf: [u8; 16] = [0; 16];
    loop {
        match filelike.read(&mut buf) {
            Ok(0) => {
                println!("nothing to read");
                break;
            }
            Ok(n) => {
                let content = &buf[..n];
                let as_str = String::from_utf8(content.to_vec())?;
                println!("read {n} bytes: {content:?} ({as_str})");
            }
            Err(e) => {
                println!("ERROR: {e:?}");
                break;
            }
        }
    }
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pkg1(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_from_file, m)?)?;
    Ok(())
}
