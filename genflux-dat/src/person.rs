
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
