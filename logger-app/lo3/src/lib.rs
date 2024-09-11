use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

#[pyclass]
struct RustLogger{
    file_path: String,
}

#[pymethods]
impl RustLogger{
    #[new]
    fn new(file_path: String) -> Self{
        RustLogger{file_path}
    }

    fn log(&self, level: &str, message: String, kwargs: Option<&PyDict>) -> PyResult<()>{
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&self.file_path)?;
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut log_entry = format!("[{}] - {} - {}", timestamp, level.to_uppercase(), message);

        if let Some(kwargs) = kwargs{
            log_entry += " ";
            for (key, value) in kwargs.iter(){
                log_entry+=&format!("{}={:?}", key, value);
            }
        }

        writeln!(file, "{}", log_entry)?;
        Ok(())
    }

    fn debug(&self, message: String, kwargs: Option<&PyDict>) -> PyResult<()>{
        self.log("DEBUG", message, kwargs)
    }
    fn info(&self, message: String, kwargs: Option<&PyDict>) -> PyResult<()>{
        self.log("INFO", message, kwargs)
    }
    fn warn(&self, message: String, kwargs: Option<&PyDict>) -> PyResult<()>{
        self.log("WARNING", message, kwargs)
    }
    fn error(&self, message: String, kwargs: Option<&PyDict>) -> PyResult<()>{
        self.log("ERROR", message, kwargs)
    }
}

#[pymodule]
fn lo3(_py: Python, m: &PyModule) -> PyResult<()>{
    m.add_class::<RustLogger>()?;
    Ok(())
}