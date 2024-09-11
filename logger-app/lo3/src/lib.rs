use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use chrono::Local;
use std::sync::Mutex;

#[pyclass]
struct RustLogger {
    file: Mutex<BufWriter<std::fs::File>>,
}

#[pymethods]
impl RustLogger {
    #[new]
    fn new(file_path: String) -> PyResult<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        Ok(RustLogger { file: Mutex::new(buf_writer) })
    }

    fn log(&self, level: &str, message: String, kwargs: Option<&PyDict>) -> PyResult<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut log_entry = format!("[{}] {} - {}", timestamp, level.to_uppercase(), message);

        if let Some(kwargs) = kwargs {
            for (key, value) in kwargs.iter() {
                log_entry.push_str(&format!(" {}={:?}", key, value));
            }
        }
        log_entry.push('\n');

        let mut file = self.file.lock().unwrap();
        file.write_all(log_entry.as_bytes())?;
        Ok(())
    }

    fn debug(&self, message: String, kwargs: Option<&PyDict>) -> PyResult<()> {
        self.log("DEBUG", message, kwargs)
    }

    fn info(&self, message: String, kwargs: Option<&PyDict>) -> PyResult<()> {
        self.log("INFO", message, kwargs)
    }

    fn warning(&self, message: String, kwargs: Option<&PyDict>) -> PyResult<()> {
        self.log("WARNING", message, kwargs)
    }

    fn error(&self, message: String, kwargs: Option<&PyDict>) -> PyResult<()> {
        self.log("ERROR", message, kwargs)
    }

    fn flush(&self) -> PyResult<()> {
        let mut file = self.file.lock().unwrap();
        file.flush()?;
        Ok(())
    }
}

#[pymodule]
fn lo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustLogger>()?;
    Ok(())
}
