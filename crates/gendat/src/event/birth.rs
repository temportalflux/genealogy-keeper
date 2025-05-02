use crate::{Date, PersonId};

#[derive(Clone, Debug, PartialEq)]
pub struct Birth {
	date: Date,
	// TODO: How to represent multi-birth with different timestamps (e.g. twins/triplets born at different times)
	offspring: Vec<PersonId>,
	donors: Vec<PersonId>,
	carrier: Option<PersonId>,
	location: Option<String>,
	notes: Option<String>,
}
