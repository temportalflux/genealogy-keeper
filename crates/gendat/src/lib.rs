
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

pub fn parse_document_contents(contents: &str) -> Result<Vec<Entry>, miette::Error> {
	use kdlize::{NodeId, FromKdlNode};
	let doc = kdl::KdlDocument::parse(contents)?;
	let mut entries = Vec::new();
	entries.reserve(doc.nodes().len());
	for node in doc.nodes() {
		let node_name = node.name().value();
		let mut reader = kdlize::reader::Node::new(node, &());
		if node_name == Person::id() {
			let person_res = Person::from_kdl(&mut reader);
			let person = person_res.map_err(|err| err.with_source_code(contents.to_owned()))?;
			entries.push(Entry::Person(person));
		}
		else if node_name == Event::id() {
			//entries.push(Entry::Event(Event::from_kdl(&mut reader)?));
		}
		else if node_name == Link::id() {
			//entries.push(Entry::Link(Link::from_kdl(&mut reader)?));
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
				let node = doc.get($node_name).ok_or_else(|| {
					let src = $doc_str.to_string();
					let label = format!("missing child named {:?}", $node_name);
					let span = miette::LabeledSpan::new_primary_with_span(Some(label), (0, src.len()));
					kdlize::error::NodeMissingChild {
						src,
						span,
						child_name: kdl::KdlIdentifier::parse($node_name).unwrap(), 
					}
				})?;
				let mut reader = kdlize::reader::Node::new(node, $context);
				<$value_ty>::from_kdl(&mut reader)?
			}
		};
	}

}
