
#[derive(Clone, Debug, PartialEq)]
pub struct Date(time::OffsetDateTime);

kdlize::impl_kdlvalue_str!(Date);

impl From<time::OffsetDateTime> for Date {
	fn from(value: time::OffsetDateTime) -> Self {
		Self(value)
	}
}

impl std::str::FromStr for Date {
	type Err = time::error::Parse;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self(time::OffsetDateTime::parse(s, &time::format_description::well_known::Iso8601::DEFAULT)?))
	}
}

impl std::fmt::Display for Date {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.format(&time::format_description::well_known::Iso8601::DEFAULT).unwrap())
	}
}

