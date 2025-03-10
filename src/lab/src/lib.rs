use strum::IntoEnumIterator;
use lazy_static::lazy_static;
use std::collections::HashMap;
use suite::test_suite::TestSuiteObject;
use strum_macros::{EnumString, Display, EnumIter, AsRefStr};

mod circle_area;
mod population;
mod sequence;
mod three_number;

#[derive(Debug, Eq, PartialEq, Hash, EnumString, Display, EnumIter, AsRefStr)]
pub enum TestSuiteType {
	#[strum(serialize = "circle_area")]
	CircleArea,
	#[strum(serialize = "population")]
	Population,
	#[strum(serialize = "sequence")]
	Sequence,
	#[strum(serialize = "three_number")]
	ThreeNumber,
}

impl TestSuiteType {
	pub fn from_endwith(s: &str) -> Option<TestSuiteType> {
		TestSuiteType::iter().find(|variant| s.ends_with(variant.as_ref()))
	}
}


lazy_static! {
	pub static ref TEST_SUITE_MAP: HashMap<TestSuiteType, Box<dyn TestSuiteObject>> =
		HashMap::from([
			(
				TestSuiteType::CircleArea,
				Box::new(circle_area::CIRCLE_AREA_TEST_SUITE.clone()) as Box<dyn TestSuiteObject>,
			),
			(
				TestSuiteType::Population,
				Box::new(population::POPULATION_TEST_SUITE.clone()) as Box<dyn TestSuiteObject>,
			),
			(
				TestSuiteType::Sequence,
				Box::new(sequence::SEQUENCE_TEST_SUITE.clone()) as Box<dyn TestSuiteObject>,
			),
			(
				TestSuiteType::ThreeNumber,
				Box::new(three_number::THREE_NUMBER_TEST_SUITE.clone())
					as Box<dyn TestSuiteObject>,
			),
		])
	;
}
