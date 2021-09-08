#![feature(test)]

use std::env;
use pyo3::prelude::*;
use pyo3::types::PyList;

#[cfg(test)]
mod tests;

pub fn prepare_syspath(py: Python) {   
    let syspath: &PyList = py.import("sys")
        .unwrap()
        .getattr("path")
        .unwrap()
        .try_into()
        .unwrap();

    let mut current_path = env::current_dir().unwrap();
    current_path.push("src");
    syspath.insert(0, current_path).unwrap();
}

pub fn call_python_func((funcs, u_oup, u_xv, u_src, oup, epp, uf6): (&PyModule, f64, f64, f64, f64, f64, f64)) -> PyResult<(f64, f64)> {
    let (x, y) = funcs.getattr("calculate")?.call1((u_oup, u_xv, u_src, oup, epp, uf6,))?.extract()?;

    Ok((x, y))    
}

fn main() {
    println!("starting computing");
    let gil = Python::acquire_gil();
    let py = gil.python();
    prepare_syspath(py);
    let funcs = py.import("myfunctions").unwrap();
    let data = (funcs, 4.9, 0.13, 0.215917, 6085.116, 150861.133, 338032.351);
    let (x, y) = call_python_func(data).expect("Error calling python function");
    println!("Receiving result x={}, y={}", x, y);
}
