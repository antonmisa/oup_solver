extern crate test;

use super::*;
use test::Bencher;

#[bench]
fn bench_call_python_func(b: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    prepare_syspath(py);
    let funcs = py.import("myfunctions").unwrap();
    let data = (funcs, 4.9, 0.13, 0.215917, 6085.116, 150861.133, 338032.351);
    b.iter(|| {
        let _n = test::black_box(1000);

        call_python_func(data)
    })
}

#[test]
fn test_call_python_func() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    prepare_syspath(py);
    let funcs = py.import("myfunctions").unwrap();
    let data = (funcs, 4.9, 0.13, 0.215917, 6085.116, 150861.133, 338032.351);
    let (x, y) = call_python_func(data).expect("Error calling python function");
    assert!(x == 4.902748029373128 && y == 0.13000000031162223);
}