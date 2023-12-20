use super::course::Course;

pub struct CourseNode {
	_courses: Vec<Course>, // TODO: use this to check node prerequisites
	id: String,
}

impl CourseNode {

	// TODO: courses must have a length of at least 1
	pub fn new(courses: Vec<Course>) -> Self {

		// TODO: Make sure this method is valid
		// ^ can't add multiple CourseNodes that share the same first course to the tree, or it will result in duplicate IDs
		let id = format!("node-{}", courses[0].get_id());

		Self {
			_courses: courses,
			id,
		}
	}

	pub fn get_id(&self) -> &String {
		&self.id
	}
}