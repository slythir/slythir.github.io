mod courses;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate once_cell;

use std::sync::Mutex;
use courses::{course_manager::CourseManager, course_tree::CourseTree};
use wasm_bindgen::{prelude::*, JsCast};
use once_cell::sync::Lazy;
use web_sys::{MouseEvent, window};

// list of every course
static COURSES: Lazy<CourseManager> = Lazy::new(|| CourseManager::new());

// course tree manager
static COURSE_TREE: Lazy<Mutex<CourseTree>> = Lazy::new(|| Mutex::new(CourseTree::new()));

#[wasm_bindgen]
pub fn load() {

	// TODO: Remove
	// test adding nodes to the tree
	let mth_321 = COURSES.get_course(&"mth-321".to_string()).unwrap();
	let cs_123 = COURSES.get_course(&"cs-123".to_string()).unwrap();

	let mut ct = COURSE_TREE.lock().unwrap();
	ct.add_course(mth_321.clone());
	ct.add_course(cs_123.clone());

	// TODO: clean this up (put inside its own node maker thing, do for every course)
	start_course_adder();
}

// TODO: clean this up (put inside its own node maker thing, do for every course)
fn start_course_adder() {

	// Add node when the user clicks "+" on "CS 261"
	let document = window().unwrap().document().unwrap();
	let addrem_cs_261 = document.get_element_by_id("addrem-course-cs-261").unwrap();

	let add_cs_261: Closure<dyn FnMut(MouseEvent)> = Closure::new(Box::new(|_event| {
		let mut ct = COURSE_TREE.lock().unwrap();
		let cs_261 = COURSES.get_course(&"cs-261".to_string()).unwrap();
		(*ct).add_course(cs_261.clone());
	}));
	addrem_cs_261.add_event_listener_with_callback("click", add_cs_261.as_ref().unchecked_ref()).expect("failed to add listener");
	add_cs_261.forget();
}