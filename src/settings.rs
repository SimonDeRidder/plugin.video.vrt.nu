use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

use pyo3::types::{PyAnyMethods as _, PyDict, PyDictMethods as _, PyMapping, PyMappingMethods as _};
use pyo3::{Bound, FromPyObject, PyResult};

use crate::errors::{VrtError, not_initialized};

/// Module-level state initialized by `init`.
static SETTINGS: OnceLock<RwLock<Settings>> = OnceLock::new();
static PATHS: OnceLock<Paths> = OnceLock::new();
const SETTINGS_KEYS: [&str; 19] = [
	"username",
	"password",
	"itemsperpage",
	"usefavorites",
	"useresumepoints",
	"showpermalink",
	"showfanart",
	"showyoutube",
	"usedrm",
	"useinputstreamadaptive",
	"max_bandwidth",
	"kodi_version_major",
	"has_inputstream_adaptive",
	"can_play_drm",
	"supports_drm",
	"has_credentials",
	"has_studios_white",
	"has_youtube",
	"has_iptv_manager",
];

#[derive(Clone, Default)]
pub struct Settings {
	// User prefs
	pub username: String,
	pub password: String,
	pub itemsperpage: u32,
	pub usefavorites: bool,
	pub useresumepoints: bool,
	pub showpermalink: bool,
	pub showfanart: bool,
	pub showyoutube: bool,
	pub usedrm: bool,
	pub useinputstreamadaptive: bool,
	pub max_bandwidth: u32,

	// Kodi-env state
	pub kodi_version_major: u32,
	pub has_inputstream_adaptive: bool,
	pub can_play_drm: bool,
	pub supports_drm: bool,
	pub has_credentials: bool,
	pub has_studios_white: bool,
	pub has_youtube: bool,
	pub has_iptv_manager: bool,
}

// Manual `Debug` to redact `username` and `password`.
impl std::fmt::Debug for Settings {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter
			.debug_struct("Settings")
			.field("username", &Redacted(self.username.len()))
			.field("password", &Redacted(self.password.len()))
			.field("itemsperpage", &self.itemsperpage)
			.field("usefavorites", &self.usefavorites)
			.field("useresumepoints", &self.useresumepoints)
			.field("showpermalink", &self.showpermalink)
			.field("showfanart", &self.showfanart)
			.field("showyoutube", &self.showyoutube)
			.field("usedrm", &self.usedrm)
			.field("useinputstreamadaptive", &self.useinputstreamadaptive)
			.field("max_bandwidth", &self.max_bandwidth)
			.field("kodi_version_major", &self.kodi_version_major)
			.field("has_inputstream_adaptive", &self.has_inputstream_adaptive)
			.field("can_play_drm", &self.can_play_drm)
			.field("supports_drm", &self.supports_drm)
			.field("has_credentials", &self.has_credentials)
			.field("has_studios_white", &self.has_studios_white)
			.field("has_youtube", &self.has_youtube)
			.field("has_iptv_manager", &self.has_iptv_manager)
			.finish()
	}
}

struct Redacted(usize);
impl std::fmt::Debug for Redacted {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.0 == 0 {
			write!(formatter, "<redacted, empty>")
		} else {
			write!(formatter, "<redacted, {} chars>", self.0)
		}
	}
}

#[derive(Clone, Debug)]
pub struct Paths {
	pub cache_dir: PathBuf,
	pub profile_dir: PathBuf,
}

/// Validate a path string coming from Python
/// Rules:
///   - must be absolute (`Path::is_absolute()`)
///   - must not contain any `..` components (`Component::ParentDir`)
///   - interior NUL bytes are already rejected by PyO3's `&str` conversion
///
/// Symlinks are NOT resolved.
/// Returns `PyValueError` on rejection.
pub fn validate_path(label: &str, path: &str) -> PyResult<PathBuf> {
	use std::path::{Component, Path};
	let parsed_path = Path::new(path);
	if !parsed_path.is_absolute() {
		return Err(pyo3::exceptions::PyValueError::new_err(format!(
			"{label} must be an absolute path (got {path:?})"
		)));
	}
	if parsed_path
		.components()
		.any(|c| matches!(c, Component::ParentDir))
	{
		return Err(pyo3::exceptions::PyValueError::new_err(format!(
			"{label} must not contain '..' components (got {path:?})"
		)));
	}
	Ok(parsed_path.to_path_buf())
}

pub fn install(settings: Settings, paths: Paths) -> PyResult<()> {
	// `init` may be called more than once per process (tests, defensive re-init).
	// First call wins via `OnceLock::set`;
	match SETTINGS.set(RwLock::new(settings.clone())) {
		Ok(_) => {},
		Err(_) => {
			*SETTINGS
				.get()
				.ok_or(not_initialized("Could not get settings lock"))?
				.write()
				.map_err(|err| not_initialized(format!("Could not get settings lock: {}", err)))? = settings;
		},
	}
	match PATHS.set(paths.clone()) {
		Ok(_) => {},
		Err(_) => {
			// Paths are immutable for the life of the process per the Boundary Contract.
			// Tests that re-init in the same process keep the original paths.
			//
			// Warn loudly so a developer who did expect new paths gets a signal.
			let existing = PATHS.get().ok_or(not_initialized("could not get paths lock"))?;
			if existing.cache_dir != paths.cache_dir || existing.profile_dir != paths.profile_dir {
				log::warn!(
					"settings::install called again with different paths \
					 (existing cache_dir={:?}, profile_dir={:?}); new paths ignored",
					existing.cache_dir,
					existing.profile_dir,
				);
			}
		},
	};
	Ok(())
}

pub fn update(updates: &Bound<'_, PyMapping>) -> PyResult<()> {
	let cell = SETTINGS
		.get()
		.ok_or_else(|| not_initialized("update_settings called before init"))?;
	// Take the write lock for the whole operation, but apply to a local clone
	// first, that way a `ParseError` raised partway through  returns Err with
	// the stored Settings untouched.
	let mut guard = cell
		.write()
		.map_err(|err| not_initialized(format!("could not get settings lock: {}", err)))?;
	let mut temp = guard.clone();
	apply(&mut temp, updates, false)?;
	*guard = temp;
	Ok(())
}

pub fn snapshot() -> PyResult<Settings> {
	let cell = SETTINGS
		.get()
		.ok_or_else(|| not_initialized("snapshot taken before init"))?;
	Ok(cell
		.read()
		.map_err(|err| not_initialized(format!("could not get settings lock: {}", err)))?
		.clone())
}

pub fn paths() -> PyResult<&'static Paths> {
	PATHS
		.get()
		.ok_or_else(|| not_initialized("paths read before init"))
}

/// Build a `Settings` from a Python dict.
pub fn settings_from_dict(dict: &Bound<'_, PyDict>) -> PyResult<Settings> {
	let mut settings = Settings::default();
	apply(&mut settings, dict.as_mapping(), true)?;
	Ok(settings)
}

/// Single source of truth for the Settings field list. Both `settings_from_dict`
/// (required=true; missing fields raise `ParseError`) and `update` (required=false;
/// missing fields are simply skipped) route through here, so a new field added to
/// `Settings` is impossible to forget on one side and remember on the other.
fn apply(settings: &mut Settings, dict: &Bound<'_, PyMapping>, required: bool) -> PyResult<()> {
	set_setting(dict, "username", "str", &mut settings.username, required)?;
	set_setting(dict, "password", "str", &mut settings.password, required)?;
	set_setting(dict, "itemsperpage", "int", &mut settings.itemsperpage, required)?;
	set_setting(dict, "usefavorites", "bool", &mut settings.usefavorites, required)?;
	set_setting(dict, "useresumepoints", "bool", &mut settings.useresumepoints, required)?;
	set_setting(dict, "showpermalink", "bool", &mut settings.showpermalink, required)?;
	set_setting(dict, "showfanart", "bool", &mut settings.showfanart, required)?;
	set_setting(dict, "showyoutube", "bool", &mut settings.showyoutube, required)?;
	set_setting(dict, "usedrm", "bool", &mut settings.usedrm, required)?;
	set_setting(dict, "useinputstreamadaptive", "bool", &mut settings.useinputstreamadaptive, required)?;
	set_setting(dict, "max_bandwidth", "int", &mut settings.max_bandwidth, required)?;
	set_setting(dict, "kodi_version_major", "int", &mut settings.kodi_version_major, required)?;
	set_setting(dict, "has_inputstream_adaptive", "bool", &mut settings.has_inputstream_adaptive, required)?;
	set_setting(dict, "can_play_drm", "bool", &mut settings.can_play_drm, required)?;
	set_setting(dict, "supports_drm", "bool", &mut settings.supports_drm, required)?;
	set_setting(dict, "has_credentials", "bool", &mut settings.has_credentials, required)?;
	set_setting(dict, "has_studios_white", "bool", &mut settings.has_studios_white, required)?;
	set_setting(dict, "has_youtube", "bool", &mut settings.has_youtube, required)?;
	set_setting(dict, "has_iptv_manager", "bool", &mut settings.has_iptv_manager, required)?;
	reject_unknown_keys(dict)?;
	Ok(())
}

fn set_setting<'py, T: for<'a> FromPyObject<'a, 'py>>(
	dict: &Bound<'py, PyMapping>,
	field_name: &str,
	type_label: &str,
	field: &mut T,
	required: bool,
) -> PyResult<()> {
	match dict.get_item(field_name) {
		Ok(field_value) => field_value.extract::<T>().map_or_else(
			|_err| {
				Err(VrtError::Parse(format!(
					"Could not parse value for setting '{}' to {}.",
					field_name, type_label
				)))
			},
			|value| {
				*field = value;
				Ok(())
			},
		),
		Err(_) => {
			if required {
				Err(VrtError::Parse(format!("No entry for required setting '{}'.", field_name)))
			} else {
				Ok(())
			}
		},
	}?;
	Ok(())
}

fn reject_unknown_keys(dict: &Bound<'_, PyMapping>) -> PyResult<()> {
	if let Ok(keys) = dict.keys() {
		for key in keys {
			match key.extract() {
				Ok(key) => {
					if !SETTINGS_KEYS.contains(&key) {
						return Err(VrtError::Parse(format!("Unknown settings key '{:?}'.", key)).into());
					}
				},
				Err(_) => {
					return Err(VrtError::Parse(format!("Unable to parse settings key '{:?}'.", key)).into());
				},
			}
		}
	};
	Ok(())
}

#[cfg(test)]
mod tests {
	use pyo3::{IntoPyObjectExt, Python};
	use serial_test::serial;

	use super::*;

	#[test]
	fn debug_redacts_username_and_password() {
		let fake_username = "santa@anna.be";
		let fake_password = "gainedTheDay";
		let settings = Settings {
			username: String::from(fake_username),
			password: String::from(fake_password),
			..Default::default()
		};
		let debug_string = format!("{:?}", settings);
		assert!(!debug_string.matches(fake_username).any(|_| true));
		assert!(!debug_string.matches(fake_password).any(|_| true));
	}

	#[test]
	fn settings_from_dict_round_trip() {
		let orig_settings = _example_settings();

		Python::initialize();
		Python::attach(|py| {
			let python_dict = PyDict::from_sequence(
				&[
					("username", orig_settings.username.clone().into_py_any(py).unwrap()),
					("password", orig_settings.password.clone().into_py_any(py).unwrap()),
					("itemsperpage", orig_settings.itemsperpage.into_py_any(py).unwrap()),
					("usefavorites", orig_settings.usefavorites.into_py_any(py).unwrap()),
					("useresumepoints", orig_settings.useresumepoints.into_py_any(py).unwrap()),
					("showpermalink", orig_settings.showpermalink.into_py_any(py).unwrap()),
					("showfanart", orig_settings.showfanart.into_py_any(py).unwrap()),
					("showyoutube", orig_settings.showyoutube.into_py_any(py).unwrap()),
					("usedrm", orig_settings.usedrm.into_py_any(py).unwrap()),
					("useinputstreamadaptive", orig_settings.useinputstreamadaptive.into_py_any(py).unwrap()),
					("max_bandwidth", orig_settings.max_bandwidth.into_py_any(py).unwrap()),
					("kodi_version_major", orig_settings.kodi_version_major.into_py_any(py).unwrap()),
					(
						"has_inputstream_adaptive",
						orig_settings.has_inputstream_adaptive.into_py_any(py).unwrap(),
					),
					("can_play_drm", orig_settings.can_play_drm.into_py_any(py).unwrap()),
					("supports_drm", orig_settings.supports_drm.into_py_any(py).unwrap()),
					("has_credentials", orig_settings.has_credentials.into_py_any(py).unwrap()),
					("has_studios_white", orig_settings.has_studios_white.into_py_any(py).unwrap()),
					("has_youtube", orig_settings.has_youtube.into_py_any(py).unwrap()),
					("has_iptv_manager", orig_settings.has_iptv_manager.into_py_any(py).unwrap()),
				]
				.into_bound_py_any(py)
				.unwrap(),
			)
			.unwrap();

			let settings_result = settings_from_dict(&python_dict);
			assert!(settings_result.is_ok());
			let settings = settings_result.unwrap();
			assert_eq!(settings.username, orig_settings.username);
			assert_eq!(settings.password, orig_settings.password);
			assert_eq!(settings.itemsperpage, orig_settings.itemsperpage);
			assert_eq!(settings.usefavorites, orig_settings.usefavorites);
			assert_eq!(settings.useresumepoints, orig_settings.useresumepoints);
			assert_eq!(settings.showpermalink, orig_settings.showpermalink);
			assert_eq!(settings.showfanart, orig_settings.showfanart);
			assert_eq!(settings.showyoutube, orig_settings.showyoutube);
			assert_eq!(settings.usedrm, orig_settings.usedrm);
			assert_eq!(settings.useinputstreamadaptive, orig_settings.useinputstreamadaptive);
			assert_eq!(settings.max_bandwidth, orig_settings.max_bandwidth);
			assert_eq!(settings.kodi_version_major, orig_settings.kodi_version_major);
			assert_eq!(settings.has_inputstream_adaptive, orig_settings.has_inputstream_adaptive);
			assert_eq!(settings.can_play_drm, orig_settings.can_play_drm);
			assert_eq!(settings.supports_drm, orig_settings.supports_drm);
			assert_eq!(settings.has_credentials, orig_settings.has_credentials);
			assert_eq!(settings.has_studios_white, orig_settings.has_studios_white);
			assert_eq!(settings.has_youtube, orig_settings.has_youtube);
			assert_eq!(settings.has_iptv_manager, orig_settings.has_iptv_manager);
		});
	}

	#[test]
	fn settings_from_dict_rejects_missing_required() {
		Python::initialize();
		Python::attach(|py| {
			let python_dict = PyDict::from_sequence(
				&[("username", "nelson@admiral.com".into_py_any(py).unwrap())]
					.into_bound_py_any(py)
					.unwrap(),
			)
			.unwrap();

			let settings_result = settings_from_dict(&python_dict);
			assert!(settings_result.is_err());
			let error = settings_result.unwrap_err();
			assert!(
				error
					.to_string()
					.matches("No entry for required setting")
					.any(|_| true)
			);
		});
	}

	#[test]
	fn settings_from_dict_rejects_unknown_key() {
		let orig_settings = _example_settings();

		Python::initialize();
		Python::attach(|py| {
			let python_dict = PyDict::from_sequence(
				&[
					("username", orig_settings.username.clone().into_py_any(py).unwrap()),
					("password", orig_settings.password.clone().into_py_any(py).unwrap()),
					("itemsperpage", orig_settings.itemsperpage.into_py_any(py).unwrap()),
					("usefavorites", orig_settings.usefavorites.into_py_any(py).unwrap()),
					("useresumepoints", orig_settings.useresumepoints.into_py_any(py).unwrap()),
					("showpermalink", orig_settings.showpermalink.into_py_any(py).unwrap()),
					("showfanart", orig_settings.showfanart.into_py_any(py).unwrap()),
					("showyoutube", orig_settings.showyoutube.into_py_any(py).unwrap()),
					("usedrm", orig_settings.usedrm.into_py_any(py).unwrap()),
					("useinputstreamadaptive", orig_settings.useinputstreamadaptive.into_py_any(py).unwrap()),
					("max_bandwidth", orig_settings.max_bandwidth.into_py_any(py).unwrap()),
					("kodi_version_major", orig_settings.kodi_version_major.into_py_any(py).unwrap()),
					(
						"has_inputstream_adaptive",
						orig_settings.has_inputstream_adaptive.into_py_any(py).unwrap(),
					),
					("can_play_drm", orig_settings.can_play_drm.into_py_any(py).unwrap()),
					("supports_drm", orig_settings.supports_drm.into_py_any(py).unwrap()),
					("has_credentials", orig_settings.has_credentials.into_py_any(py).unwrap()),
					("has_studios_white", orig_settings.has_studios_white.into_py_any(py).unwrap()),
					("has_youtube", orig_settings.has_youtube.into_py_any(py).unwrap()),
					("has_iptv_manager", orig_settings.has_iptv_manager.into_py_any(py).unwrap()),
					("francois", "l'ollonois".into_py_any(py).unwrap()),
				]
				.into_bound_py_any(py)
				.unwrap(),
			)
			.unwrap();

			let settings_result = settings_from_dict(&python_dict);
			assert!(settings_result.is_err());
			let error = settings_result.unwrap_err();
			assert!(error.to_string().matches("Unknown settings key").any(|_| true));
		});
	}

	#[test]
	fn settings_from_dict_rejects_wrong_type() {
		Python::initialize();
		Python::attach(|py| {
			let python_dict = PyDict::from_sequence(
				&[
					("username", "edward@teach.co.uk".into_py_any(py).unwrap()),
					("password", ["some_dark_secret"].into_py_any(py).unwrap()),
				]
				.into_bound_py_any(py)
				.unwrap(),
			)
			.unwrap();

			let settings_result = settings_from_dict(&python_dict);
			assert!(settings_result.is_err());
			let error = settings_result.unwrap_err().to_string();
			assert!(error.matches("Could not parse value for setting").any(|_| true));
			assert!(!error.matches("some_dark_secret").any(|_| true))
		});
	}

	#[test]
	#[serial]
	fn update_failure_leaves_state_unchanged() {
		_overwrite_static_settings(Settings::default());
		Python::initialize();
		Python::attach(|py| {
			let python_dict = PyDict::from_sequence(
				&[("usefavorites", true.into_py_any(py).unwrap()), ("typoooo", true.into_py_any(py).unwrap())]
					.into_bound_py_any(py)
					.unwrap(),
			)
			.unwrap();

			let update_result = update(python_dict.as_mapping());
			assert!(update_result.is_err());
			let error = update_result.unwrap_err().to_string();
			assert!(error.matches("Unknown settings key").any(|_| true));
		});
		assert!(!SETTINGS.get().unwrap().read().unwrap().usefavorites);
	}

	#[test]
	#[serial]
	fn update_after_init_visible_to_next_snapshot() {
		_overwrite_static_settings(Settings::default());
		let snapshot_1 = snapshot().unwrap();
		assert!(!snapshot_1.usefavorites);
		Python::initialize();
		Python::attach(|py| {
			let python_dict = PyDict::from_sequence(
				&[("usefavorites", true.into_py_any(py).unwrap())]
					.into_bound_py_any(py)
					.unwrap(),
			)
			.unwrap();

			let update_result = update(python_dict.as_mapping());
			assert!(update_result.is_ok());
		});
		assert!(SETTINGS.get().unwrap().read().unwrap().usefavorites);
		let snapshot_2 = snapshot().unwrap();
		assert!(!snapshot_1.usefavorites);
		assert!(snapshot_2.usefavorites);
		// reset state
		SETTINGS.get().unwrap().write().unwrap().usefavorites = false;
	}

	#[test]
	fn validate_path_accepts_absolute() {
		let test_path = "/var/lib/kodi/cache";
		let validated_path = validate_path("test", test_path);
		assert!(validated_path.is_ok());
		assert_eq!(validated_path.unwrap().to_str().unwrap(), test_path);
	}

	#[test]
	fn validate_path_rejects_relative() {
		assert!(validate_path("test", "some_local_dir/cache").is_err());
		assert!(validate_path("test", "./cache").is_err());
		let validate_result = validate_path("test123", "");
		assert!(validate_result.is_err());
		Python::initialize();
		let validate_result_str = validate_result.unwrap_err().to_string();
		assert!(validate_result_str.matches("test123").any(|_| true));
	}

	#[test]
	fn validate_path_rejects_parent_dir_component() {
		assert!(validate_path("test", "/var/lib/../etc/kodi").is_err());
		assert!(validate_path("test", "../var/lib/cache").is_err());
	}

	fn _example_settings() -> Settings {
		Settings {
			username: String::from("test_1"),
			password: String::from("test_2"),
			itemsperpage: 3,
			usefavorites: true,
			useresumepoints: true,
			showpermalink: true,
			showfanart: true,
			showyoutube: true,
			usedrm: true,
			useinputstreamadaptive: true,
			max_bandwidth: 4,
			kodi_version_major: 5,
			has_inputstream_adaptive: true,
			can_play_drm: true,
			supports_drm: true,
			has_credentials: true,
			has_studios_white: true,
			has_youtube: true,
			has_iptv_manager: true,
		}
	}

	fn _overwrite_static_settings(settings: Settings) {
		install(
			settings,
			Paths {
				cache_dir: PathBuf::from("test_cache_dir"),
				profile_dir: PathBuf::from("test_profile_dir"),
			},
		)
		.unwrap();
	}
}
