use pyo3::prelude::*;      // It enable the macros pyo3 crate has
use pyo3::wrap_pyfunction; // It let us wrap Rust function into module

//Initial declare the module fib_calcs
mod fib_calcs;
// Prefix `__pyo3_get_function_` let us to retain the macros applied to the functions
use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number;
use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;

pub mod fib_numbers;

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
        m.add_wrapped(wrap_pyfunction!(fibonacci_number));
        m.add_wrapped(wrap_pyfunction!(fibonacci_numbers));
        Ok(())
}