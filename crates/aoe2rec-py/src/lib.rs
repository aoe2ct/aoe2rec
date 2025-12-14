use pyo3::prelude::*;

#[pymodule]
mod aoe2rec_py {
    use pyo3::{pyclass, pyfunction, pymethods, Bound, PyAny, PyResult, Python};
    use pythonize::pythonize;

    #[pyfunction]
    fn parse_rec(py: Python<'_>, data: Vec<u8>) -> PyResult<Bound<'_, PyAny>> {
        let rec = aoe2rec::Savegame::from_bytes(&data.try_into()?).unwrap();
        let pyrec = pythonize(py, &rec)?;
        Ok(pyrec)
    }

    #[pyclass]
    struct Savegame(aoe2rec::Savegame);
    #[pymethods]
    impl Savegame {
        #[new]
        fn from_bytes(data: Vec<u8>) -> PyResult<Self> {
            let savegame = aoe2rec::Savegame::from_bytes(&data.try_into()?).unwrap();
            Ok(Savegame(savegame))
        }
    }
}
