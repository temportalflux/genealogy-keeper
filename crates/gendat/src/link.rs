use crate::{Date, PersonId};

// A relationship between two or more persons
#[derive(Clone, Debug, PartialEq)]
pub enum Link {
	Parent(Parentage)
}

kdlize::impl_kdl_node!(Link, "link");

impl<'doc> kdlize::FromKdlNode<'doc, ()> for Link {
	type Error = miette::Error;
	fn from_kdl(_node: &mut kdlize::reader::Node<'doc, ()>) -> Result<Self, Self::Error> {
		todo!();
	}
}

// Parent/Child relationship (ancestor? parentage?)
#[derive(Clone, Debug, PartialEq)]
pub struct Parentage {
	date: Date,
	child: PersonId,
	parents: Vec<PersonId>,
}
