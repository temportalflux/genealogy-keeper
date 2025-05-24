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
	type Error = miette::Error;
	fn from_kdl(node: &mut kdlize::reader::Node<'doc, ()>) -> Result<Self, Self::Error> {
		use kdlize::reader::*;
		let id = node.next()?.to()?;
		let names = node.children("name").to().collect()?;
		let pronouns = node.children("pronoun").to().collect()?;
		let sex_at_birth = node.child("sex").ok().next()?.to()?;
		let notes = node.child("notes").ok().next()?.to()?;
		Ok(Self {
			id,
			names,
			pronouns,
			sex_at_birth,
			notes,
		})
	}
}
