use _vrtmax::kodi_log::install;
use pyo3::{IntoPyObjectExt, Python, pyclass, pymethods};
use serial_test::serial;

#[derive(Clone)]
#[pyclass(from_py_object)]
struct FakeCallback {
	counter: u8,
}

#[pymethods]
impl FakeCallback {
	fn __call__(&mut self, _level: i32, _message: String) {
		self.counter += 1;
	}
}

#[test]
#[serial]
fn log_after_re_init_uses_new_callback() {
	Python::initialize();
	Python::attach(|py| {
		let callback_a = (FakeCallback { counter: 0 }).into_py_any(py).unwrap();
		let callback_b = (FakeCallback { counter: 0 }).into_py_any(py).unwrap();
		install(py, callback_a.clone_ref(py)).unwrap();
		log::info!("hello!");
		assert_eq!(callback_a.extract::<FakeCallback>(py).unwrap().counter, 1);
		assert_eq!(callback_b.extract::<FakeCallback>(py).unwrap().counter, 0);
		install(py, callback_b.clone_ref(py)).unwrap();
		log::info!("hello!");
		assert_eq!(callback_a.extract::<FakeCallback>(py).unwrap().counter, 1);
		assert_eq!(callback_b.extract::<FakeCallback>(py).unwrap().counter, 1);
	});
}

#[test]
#[serial]
fn log_from_thread_without_gil_works() {
	Python::initialize();
	Python::attach(|py| {
		let callback = (FakeCallback { counter: 0 }).into_py_any(py).unwrap();
		install(py, callback.clone_ref(py)).unwrap();
		py.detach(|| {
			log::warn!("hello!");
		});
		assert_eq!(callback.extract::<FakeCallback>(py).unwrap().counter, 1);
	});
}
