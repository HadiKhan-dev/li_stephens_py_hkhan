use ndarray;
use numpy::{IntoPyArray, PyArray1, PyArray2, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use li_stephens;
use std::collections::HashMap;


#[pymodule]
fn li_stephens_py_hkhan(_py: Python<'_>, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn li_stephens_probs<'py>(py: Python<'py>,haplotypes: Vec<String>, test_haplotype: String, mutation_prob: f64,
        recombination_prob: f64) -> f64
    {

        return li_stephens::li_stephens_probs(&haplotypes,&test_haplotype, mutation_prob, recombination_prob)
    }

    return Ok(());
}