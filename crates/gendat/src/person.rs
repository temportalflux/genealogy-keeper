
mod id;
pub use id::*;

mod name;
pub use name::*;

mod pronoun;
pub use pronoun::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
	id: PersonId,
	names: Vec<PersonName>,
	pronouns: Vec<Pronoun>,
	// e.g. male, female, intersex
	sex_at_birth: Option<String>,
	notes: Option<String>,
}

kdlize::impl_kdl_node!(Person, "person");

impl Person {
	pub fn get_id(&self) -> &PersonId {
		&self.id
	}
}

impl<'doc> kdlize::FromKdlNode<'doc, ()> for Person {
	type Error = crate::Error;
	fn from_kdl(node: &mut kdlize::reader::Node<'doc, ()>) -> Result<Self, Self::Error> {
		todo!();
		//Ok(Self { id, names, pronouns, sex_at_birth, notes })
	}
}
