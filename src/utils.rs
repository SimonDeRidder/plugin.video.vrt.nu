use pyo3::{
	Bound, PyAny, Python, pyfunction,
	types::{PyAnyMethods, PyDict, PyDictMethods},
};

/// Reformat images.vrt.be urls
#[pyfunction]
pub fn reformat_image_url(url: Option<String>) -> String {
	match url {
		None => String::new(),
		Some(url_str) => match url_str.is_empty() {
			true => String::new(),
			false => {
				if url_str.starts_with("//") {
					String::from("https:") + &url_str.replace("images.vrt.be/orig/", "images.vrt.be/w1920hx/")
				} else if url_str.starts_with("/") {
					String::from("https://images.vrt.be") + &(url_str.replace("/orig/", "/w1920hx/"))
				} else {
					url_str.replace("images.vrt.be/orig/", "images.vrt.be/w1920hx/")
				}
			},
		},
	}
}

/// Convert
/// 	- a targetUrl (e.g. //www.vrt.be/vrtnu/a-z/de-campus-cup.relevant/),
/// 	- a short programUrl (e.g. /vrtnu/a-z/de-campus-cup/) or
/// 	- a medium programUrl (e.g. //www.vrt.be/vrtnu/a-z/de-campus-cup/)
/// 	- a long programUrl (e.g. https://www.vrt.be/vrtnu/a-z/de-campus-cup/)
/// to a program url component (e.g. de-campus-cup).
/// Any season or episode information is removed as well.
#[pyfunction]
pub fn url_to_program(url: String) -> String {
	let mut program = String::new();
	if url.starts_with("https://www.vrt.be/vrtnu/a-z/")
		|| url.starts_with("//www.vrt.be/vrtnu/a-z/")
		|| url.starts_with("https://www.vrt.be/vrtmax/a-z/")
		|| url.starts_with("//www.vrt.be/vrtmax/a-z/")
	{
		program = url.split('/').nth(5).unwrap_or("").to_string();
	} else if url.starts_with("/vrtnu/a-z/") || url.starts_with("/vrtmax/a-z/") {
		// Workaround: when adding a favourite on https://www.vrt.be/vrtmax/ sometimes '.html' is wrongly added
		program = url.split('/').nth(3).unwrap_or("").replace(".html", "");
	}
	if program.ends_with(".relevant") {
		program = program.replace(".relevant", "");
	}
	program
}

/// Convert a full VRT MAX url (e.g. https://www.vrt.be/vrtnu/a-z/de-ideale-wereld/2019-nj/de-ideale-wereld-d20191010/)
/// to a VRT Search API url (e.g. //www.vrt.be/vrtnu/a-z/de-ideale-wereld/2019-nj/de-ideale-wereld-d20191010/)
fn video_to_api_url(url: String) -> String {
	// NOTE: ensure a trailing slash because routing plugin removes it and VRT MAX Search API needs it
	url.trim_start_matches("https:").trim_end_matches('/').to_owned() + "/"
}

/// Convert a plugin:// url (e.g. plugin://plugin.video.vrt.nu/play/id/vid-5b12c0f6-b8fe-426f-a600-557f501f3be9/pbs-pub-7e2764cf-a8c0-4e78-9cbc-46d39381c237)
/// to an id dictionary (e.g. {"video_id": 'vid-5b12c0f6-b8fe-426f-a600-557f501f3be9'}
#[pyfunction]
pub fn play_url_to_id(py: Python<'_>, url: String) -> Bound<'_, PyDict> {
	let play_ids = pyo3::types::PyDict::new(py);
	if url.contains("play/id/")
		&& let Some(video_id) = url
			.split("play/id/")
			.nth(1)
			.and_then(|remainder| remainder.split('/').next())
	{
		let _ = play_ids.set_item("video_id", video_id);
	} else if url.contains("play/upnext/") {
		if let Some(video_id) = url.split("play/upnext/").nth(1) {
			let _ = play_ids.set_item("video_id", video_id);
		}
	} else if url.contains("/play/url/") {
		if let Some(video_url) = url.split("play/url/").nth(1) {
			let _ = play_ids.set_item("video_url", video_to_api_url(video_url.to_owned()));
		}
	} else if url.contains("play/whatson/") {
		if let Some(whatson_id) = url.split("play/whatson/").nth(1) {
			let _ = play_ids.set_item("whatson_id", whatson_id);
		}
	} else if url.contains("play/episode/") {
		if let Some(episode_id) = url.split("play/episode/").nth(1) {
			let _ = play_ids.set_item("episode_id", episode_id);
		}
	} else if url.contains("play/airdate/")
		&& let Some(video_id) = url
			.split("play/airdate/")
			.nth(1)
			.and_then(|remainder| remainder.split('/').next())
	{
		let _ = play_ids.set_item("video_id", video_id);
	}
	play_ids
}

/// Create a link that is as short as possible
#[pyfunction]
pub fn shorten_link(url: Option<String>) -> Option<String> {
	url.map(|url_| {
		if url_.starts_with("https://www.vrt.be/vrtmax/") {
			// As used in episode search result "permalink"
			url_.replace("https://www.vrt.be/vrtmax/", "vrtmax.be/")
		} else if url_.starts_with("//www.vrt.be/vrtmax/") {
			// As used in program a-z listing "targetUrl"
			url_.replace("//www.vrt.be/vrtmax/", "vrtmax.be/")
		} else {
			url_
		}
	})
}

/// Find (the first) dictionary in a list where key matches value
#[pyfunction]
pub fn find_entry<'a>(
	dict_list: Bound<'a, PyAny>,
	key: String,
	value: Bound<'_, PyAny>,
) -> Option<Bound<'a, PyAny>> {
	dict_list.try_iter().map_or(None, |iterator| {
		iterator
			.filter_map(|dict_result| {
				dict_result.map_or(None, |dict| {
					dict.get_item(&key)
						.map_or(None, |dict_value| dict_value.eq(&value).unwrap_or(false).then_some(dict))
				})
			})
			.next()
	})
}

/// Convert a YouTube URL to a Kodi plugin URL
#[pyfunction]
pub fn youtube_to_plugin_url(url: String) -> String {
	url.replace("https://www.youtube.com/", "plugin://plugin.video.youtube/")
		.trim_end_matches('/')
		.to_owned()
		+ "/"
}

#[cfg(test)]
mod tests {
	use pyo3::{
		IntoPyObjectExt, Python,
		types::{PyAnyMethods, PyDictMethods},
	};

	use crate::utils::{
		find_entry, play_url_to_id, reformat_image_url, shorten_link, url_to_program, video_to_api_url,
		youtube_to_plugin_url,
	};

	#[test]
	fn test_reformat_image_url() {
		assert_eq!(reformat_image_url(None), "");
		assert_eq!(reformat_image_url(Some(String::from(""))), "");
		assert_eq!(
			reformat_image_url(Some(String::from("http://images.vrt.be/orig/some_image.jpg"))),
			"http://images.vrt.be/w1920hx/some_image.jpg"
		);
		assert_eq!(
			reformat_image_url(Some(String::from("//images.vrt.be/orig/some_image.png"))),
			"https://images.vrt.be/w1920hx/some_image.png"
		);
		assert_eq!(
			reformat_image_url(Some(String::from("/orig/some_image.gif"))),
			"https://images.vrt.be/w1920hx/some_image.gif"
		);
	}

	#[test]
	fn test_url_to_program() {
		let program = "buck";
		let sub_path = format!("a-z/{}/1/{}-s1a32", program, program);

		assert_eq!(program, url_to_program(format!("/vrtnu/{}/", sub_path)));
		assert_eq!(program, url_to_program(format!("/vrtmax/{}/", sub_path)));
		assert_eq!(program, url_to_program(format!("//www.vrt.be/vrtnu/{}/", sub_path)));
		assert_eq!(program, url_to_program(format!("//www.vrt.be/vrtmax/{}/", sub_path)));
		assert_eq!(program, url_to_program(format!("https://www.vrt.be/vrtnu/{}/", sub_path)));
		assert_eq!(program, url_to_program(format!("https://www.vrt.be/vrtmax/{}/", sub_path)));
		assert_eq!(program, url_to_program(format!("/vrtnu/a-z/{}.relevant/", program)));
		assert_eq!(program, url_to_program(format!("/vrtmax/a-z/{}.relevant/", program)));
	}

	#[test]
	fn test_video_to_api_url() {
		let api_url = "//www.vrt.be/vrtnu/a-z/winteruur/1/winteruur-s1a61/";
		let video_url = format!("https:{}", api_url);
		assert_eq!(api_url, video_to_api_url(video_url))
	}

	#[test]
	fn test_play_url_to_id() {
		Python::initialize();
		Python::attach(|py| {
			let video_id = "vid-5b12c0f6-b8fe-426f-a600-557f501f3be9";

			{
				let result = play_url_to_id(
					py,
					format!(
						"plugin://plugin.video.vrt.nu/play/id/{}/pbs-pub-7e2764cf-a8c0-4e78-9cbc-46d39381c237",
						video_id
					),
				);
				let result_video_id = result.get_item("video_id").unwrap();
				assert!(result_video_id.is_some());
				assert_eq!(result_video_id.unwrap().extract::<String>().unwrap(), video_id);
			}
			{
				let result =
					play_url_to_id(py, format!("plugin://plugin.video.vrt.nu/play/upnext/{}", video_id));
				let result_video_id = result.get_item("video_id").unwrap();
				assert!(result_video_id.is_some());
				assert_eq!(result_video_id.unwrap().extract::<String>().unwrap(), video_id);
			}
			{
				let video_url = "//www.vrt.be/vrtmax/kanalen/canvas/";
				let result =
					play_url_to_id(py, format!("plugin://plugin.video.vrt.nu/play/url/https:{}", video_url));
				let result_video_url = result.get_item("video_url").unwrap();
				assert!(result_video_url.is_some());
				assert_eq!(result_video_url.unwrap().extract::<String>().unwrap(), video_url);
			}
		});
	}

	#[test]
	fn test_shorten_link() {
		let link_id = "p.LR90GkqOD";
		let short_link = format!("vrtmax.be/{}", link_id);
		let medium_url = format!("//www.vrt.be/vrtmax/{}", link_id);
		let long_url = format!("https://www.vrt.be/vrtmax/{}", link_id);

		assert_eq!(Some(short_link.clone()), shorten_link(Some(long_url)));
		assert_eq!(Some(short_link), shorten_link(Some(medium_url)));
		assert_eq!(None, shorten_link(None));
	}

	#[test]
	fn test_find_entry() {
		Python::initialize();
		Python::attach(|py| {
			let dict_1 = pyo3::types::PyDict::from_sequence(
				&[("foo", "foo"), ("bar", "bar"), ("baz", "baz")]
					.into_bound_py_any(py)
					.unwrap(),
			)
			.unwrap();
			let dict_2 = pyo3::types::PyDict::from_sequence(
				&[("foo", "bar"), ("bar", "baz"), ("baz", "foo")]
					.into_bound_py_any(py)
					.unwrap(),
			)
			.unwrap();
			let dict_3 = pyo3::types::PyDict::from_sequence(
				&[("foo", "baz"), ("bar", "foo"), ("baz", "bar")]
					.into_bound_py_any(py)
					.unwrap(),
			)
			.unwrap();
			let haystack = pyo3::types::PyList::new(py, [dict_1.clone(), dict_2.clone(), dict_3.clone()])
				.unwrap()
				.into_bound_py_any(py)
				.unwrap();

			println!("dict_3: {}", dict_3);
			println!(
				"find_entry: {:?}",
				find_entry(
					haystack.clone(),
					"foo".to_string(),
					"baz".to_string().into_bound_py_any(py).unwrap()
				)
				.unwrap()
			);
			assert!(
				dict_3
					.eq(find_entry(
						haystack.clone(),
						"foo".to_string(),
						"baz".to_string().into_bound_py_any(py).unwrap()
					)
					.unwrap())
					.unwrap()
			);
			assert!(
				dict_2
					.eq(find_entry(
						haystack.clone(),
						"bar".to_string(),
						"baz".to_string().into_bound_py_any(py).unwrap()
					)
					.unwrap())
					.unwrap()
			);
			assert!(
				find_entry(haystack.clone(), "foo".to_string(), "blah".into_bound_py_any(py).unwrap())
					.is_none()
			);
		})
	}

	#[test]
	fn test_youtube_to_plugin_url() {
		assert_eq!(
			"plugin://plugin.video.youtube/foo/bar/",
			youtube_to_plugin_url("https://www.youtube.com/foo/bar".to_owned())
		);
		assert_eq!(
			"plugin://plugin.video.youtube/foo/bar/baz/",
			youtube_to_plugin_url("https://www.youtube.com/foo/bar/baz/".to_owned())
		);
	}
}
