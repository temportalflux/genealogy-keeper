use crate::{Date, PersonId};

// A relationship between two or more persons
#[derive(Clone, Debug, PartialEq)]
pub enum Link {
	Parent(Parentage)
}

// Parent/Child relationship (ancestor? parentage?)
#[derive(Clone, Debug, PartialEq)]
pub struct Parentage {
	date: Date,
	child: PersonId,
	parents: Vec<PersonId>,
}
