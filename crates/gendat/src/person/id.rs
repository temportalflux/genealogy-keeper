
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PersonId(String);

kdlize::impl_kdlvalue_str!(PersonId);

impl std::str::FromStr for PersonId {
	type Err = std::convert::Infallible;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self(s.to_owned()))
	}
}

impl std::fmt::Display for PersonId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
