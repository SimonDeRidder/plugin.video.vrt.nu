use pyo3::{
	Bound, IntoPyObject as _, PyAny, PyResult, Python,
	types::{PyDict, PyDictMethods as _, PyList, PyListMethods as _},
};
use serde_json::Value;

use crate::errors::VrtError;

/// Convert a serde_json::Value into the natural Python object
pub fn json_to_py<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, PyAny>> {
	Ok(match value {
		Value::Null => py.None().into_bound(py),
		Value::Bool(val) => val.into_pyobject(py)?.to_owned().into_any(),
		Value::Number(val) => {
			if let Some(integer) = val.as_i64() {
				integer.into_pyobject(py)?.into_any()
			} else if let Some(unsigned) = val.as_u64() {
				unsigned.into_pyobject(py)?.into_any()
			} else {
				val.as_f64()
					.ok_or_else(|| {
						VrtError::Parse("Unable to parse number as f64 (non-finite/out of range)".to_string())
					})?
					.into_pyobject(py)?
					.into_any()
			}
		},
		Value::String(val) => val.into_pyobject(py)?.into_any(),
		Value::Array(items) => {
			let list = PyList::empty(py);
			for item in items {
				list.append(json_to_py(py, item)?)?;
			}
			list.into_any()
		},
		Value::Object(map) => {
			let dict = PyDict::new(py);
			for (key, val) in map {
				dict.set_item(key, json_to_py(py, val)?)?;
			}
			dict.into_any()
		},
	})
}

#[cfg(test)]
mod tests {

	use pyo3::{PyTypeCheck, Python, types::PyAnyMethods as _};
	use serde_json::Value;

	use crate::pyconv::json_to_py;

	#[test]
	fn full_conversion() {
		let mut json_dict = serde_json::value::Map::new();
		json_dict.insert("null_test".to_string(), Value::Null);
		json_dict.insert("bool_test".to_string(), Value::Bool(true));
		json_dict.insert("int_test".to_string(), Value::Number(30.into()));
		json_dict.insert("float_test".to_string(), Value::Number(serde_json::Number::from_f64(6.3).unwrap()));
		json_dict.insert("string_test".to_string(), Value::String("Tortuga".to_string()));
		json_dict.insert(
			"list_test".to_string(),
			Value::Array(
				["Nelson", "Hornblower", "Benbow"]
					.iter()
					.map(|string| Value::String(string.to_string()))
					.collect(),
			),
		);

		Python::initialize();
		Python::attach(|py| {
			let py_dict = json_to_py(py, &Value::Object(json_dict)).unwrap();

			assert!(pyo3::types::PyNone::type_check(&py_dict.get_item("null_test").unwrap()));
			assert!(pyo3::types::PyBool::type_check(&py_dict.get_item("bool_test").unwrap()));
			assert!(pyo3::types::PyInt::type_check(&py_dict.get_item("int_test").unwrap()));
			assert!(pyo3::types::PyFloat::type_check(&py_dict.get_item("float_test").unwrap()));
			assert!(pyo3::types::PyString::type_check(&py_dict.get_item("string_test").unwrap()));
			assert!(pyo3::types::PyList::type_check(&py_dict.get_item("list_test").unwrap()));
		});
	}
}
