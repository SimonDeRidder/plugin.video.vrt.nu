use pyo3::{
	Bound, Py, PyAny, PyResult, Python, pyfunction, pymodule,
	types::{PyAnyMethods as _, PyDict, PyMapping, PyMappingMethods as _, PyModule, PyModuleMethods as _},
	wrap_pyfunction,
};

pub mod data;
pub mod errors;
pub mod kodi_log;
mod pyconv;
pub mod settings;
mod utils;

#[pyfunction]
fn init(
	py: Python<'_>,
	settings: &Bound<'_, PyDict>,
	cache_dir: &str,
	profile_dir: &str,
	log: Py<PyAny>,
) -> PyResult<()> {
	// validate paths
	let cache_path = settings::validate_path("cache_dir", cache_dir)?;
	let profile_path = settings::validate_path("profile_dir", profile_dir)?;
	// install log callback
	kodi_log::install(py, log)?;
	// initialise settings
	let parsed = settings::settings_from_dict(settings)?;
	settings::install(parsed, settings::Paths { cache_dir: cache_path, profile_dir: profile_path })?;
	// finish up
	log::info!("_vrtmax initialized (cache_dir={}, profile_dir={})", cache_dir, profile_dir);
	Ok(())
}

#[pyfunction]
fn update_settings(updates: &Bound<'_, PyMapping>) -> PyResult<()> {
	let keys = updates.keys()?;
	settings::update(updates)?;
	log::info!("update_settings applied: keys={}", keys.repr()?);
	Ok(())
}

#[pymodule]
fn _vrtmax(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add("__version__", env!("CARGO_PKG_VERSION"))?;
	errors::register(m.py(), m)?;
	m.add_function(wrap_pyfunction!(init, m)?)?;
	m.add_function(wrap_pyfunction!(update_settings, m)?)?;
	register_utils(m.py(), m)?;
	register_data(m.py(), m)?;
	Ok(())
}

fn register_utils(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
	let m = PyModule::new(py, "utils")?;
	m.add_function(wrap_pyfunction!(utils::reformat_image_url, &m)?)?;
	m.add_function(wrap_pyfunction!(utils::url_to_program, &m)?)?;
	m.add_function(wrap_pyfunction!(utils::play_url_to_id, &m)?)?;
	m.add_function(wrap_pyfunction!(utils::shorten_link, &m)?)?;
	m.add_function(wrap_pyfunction!(utils::find_entry, &m)?)?;
	m.add_function(wrap_pyfunction!(utils::youtube_to_plugin_url, &m)?)?;

	parent.add_submodule(&m)?;
	register_in_sys_modules(py, "_vrtmax.utils", &m)?;
	Ok(())
}

fn register_data(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
	let m = PyModule::new(py, "data")?;
	m.add_function(wrap_pyfunction!(data::categories, &m)?)?;
	m.add_function(wrap_pyfunction!(data::channels, &m)?)?;
	m.add_function(wrap_pyfunction!(data::relative_dates, &m)?)?;
	parent.add_submodule(&m)?;
	register_in_sys_modules(py, "_vrtmax.data", &m)?;
	Ok(())
}

fn register_in_sys_modules(py: Python<'_>, name: &str, module: &Bound<'_, PyModule>) -> PyResult<()> {
	py.import("sys")?.getattr("modules")?.set_item(name, module)?;
	Ok(())
}

#[cfg(test)]
mod tests {
	#[test]
	fn version_is_nonempty() {
		assert!(!env!("CARGO_PKG_VERSION").is_empty());
	}
}
