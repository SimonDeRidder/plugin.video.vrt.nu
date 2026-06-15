use _vrtmax::data::{categories, channels, relative_dates};
use pyo3::{
	PyTypeCheck, Python,
	types::{PyAnyMethods, PyDict, PyList},
};

#[test]
fn data_categories_exposed_to_python() {
	Python::initialize();
	Python::attach(|py| {
		let categories_ = categories(py).unwrap();
		assert!(PyList::type_check(&categories_));
		assert!(PyDict::type_check(&categories_.get_item(0).unwrap()));
		let cultuur_cat = categories_
			.try_iter()
			.unwrap()
			.filter_map(|dict_result| {
				dict_result.map_or(None, |dict| {
					dict.get_item("id")
						.unwrap()
						.eq("cultuur")
						.unwrap_or(false)
						.then_some(dict)
				})
			})
			.next()
			.unwrap();
		assert_eq!(cultuur_cat.get_item("name").unwrap().extract::<String>().unwrap(), "Cultuur");
	});
}

#[test]
fn data_channels_exposed_to_python() {
	Python::initialize();
	Python::attach(|py| {
		let channels_ = channels(py).unwrap();
		assert!(PyList::type_check(&channels_));
		assert!(PyDict::type_check(&channels_.get_item(0).unwrap()));
		let vrt1_channel = channels_
			.try_iter()
			.unwrap()
			.filter_map(|dict_result| {
				dict_result.map_or(None, |dict| {
					dict.get_item("name")
						.unwrap()
						.eq("een")
						.unwrap_or(false)
						.then_some(dict)
				})
			})
			.next()
			.unwrap();
		assert_eq!(
			vrt1_channel
				.get_item("label")
				.unwrap()
				.extract::<String>()
				.unwrap(),
			"VRT 1"
		);
	});
}

#[test]
fn data_relative_dates_exposed_to_python() {
	Python::initialize();
	Python::attach(|py| {
		let relative_dates_ = relative_dates(py).unwrap();
		assert!(PyList::type_check(&relative_dates_));
		assert!(PyDict::type_check(&relative_dates_.get_item(0).unwrap()));
		let today_date = relative_dates_
			.try_iter()
			.unwrap()
			.filter_map(|dict_result| {
				dict_result.map_or(None, |dict| {
					dict.get_item("id")
						.unwrap()
						.eq("today")
						.unwrap_or(false)
						.then_some(dict)
				})
			})
			.next()
			.unwrap();
		assert_eq!(today_date.get_item("offset").unwrap().extract::<i8>().unwrap(), 0);
	});
}
