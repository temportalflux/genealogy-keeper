use crate::Date;

#[derive(Clone, Debug, PartialEq)]
pub struct PersonName {
	names: Vec<TypedName>,
	started_at: Option<Date>,
}

#[derive(Clone, Debug, PartialEq)]
enum TypedName {
	Given(String),
	Surname(String),
}

#[derive(thiserror::Error, Debug, PartialEq, miette::Diagnostic)]
#[error("Unsupported name type-id {0}")]
#[diagnostic(code(gendat::unsupported_personname_type))]
pub struct UnsupportedPersonNameType(String);

impl<'doc> kdlize::FromKdlNode<'doc, ()> for PersonName {
	type Error = miette::Error;
	fn from_kdl(node: &mut kdlize::reader::Node<'doc, ()>) -> Result<Self, Self::Error> {
		use kdlize::reader::*;
		let mut names = Vec::new();
		while let Ok(entry) = node.next() {
			let name = entry.to::<String>()?;
			names.push(match entry.ty().map(|id| id.value()) {
				None => TypedName::Given(name),
				Some("surname") => TypedName::Surname(name),
				Some(type_id) => Err(UnsupportedPersonNameType(type_id.into()))?,
			});
		}
		let started_at = node.prop("start").ok().to()?;
		Ok(Self { names, started_at })
	}
}

impl kdlize::AsKdlNode for PersonName {
	fn as_kdl(&self) -> kdlize::builder::Node {
		use kdlize::builder::*;
		let mut node = Node::default();
		for typed_name in &self.names {
			match typed_name {
				TypedName::Given(name) => node += Value(name),
				TypedName::Surname(name) => node += Typed("surname", Value(name)),
			}
		}
		node += OmitIfEmpty(Property("start", Value(&self.started_at)));
		node
	}
}

#[cfg(test)]
mod test {
	use kdlize::FromKdlNode;
	use super::*;

	#[test]
	fn parse_minimal() -> miette::Result<()> {
		let doc_str = "name Given1 Given2 (surname)Surname";
		let data = crate::from_kdl!(PersonName, doc_str, "name", &());
		assert_eq!(data, PersonName {
			names: vec![
				TypedName::Given("Given1".into()),
				TypedName::Given("Given2".into()),
				TypedName::Surname("Surname".into()),
			],
			started_at: None,
		});
		Ok(())
	}

	#[test]
	fn parse_started() -> miette::Result<()> {
		let doc_str = "name Given1 (surname)Surname start=\"1990-06-15T05:00-05\"";
		let data = crate::from_kdl!(PersonName, doc_str, "name", &());
		assert_eq!(data, PersonName {
			names: vec![
				TypedName::Given("Given1".into()),
				TypedName::Surname("Surname".into()),
			],
			started_at: Some(Date::from(time::OffsetDateTime::new_in_offset(
				time::Date::from_calendar_date(1990, time::Month::June, 15).unwrap(),
				time::Time::from_hms(5, 0, 0).unwrap(), time::UtcOffset::from_hms(-5, 0, 0).unwrap()
			))),
		});
		Ok(())
	}
}
