use pyo3::exceptions::PyException;
use pyo3::types::{PyModule, PyModuleMethods as _};
use pyo3::{Bound, PyErr, PyResult, Python, create_exception};
use thiserror::Error;

// One Python exception class per VrtError variant.
create_exception!(_vrtmax, AuthError, PyException);
create_exception!(_vrtmax, LoginInvalidError, PyException);
create_exception!(_vrtmax, LoginEmptyError, PyException);
create_exception!(_vrtmax, RefreshTokenError, PyException);
create_exception!(_vrtmax, NetworkError, PyException);
create_exception!(_vrtmax, GraphQLError, PyException);
create_exception!(_vrtmax, ParseError, PyException);
create_exception!(_vrtmax, RateLimitError, PyException);
create_exception!(_vrtmax, NotFoundError, PyException);

#[derive(Debug, Error)]
pub enum VrtError {
	#[error("auth error: {0}")]
	Auth(String),
	#[error("login credentials invalid: {0}")]
	LoginInvalid(String),
	#[error("login credentials missing")]
	LoginEmpty,
	#[error("token refresh failed: {0}")]
	RefreshToken(String),
	#[error("network error: {0}")]
	Network(String),
	#[error("graphql error: {0}")]
	GraphQL(String),
	#[error("parse error: {0}")]
	Parse(String),
	#[error("rate limited: retry after {retry_after_secs}s")]
	RateLimit { retry_after_secs: u64, message: String },
	#[error("not found: {0}")]
	NotFound(String),
}

impl From<VrtError> for PyErr {
	fn from(e: VrtError) -> PyErr {
		match e {
			VrtError::Auth(m) => AuthError::new_err(m),
			VrtError::LoginInvalid(m) => LoginInvalidError::new_err(m),
			VrtError::LoginEmpty => LoginEmptyError::new_err("credentials missing"),
			VrtError::RefreshToken(m) => RefreshTokenError::new_err(m),
			VrtError::Network(m) => NetworkError::new_err(m),
			VrtError::GraphQL(m) => GraphQLError::new_err(m),
			VrtError::Parse(m) => ParseError::new_err(m),
			VrtError::RateLimit { retry_after_secs, message } => {
				// Retry hint as a tuple arg, python can read `err.args[1]` if needed.
				RateLimitError::new_err((message, retry_after_secs))
			},
			VrtError::NotFound(m) => NotFoundError::new_err(m),
		}
	}
}

/// Helper for the uninitialized case. Dev error, hence not a VrtError.
pub fn not_initialized(msg: impl Into<String>) -> PyErr {
	pyo3::exceptions::PyRuntimeError::new_err(msg.into())
}

/// Register every `_vrtmax.*Error` exception class on the module. Called from `lib.rs`.
pub fn register(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add("AuthError", py.get_type::<AuthError>())?;
	m.add("LoginInvalidError", py.get_type::<LoginInvalidError>())?;
	m.add("LoginEmptyError", py.get_type::<LoginEmptyError>())?;
	m.add("RefreshTokenError", py.get_type::<RefreshTokenError>())?;
	m.add("NetworkError", py.get_type::<NetworkError>())?;
	m.add("GraphQLError", py.get_type::<GraphQLError>())?;
	m.add("ParseError", py.get_type::<ParseError>())?;
	m.add("RateLimitError", py.get_type::<RateLimitError>())?;
	m.add("NotFoundError", py.get_type::<NotFoundError>())?;
	Ok(())
}
