use std::collections::HashMap;

use super::course::Course;

// TODO: find a better way to access a json file with this data
const COURSE_DATA: &str = r#"
[
	{
		"subject": "CS",
		"number": 123,
		"prerequisites": [],
		"name": "Intro to Computer Science I",
		"description": "Introductory course."
	},
	{
		"subject": "MTH",
		"number": 321,
		"prerequisites": [],
		"name": "Linear Algebra I",
		"description": "Introductory course to linear algebra."
	},
	{
		"subject": "CS",
		"number": 261,
		"prerequisites": ["cs-123", "mth-321"],
		"name": "Data Structures",
		"description": "Learn about the architecture, doing hashmaps and arrays. Taught in C."
	}
]
"#;

pub struct CourseManager {
	courses: HashMap<String, Course>, // TODO: maybe switch String for CourseID(Subject, Number)
}

impl CourseManager {
	pub fn new() -> Self {
		let course_vec: Vec<Course> = serde_json::from_str(COURSE_DATA).expect("Error deserializing data");
		Self {
			courses: course_vec.into_iter().map(|course| (course.get_id(), course)).collect()
		}
	}

	pub fn get_course(&self, course_id: &String) -> Option<&Course> {
		self.courses.get(course_id)
	}
}