extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

//Rust extension in python based on template provided by :https://codeburst.io/how-to-use-rust-to-extend-python-360174ee5819

py_module_initializer!(mylib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_result", py_fn!(py, get_result(val: &str)))?;
    Ok(())
});

fn get_result(_py: Python, val: &str) -> PyResult<String> {
    Ok("Rust says: ".to_owned() + val)
}
