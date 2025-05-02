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

impl<'doc> kdlize::FromKdlNode<'doc, ()> for PersonName {
	type Error = ErrorParsePersonName;
	fn from_kdl(node: &mut kdlize::reader::Node<'doc, ()>) -> Result<Self, Self::Error> {
		use kdlize::reader::*;
		let mut names = Vec::new();
		while let Ok(entry) = node.next() {
			let name = entry.to::<String>()?;
			names.push(match entry.ty().map(|id| id.value()) {
				None => TypedName::Given(name),
				Some("surname") => TypedName::Surname(name),
				Some(type_id) => Err(ErrorParsePersonName::UnsupportedNameType(type_id.into()))?,
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

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ErrorParsePersonName {
	#[error(transparent)]
	Kdl(kdlize::error::QueryError),
	#[error(transparent)]
	ParseDateTime(time::error::Parse),
	#[error("Unsupported name type-id {0}")]
	UnsupportedNameType(String),
}
impl From<kdlize::error::MissingEntry> for ErrorParsePersonName {
	fn from(value: kdlize::error::MissingEntry) -> Self {
		Self::Kdl(value.into())
	}
}
impl From<kdlize::error::MissingEntryType> for ErrorParsePersonName {
	fn from(value: kdlize::error::MissingEntryType) -> Self {
		Self::Kdl(value.into())
	}
}
impl From<kdlize::error::ValueTypeMismatch> for ErrorParsePersonName {
	fn from(value: kdlize::error::ValueTypeMismatch) -> Self {
		Self::Kdl(value.into())
	}
}
impl From<kdlize::error::MissingChild> for ErrorParsePersonName {
	fn from(value: kdlize::error::MissingChild) -> Self {
		Self::Kdl(value.into())
	}
}
impl From<kdlize::error::ParseValueFromStr<time::error::Parse>> for ErrorParsePersonName {
	fn from(value: kdlize::error::ParseValueFromStr<time::error::Parse>) -> Self {
		match value {
			kdlize::error::ParseValueFromStr::FailedToParse(mismatch) => Self::from(mismatch),
			kdlize::error::ParseValueFromStr::FailedToInterpret(err) => Self::from(err),
		}
	}
}
impl From<time::error::Parse> for ErrorParsePersonName {
	fn from(value: time::error::Parse) -> Self {
		Self::ParseDateTime(value)
	}
}

#[cfg(test)]
mod test {
	use kdlize::FromKdlNode;
	use super::*;

	#[test]
	fn parse_minimal() -> anyhow::Result<()> {
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
	fn parse_started() -> anyhow::Result<()> {
		let doc_str = "name Given1 (surname)Surname start=\"1990-06-15T05:00-05\"";
		let data = crate::from_kdl!(PersonName, doc_str, "name", &());
		assert_eq!(data, PersonName {
			names: vec![
				TypedName::Given("Given1".into()),
				TypedName::Surname("Surname".into()),
			],
			started_at: Some(Date::from(time::OffsetDateTime::new_in_offset(
				time::Date::from_calendar_date(1990, time::Month::June, 15)?,
				time::Time::from_hms(5, 0, 0)?, time::UtcOffset::from_hms(-5, 0, 0)?
			))),
		});
		Ok(())
	}
}
