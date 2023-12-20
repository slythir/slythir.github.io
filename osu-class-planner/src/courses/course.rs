extern crate serde;

// TODO: remove serialize
use self::serde::{Serialize, Deserialize};

// TODO: Add more subjects
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
enum Subject {
	CS,
	MTH,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
	subject: Subject,
	number: u16,
	prerequisites: Vec<String>,
	pub name: String,
	pub description: String,
}

impl Course {
	pub fn get_id(&self) -> String {
		format!("{}-{}",
			match self.subject {
				Subject::CS => { "cs" },
				Subject::MTH => { "mth" },
			},
			self.number.to_string()
		)
	}
}