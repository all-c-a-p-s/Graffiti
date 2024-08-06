use pyo3::prelude::*;
use pyo3::prepare_freethreaded_python;

pub fn write_holds_to_file() -> std::io::Result<()> {
    Ok(())
}

//run this AFTER calling write_holds_to_file()
pub fn import_model() {
    let model = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/model/model.py"
    ));
    let auto = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/model/auto.py"
    ));

    prepare_freethreaded_python();
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from_code_bound(py, auto, "auto", "auto")?;
        let model: Py<PyAny> = PyModule::from_code_bound(py, model, "", "")?
            .getattr("run_model")?
            .into();
        model.call0(py)
    }).expect("failed to call model");
    println!("{}", from_python);
}
