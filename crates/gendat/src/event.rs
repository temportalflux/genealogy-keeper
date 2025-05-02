
mod birth;
pub use birth::*;

pub enum Event {
	Birth(Birth),
	Death(Death),
	Marriage(Marriage),
	Divorce(Divorce),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Death;

#[derive(Clone, Debug, PartialEq)]
pub struct Marriage;

#[derive(Clone, Debug, PartialEq)]
pub struct Divorce;
