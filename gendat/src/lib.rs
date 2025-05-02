
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
