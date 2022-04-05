use pyo3::prelude::*;      // It enable the macros pyo3 crate has
use pyo3::wrap_pyfunction; // It let us wrap Rust function into module


//Apply a Python function macro from pyO3 to the say_hello function
#[pyfunction]
fn say_hello(){
    println!("Say Hello");
}


// Define the module as flitton_fib_rs
// That will be imported as flitton_fib_rs when using it.
#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule)->PyResult<()>{
        //wrap_pyfunction takes a Python instance  and resturn a python function
        m.add_wrapped(wrap_pyfunction!(say_hello));
        Ok(())
}