
mod date;
pub use date::*;

mod document;
pub use document::*;

mod event;
pub use event::*;

mod link;
pub use link::*;

mod person;
pub use person::*;

pub enum Entry {
	Person(Person),
	Event(Event),
	Link(Link),
}

#[derive(thiserror::Error, Debug)]
#[derive(miette::Diagnostic)]
pub enum Error {
	#[error(transparent)]
	#[diagnostic(transparent)]
	FailedToParseDocument(kdl::KdlError),
}

pub fn parse_document_contents(contents: &str) -> Result<Vec<Entry>, Error> {
	use kdlize::{NodeId, FromKdlNode};
	let doc: Result<kdl::KdlDocument, kdl::KdlError> = kdl::KdlDocument::parse(contents);
	let doc = doc.map_err(|err| Error::FailedToParseDocument(err))?;
	let mut entries = Vec::new();
	entries.reserve(doc.nodes().len());
	for node in doc.nodes() {
		let node_name = node.name().value();
		let mut reader = kdlize::reader::Node::new(node, &());
		if node_name == Person::id() {
			entries.push(Entry::Person(Person::from_kdl(&mut reader)?));
		}
		else if node_name == Event::id() {
			entries.push(Entry::Event(Event::from_kdl(&mut reader)?));
		}
		else if node_name == Link::id() {
			entries.push(Entry::Link(Link::from_kdl(&mut reader)?));
		}
	}
	Ok(entries)
}

#[cfg(test)]
pub(crate) mod test {

	#[macro_export]
	macro_rules! from_kdl {
		($value_ty:ty, $doc_str:expr, $node_name:expr, $context:expr) => {
			{
				let doc = kdl::KdlDocument::parse($doc_str)?;
				let node = doc.get($node_name).ok_or_else(|| kdlize::error::MissingChild(doc.clone(), $node_name.into()))?;
				let mut reader = kdlize::reader::Node::new(node, $context);
				<$value_ty>::from_kdl(&mut reader)?
			}
		};
	}

}
