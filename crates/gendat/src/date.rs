use miette::Context;

#[derive(Clone, Debug, PartialEq)]
pub enum Date {
	NoTime(time::Date),
	WithTime(time::OffsetDateTime),
}

kdlize::impl_kdlvalue_str!(Date);

impl From<time::Date> for Date {
	fn from(value: time::Date) -> Self {
		Self::NoTime(value)
	}
}

impl From<time::OffsetDateTime> for Date {
	fn from(value: time::OffsetDateTime) -> Self {
		Self::WithTime(value)
	}
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, miette::Diagnostic)]
#[error("Invalid date-time string: {err:?}")]
#[diagnostic(code(time::error::Parse))]
pub struct InvalidDateString {
	#[label("{err}")]
	span: miette::SourceSpan,
	err: time::error::Parse,
}

impl std::str::FromStr for Date {
	type Err = miette::Report;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use miette::IntoDiagnostic;
		use time::macros::format_description;

		let format = time::format_description::well_known::Rfc3339;

		let err = match time::OffsetDateTime::parse(s, &format) {
			Ok(date) => return Ok(Self::WithTime(date)),
			Err(err) => err,
		};

		let fmt_date_only = format_description!("[year]-[month]-[day]");
		if let Ok(date) = time::Date::parse(s, &fmt_date_only) {
			return Ok(Self::NoTime(date));
		}

		Err(err).into_diagnostic().with_context(|| "failed to parse as a date-time-offset")
	}
}

impl std::fmt::Display for Date {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Self::NoTime(date) => date.format(&time::format_description::well_known::Rfc3339),
			Self::WithTime(offset) => offset.format(&time::format_description::well_known::Rfc3339),
		}.unwrap())
	}
}

