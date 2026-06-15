use std::sync::LazyLock;

use pyo3::{Bound, PyAny, PyResult, Python, pyfunction};

use crate::pyconv;

static CATEGORIES: LazyLock<Vec<Category>> = LazyLock::new(|| {
	vec![
		Category { id: "met-audiodescriptie", name: "Audiodescriptie", msgctxt: 30070 },
		Category { id: "cultuur", name: "Cultuur", msgctxt: 30071 },
		Category { id: "docu", name: "Docu", msgctxt: 30072 },
		Category { id: "entertainment", name: "Entertainment", msgctxt: 30073 },
		Category { id: "films", name: "Films", msgctxt: 30074 },
		Category { id: "human-interest", name: "Human interest", msgctxt: 30075 },
		Category { id: "humor", name: "Humor", msgctxt: 30076 },
		Category { id: "voor-kinderen", name: "Kinderen en jongeren", msgctxt: 30077 },
		Category { id: "koken", name: "Koken", msgctxt: 30078 },
		Category { id: "levensbeschouwing", name: "Levensbeschouwing", msgctxt: 30087 },
		Category { id: "lifestyle", name: "Lifestyle", msgctxt: 30079 },
		Category { id: "muziek", name: "Muziek", msgctxt: 30080 },
		Category { id: "nieuws-en-actua", name: "Nieuws en actua", msgctxt: 30081 },
		Category { id: "nostalgie", name: "Nostalgie", msgctxt: 30088 },
		Category { id: "series", name: "Series", msgctxt: 30082 },
		Category { id: "sport", name: "Sport", msgctxt: 30083 },
		Category { id: "talkshows", name: "Talkshows", msgctxt: 30084 },
		Category { id: "met-gebarentaal", name: "Vlaamse Gebarentaal", msgctxt: 30085 },
		Category { id: "wetenschap-en-natuur", name: "Wetenschap en natuur", msgctxt: 30086 },
	]
});

static CHANNELS: LazyLock<Vec<Channel>> = LazyLock::new(|| {
	vec![
		Channel {
			id: "O8",
			name: "een",
			label: "VRT 1",
			studio: "Een",
			live_stream: Some("https://www.vrt.be/vrtnu/livestream/video/vrt1/"),
			live_stream_id: Some("vualto_een_geo"),
			youtube: vec![
				YoutubeChannel { label: "VRT 1", url: "https://www.youtube.com/user/welkombijeen" },
				YoutubeChannel {
					label: "Muziek bij VRT 1",
					url: "https://www.youtube.com/channel/UC7mPNmdg7ADGt0gH8xOrXpQ",
				},
			],
			has_tvguide: true,
			logo: Some("https://images.vrt.be/orig/2023/04/28/c448d669-e5c1-11ed-91d7-02b7b76bf47f.png"),
			epg_id: Some("een.be"),
			preset: Some(1),
			vod: true,
		},
		Channel {
			id: "1H",
			name: "canvas",
			label: "VRT Canvas",
			studio: "Canvas",
			live_stream: Some("https://www.vrt.be/vrtmax/livestream/video/vrt-canvas/"),
			live_stream_id: Some("vualto_canvas_geo"),
			youtube: vec![
				YoutubeChannel { label: "VRT Canvas", url: "https://www.youtube.com/user/CanvasTV" },
				YoutubeChannel { label: "Sporza", url: "https://www.youtube.com/user/SporzaOfficial" },
				YoutubeChannel { label: "Terzake", url: "https://www.youtube.com/user/terzaketv" },
			],
			has_tvguide: true,
			logo: Some("https://images.vrt.be/orig/logo/canvas/CANVAS_logo_lichtblauw.jpg"),
			epg_id: Some("canvas.be"),
			preset: Some(2),
			vod: true,
		},
		Channel {
			id: "O9",
			name: "ketnet",
			label: "Ketnet",
			studio: "Ketnet",
			live_stream: Some("https://www.vrt.be/vrtmax/livestream/video/ketnet/"),
			live_stream_id: Some("vualto_ketnet_geo"),
			youtube: vec![
				YoutubeChannel { label: "Ketnet", url: "https://www.youtube.com/user/KetnetVideo" },
				YoutubeChannel {
					label: "Ketnet Musical",
					url: "https://www.youtube.com/channel/UCB90ZMfqVLgGtp3Z99h4GWg",
				},
				YoutubeChannel {
					label: "Karrewiet",
					url: "https://www.youtube.com/channel/UCCUHHJrtsoC1oyihO86mnMg",
				},
			],
			has_tvguide: true,
			logo: Some("https://images.vrt.be/orig/logo/ketnet/ketnet_LOGO_rood_geel.png"),
			epg_id: Some("ketnet.be"),
			preset: Some(12),
			vod: true,
		},
		Channel {
			id: "",
			name: "ketnet-jr",
			label: "Ketnet Junior",
			studio: "Ketnet Junior",
			live_stream: None,
			live_stream_id: Some("ketnet_jr"),
			youtube: vec![YoutubeChannel {
				label: "Ketnet Junior",
				url: "https://www.youtube.com/channel/UCTxm_H52WlKWBEB_h7PjzFA",
			}],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/2019/07/19/c309360a-aa10-11e9-abcc-02b7b76bf47f.png"),
			epg_id: Some("ketnetjr.be"),
			preset: Some(11),
			vod: true,
		},
		Channel {
			id: "",
			name: "podium19",
			label: "Podium 19",
			studio: "Podium 19",
			live_stream: None,
			live_stream_id: None,
			youtube: vec![],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/2020/12/19/53f5fa6f-4223-11eb-aae0-02b7b76bf47f.png"),
			epg_id: None,
			preset: None,
			vod: true,
		},
		Channel {
			id: "",
			name: "vrt-nu",
			label: "VRT NU",
			studio: "VRT NU",
			live_stream: None,
			live_stream_id: None,
			youtube: vec![],
			has_tvguide: false,
			logo: Some(
				"https://www.vrt.be/etc.clientlibs/vrtvideo/clientlibs/clientlib-site/resources/logo-vrt_NU-pos-rgb@4x.png",
			),
			epg_id: None,
			preset: None,
			vod: true,
		},
		Channel {
			id: "OE",
			name: "sporza",
			label: "Sporza",
			studio: "Sporza",
			live_stream: None,
			live_stream_id: Some("vualto_sporza_geo"),
			youtube: vec![YoutubeChannel {
				label: "Sporza",
				url: "https://www.youtube.com/user/SporzaOfficial",
			}],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logo/sporza/sporza_logo_zwart.png"),
			epg_id: Some("sporza.be"),
			preset: Some(801),
			vod: true,
		},
		Channel {
			id: "13",
			name: "vrtnws",
			label: "VRT NWS",
			studio: "VRT NWS",
			live_stream: None,
			live_stream_id: Some("vualto_nieuws"), /* Some("vualto_journaal"), */
			youtube: vec![
				YoutubeChannel {
					label: "VRT NWS",
					url: "https://www.youtube.com/channel/UC59gT3bFTFNSqafRcluDIsQ",
				},
				YoutubeChannel { label: "Terzake", url: "https://www.youtube.com/user/terzaketv" },
			],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logos/vrtnws.png"),
			epg_id: Some("vrtnws.be"),
			preset: Some(802),
			vod: true,
		},
		Channel {
			id: "11",
			name: "radio1",
			label: "Radio 1",
			studio: "Radio 1",
			live_stream: None,
			live_stream_id: Some("vualto_radio1"),
			youtube: vec![
				YoutubeChannel { label: "Radio 1", url: "https://www.youtube.com/user/vrtradio1" },
				YoutubeChannel {
					label: "Universiteit van Vlaanderen",
					url: "https://www.youtube.com/channel/UC7WpOKbKfzOOnD0PyUN_SYg",
				},
			],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logos/radio1.png"),
			epg_id: Some("radio1.be"),
			preset: Some(901),
			vod: true,
		},
		Channel {
			id: "22",
			name: "radio2",
			label: "Radio 2",
			studio: "Radio 2",
			live_stream: None,
			live_stream_id: Some("vualto_radio2"),
			youtube: vec![
				YoutubeChannel { label: "Radio 2", url: "https://www.youtube.com/user/radio2inbeeld" },
				YoutubeChannel {
					label: "Aha!",
					url: "https://www.youtube.com/channel/UCa9lGLvXB-xJg3d0BjK_tIQ",
				},
			],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logos/radio2.png"),
			epg_id: Some("radio2vlb.be"),
			preset: Some(902),
			vod: true,
		},
		Channel {
			id: "31",
			name: "klara",
			label: "Klara",
			studio: "Klara",
			live_stream: None,
			live_stream_id: Some("vualto_klara"),
			youtube: vec![
				YoutubeChannel { label: "Klara", url: "https://www.youtube.com/user/klararadio" },
				YoutubeChannel {
					label: "Iedereen klassiek",
					url: "https://www.youtube.com/channel/UCgyfqQgt5_K8_zrxHgh_J2w",
				},
			],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logos/klara.png"),
			epg_id: Some("klara.be"),
			preset: Some(903),
			vod: true,
		},
		Channel {
			id: "41",
			name: "stubru",
			label: "Studio Brussel",
			studio: "Studio Brussel",
			live_stream: None, /* "https://stubru.be/live", */
			live_stream_id: Some("vualto_stubru"),
			youtube: vec![YoutubeChannel {
				label: "Studio Brussel",
				url: "https://www.youtube.com/user/StuBru",
			}],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/2019/03/12/1e383cf5-44a7-11e9-abcc-02b7b76bf47f.png"),
			epg_id: Some("stubru.be"),
			preset: Some(904),
			vod: true,
		},
		Channel {
			id: "55",
			name: "mnm",
			label: "MNM",
			studio: "MNM",
			live_stream: None, /* "https://mnm.be/kijk/live", */
			live_stream_id: Some("vualto_mnm"),
			youtube: vec![YoutubeChannel { label: "MNM", url: "https://www.youtube.com/user/MNMbe" }],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logo/mnm/logo_witte_achtergrond.png"),
			epg_id: Some("mnm.be"),
			preset: Some(905),
			vod: true,
		},
		Channel {
			id: "",
			name: "vrtnxt",
			label: "VRT NXT",
			studio: "VRT NXT",
			live_stream: None,
			live_stream_id: None,
			youtube: vec![YoutubeChannel {
				label: "VRT NXT",
				url: "https://www.youtube.com/channel/UCO-VoGCVzhYVwvQvWYJq4-Q",
			}],
			has_tvguide: false,
			logo: None,
			epg_id: None,
			preset: None,
			vod: true,
		},
		Channel {
			id: "",
			name: "de-warmste-week",
			label: "De Warmste Week",
			studio: "De Warmste Week",
			live_stream: None,
			live_stream_id: None,
			youtube: vec![YoutubeChannel {
				label: "De Warmste Week",
				url: "https://www.youtube.com/channel/UC_PsMpKLAp4hSGSXyUCPtxw",
			}],
			has_tvguide: false,
			logo: None,
			epg_id: None,
			preset: None,
			vod: true,
		},
		Channel {
			id: "",
			name: "vrt-events1",
			label: "VRT Events 1",
			studio: "VRT",
			live_stream: None,
			live_stream_id: Some("vualto_events1_geo"),
			youtube: vec![YoutubeChannel {
				label: "VRT",
				url: "https://www.youtube.com/channel/UCojJNXcer3yKj9Q-RWOFZuw",
			}],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logo/vrt.png"),
			epg_id: Some("vrtevents1.be"),
			preset: Some(851),
			vod: false,
		},
		Channel {
			id: "",
			name: "vrt-events2",
			label: "VRT Events 2",
			studio: "VRT",
			live_stream: None,
			live_stream_id: Some("vualto_events2_geo"),
			youtube: vec![],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logo/vrt.png"),
			epg_id: Some("vrtevents2.be"),
			preset: Some(852),
			vod: false,
		},
		Channel {
			id: "",
			name: "vrt-events3",
			label: "VRT Events 3",
			studio: "VRT",
			live_stream: None,
			live_stream_id: Some("vualto_events3_geo"),
			youtube: vec![],
			has_tvguide: false,
			logo: Some("https://images.vrt.be/orig/logo/vrt.png"),
			epg_id: Some("vrtevents3.be"),
			preset: Some(853),
			vod: false,
		},
	]
});

static RELATIVE_DATES: LazyLock<Vec<RelativeDate>> = LazyLock::new(|| {
	vec![
		RelativeDate { id: "2-days-ago", offset: -2, msgctxt: 30330, permalink: false },
		RelativeDate { id: "yesterday", offset: -1, msgctxt: 30331, permalink: true },
		RelativeDate { id: "today", offset: 0, msgctxt: 30332, permalink: true },
		RelativeDate { id: "tomorrow", offset: 1, msgctxt: 30333, permalink: true },
		RelativeDate { id: "in-2-days", offset: 2, msgctxt: 30334, permalink: false },
	]
});

#[derive(serde::Serialize)]
struct Category {
	id: &'static str,
	name: &'static str,
	msgctxt: u16,
}

#[derive(serde::Serialize)]
struct YoutubeChannel {
	label: &'static str,
	url: &'static str,
}

#[derive(serde::Serialize)]
struct Channel {
	id: &'static str,
	name: &'static str,
	label: &'static str,
	studio: &'static str,
	#[serde(skip_serializing_if = "Option::is_none")]
	live_stream: Option<&'static str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	live_stream_id: Option<&'static str>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	youtube: Vec<YoutubeChannel>,
	#[serde(skip_serializing_if = "std::ops::Not::not")]
	has_tvguide: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	logo: Option<&'static str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	epg_id: Option<&'static str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	preset: Option<u16>,
	vod: bool,
}

#[derive(serde::Serialize)]
struct RelativeDate {
	id: &'static str,
	offset: i8,
	msgctxt: u16,
	permalink: bool,
}

#[pyfunction]
pub fn categories(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
	pyconv::json_to_py(
		py,
		&serde_json::to_value(&*CATEGORIES)
			.map_err(|err| pyo3::exceptions::PyTypeError::new_err(err.to_string()))?,
	)
}

#[pyfunction]
pub fn channels(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
	pyconv::json_to_py(
		py,
		&serde_json::to_value(&*CHANNELS)
			.map_err(|err| pyo3::exceptions::PyTypeError::new_err(err.to_string()))?,
	)
}

#[pyfunction]
pub fn relative_dates(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
	pyconv::json_to_py(
		py,
		&serde_json::to_value(&*RELATIVE_DATES)
			.map_err(|err| pyo3::exceptions::PyTypeError::new_err(err.to_string()))?,
	)
}
