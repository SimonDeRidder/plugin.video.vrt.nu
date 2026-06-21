// Lives in its own integration-test binary (separate process) so the
// module-level `SETTINGS` OnceLock is guaranteed un-initialized.

use serial_test::serial;

#[test]
#[serial]
fn update_before_init_errors() {
	use pyo3::{
		IntoPyObjectExt as _, Python,
		types::{PyDict, PyDictMethods as _},
	};

	use _vrtmax::settings::update;

	Python::initialize();
	Python::attach(|py| {
		let python_dict = PyDict::from_sequence(
			&[("usefavorites", true.into_py_any(py).unwrap())]
				.into_bound_py_any(py)
				.unwrap(),
		)
		.unwrap();

		let update_result = update(python_dict.as_mapping());
		assert!(update_result.is_err());
		let err_str = update_result.unwrap_err().to_string();
		assert!(
			err_str
				.matches("update_settings called before init")
				.any(|_| true)
		);
	});
}
