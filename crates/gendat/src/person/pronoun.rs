
// see https://www.grammarly.com/blog/parts-of-speech/personal-pronouns/
#[derive(Clone, Debug, PartialEq)]
pub struct Pronoun {
	nominative: String,
	objective: String,
}
