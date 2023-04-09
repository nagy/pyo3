#[cfg(feature = "experimental-inspect")]
use crate::inspect::types::TypeInfo;
use crate::{types::PyString, FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python, ToPyObject};
use std::net::Ipv4Addr;
use std::str::FromStr;

impl IntoPy<PyObject> for Ipv4Addr {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyString::new(py, &self.to_string()).into()
    }

    #[cfg(feature = "experimental-inspect")]
    fn type_output() -> TypeInfo {
        TypeInfo::builtin("str")
    }
}

impl ToPyObject for Ipv4Addr {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &self.to_string()).into()
    }
}

impl<'a> FromPyObject<'a> for Ipv4Addr {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        Ok(Self::from_str(obj.downcast::<PyString>().unwrap().to_str().unwrap()).unwrap())
    }

    #[cfg(feature = "experimental-inspect")]
    fn type_input() -> TypeInfo {
        Self::type_output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Python;
    use crate::ToPyObject;

    #[test]
    fn test_extract_ipv4() {
        Python::with_gil(|py| {
            let localhost = Ipv4Addr::new(127, 0, 0, 1);
            let py_obj = localhost.to_object(py);
            let extracted: Ipv4Addr = py_obj.extract(py).unwrap();
            assert_eq!(localhost, extracted);
        });
    }
}
