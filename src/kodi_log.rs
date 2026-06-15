use std::sync::OnceLock;

use log::{Level, Log, Metadata, Record};
use pyo3::{Py, PyAny, PyResult, Python};

use crate::errors::not_initialized;

static CALLBACK: OnceLock<std::sync::RwLock<Option<Py<PyAny>>>> = OnceLock::new();

fn callback_cell() -> &'static std::sync::RwLock<Option<Py<PyAny>>> {
	CALLBACK.get_or_init(|| std::sync::RwLock::new(None))
}

pub fn install(py: Python<'_>, callback: Py<PyAny>) -> PyResult<()> {
	*callback_cell()
		.write()
		.map_err(|err| not_initialized(format!("Could not get log callback lock: '{}'", err)))? = Some(callback);

	// log::set_logger may only be called once per process.
	static REGISTERED: OnceLock<()> = OnceLock::new();
	if REGISTERED.set(()).is_ok() {
		log::set_logger(&KODI_LOGGER)
			.map_err(|err| not_initialized(format!("Could not set logger: '{}'", err)))?;
		log::set_max_level(log::LevelFilter::Trace);
	}
	let _ = py; // silence unused;
	Ok(())
}

struct KodiLogger;
static KODI_LOGGER: KodiLogger = KodiLogger;

impl Log for KodiLogger {
	fn enabled(&self, _meta: &Metadata) -> bool {
		// Filtering happens Python-side.
		true
	}

	fn log(&self, record: &Record) {
		// Map log::Level to Kodi log levels (1..4).
		let kodi_level: i32 = match record.level() {
			Level::Trace => 1,
			Level::Debug => 1,
			Level::Info => 2,
			Level::Warn => 3,
			Level::Error => 4,
		};
		let message = format!("{}", record.args());
		Python::attach(|py| {
			// best effort, don't panic on error
			if let Ok(cb_guard) = callback_cell().read() {
				let Some(cb) = cb_guard.as_ref() else { return };
				let _ = cb
					.call1(py, (kodi_level, message))
					.inspect_err(|err| eprintln!("Error writing log: {}", err));
			} else {
				eprintln!("Error writing log: Could not get callback lock")
			}
		});
	}

	fn flush(&self) {}
}
