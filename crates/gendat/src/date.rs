use miette::Context;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Date {
	Year(/*gregorian*/ i32),
	YearMonth(/*gregorian*/ i32, time::Month),
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

		let re_year = Regex::new(r"^[0-9]{4}$").expect("invalid year regex");
		if re_year.is_match(s) {
			let year = s.parse::<i32>().expect("failed to parse date as a year");
			return Ok(Self::Year(year));
		}

		let re_year_month = Regex::new(r"^([0-9]{4})-([0-9]{2})$").expect("invalid year-month regex");
		if let Some(captures) = re_year_month.captures(s) {
			let year_str = captures.get(1).map(|m| m.as_str());
			let month_str = captures.get(2).map(|m| m.as_str());
			let year_month = year_str.zip(month_str);
			let year_month = year_month
				.map(|(year_str, month_str)| {
					let year_res = year_str.parse::<i32>();
					let month_res = month_str.parse::<time::Month>();
					year_res.ok().zip(month_res.ok())
				})
				.flatten();
			if let Some((year, month)) = year_month {
				return Ok(Self::YearMonth(year, month));
			}
		}

		Err(err)
			.into_diagnostic()
			.with_context(|| "failed to parse as a date-time-offset")
	}
}

impl std::fmt::Display for Date {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Year(year) => year.to_string(),
				Self::YearMonth(year, month) => format!("{year:04}-{:02}", *month as u8),
				Self::NoTime(date) => date.format(&time::format_description::well_known::Rfc3339).unwrap(),
				Self::WithTime(offset) => offset.format(&time::format_description::well_known::Rfc3339).unwrap(),
			}
		)
	}
}
