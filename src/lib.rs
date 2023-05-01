
use excel::workbook::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use polars_core::prelude::*;
use pyo3_polars::PyDataFrame;

pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION").to_string();
    // cargo uses "1.0-alpha1" etc. while python uses "1.0.0a1", this is not full compatibility,
    // but it's good enough for now
    // see https://docs.rs/semver/1.0.9/semver/struct.Version.html#method.parse for rust spec
    // see https://peps.python.org/pep-0440/ for python spec
    // it seems the dot after "alpha/beta" e.g. "-alpha.1" is not necessary, hence why this works
    version.replace("-alpha", "a").replace("-beta", "b")
}


#[pyfunction]
fn get_pivot_data(py: Python, file_path: String) -> PyResult<&PyAny> {

    
        let mut pivot: PivotTable<_> = open_pivottable(&file_path).unwrap();
        let x = pivot.row_data.iter().enumerate().map(|(i, d)| {
            Series::new(pivot.cache_definitions[i].cacheDefintionName.as_str(), d)
        }).collect();

        let df = DataFrame::new(x).unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(PyDataFrame(df))
        })
}

#[pymodule]
fn excelpivotdata(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", get_version())?;
    m.add_function(wrap_pyfunction!(get_pivot_data, m)?)?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
    
    let path = format!("/root/repos/rust/Pivot.xlsx");
    let pivot: PivotTable<_> = open_pivottable(&path).unwrap();
    //let duration = start.elapsed();


    // println!("Time elapsein expensive_function() is: {:?}", duration);
    

}


