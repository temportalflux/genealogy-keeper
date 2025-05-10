
// see https://www.grammarly.com/blog/parts-of-speech/personal-pronouns/
#[derive(Clone, Debug, PartialEq)]
pub struct Pronoun {
	nominative: String,
	objective: String,
}

impl<'doc> kdlize::FromKdlNode<'doc, ()> for Pronoun {
	type Error = miette::Error;
	fn from_kdl(node: &mut kdlize::reader::Node<'doc, ()>) -> Result<Self, Self::Error> {
		use kdlize::reader::*;
		let nominative = node.next()?.to()?;
		let objective = node.next()?.to()?;
		Ok(Self { nominative, objective })
	}
}

impl kdlize::AsKdlNode for Pronoun {
	fn as_kdl(&self) -> kdlize::builder::Node {
		use kdlize::builder::*;
		Node::default() + Value(&self.nominative) + Value(&self.objective)
	}
}
