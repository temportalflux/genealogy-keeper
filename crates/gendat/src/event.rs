
mod birth;
pub use birth::*;

pub enum Event {
	Birth(Birth),
	Death(Death),
	Marriage(Marriage),
	Divorce(Divorce),
}

kdlize::impl_kdl_node!(Event, "event");

impl<'doc> kdlize::FromKdlNode<'doc, ()> for Event {
	type Error = crate::Error;
	fn from_kdl(node: &mut kdlize::reader::Node<'doc, ()>) -> Result<Self, Self::Error> {
		todo!();
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Death;

#[derive(Clone, Debug, PartialEq)]
pub struct Marriage;

#[derive(Clone, Debug, PartialEq)]
pub struct Divorce;
