use std::{collections::HashMap, sync::Mutex};
use once_cell::sync::Lazy;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, MouseEvent};

use super::{course::Course, course_node::CourseNode, tree_element::TreeElement};

// TODO: find a better solution for storing the dragable element's data
static TREE_ELEMENT: Lazy<Mutex<TreeElement>> = Lazy::new(|| Mutex::new(TreeElement::new()));

pub struct CourseTree {
	nodes: HashMap<String, CourseNode>, // TODO: instead of CourseNode, make it an enum of valid node types
}

impl CourseTree {

	pub fn new() -> Self {
		Self::setup_tree_drag();
		Self {
			nodes: HashMap::new()
		}
	}

	// place event listeners to let the tree element get dragged
	fn setup_tree_drag() {
		let document = web_sys::window().unwrap().document().unwrap();
	
		// mouse down
		let mousedown: Closure<dyn FnMut(MouseEvent)> = Closure::new(Box::new(|event: MouseEvent| {
			let mut draggable = TREE_ELEMENT.lock().unwrap();
			(*draggable).on_mouse_down(event);
		}));
		
		// TODO: can search directly for user-tree
		let elem = document.get_element_by_id("user-tree-wrapper").unwrap().query_selector("#user-tree").expect("user-tree should exist").unwrap();
		elem.add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref()).expect("Error adding event listener");
		mousedown.forget();
	
		// mouse up
		let mouseup: Closure<dyn FnMut(MouseEvent)> = Closure::new(Box::new(|event: MouseEvent| {
			let mut draggable = TREE_ELEMENT.lock().unwrap();
			(*draggable).on_mouse_up(event);
		}));
		document.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).expect("Error adding event listener");
		mouseup.forget();
	
		// mouse move
		let mousemove: Closure<dyn FnMut(MouseEvent)> = Closure::new(Box::new(|event: MouseEvent| {
			let mut draggable = TREE_ELEMENT.lock().unwrap();
			(*draggable).on_mouse_move(event);
		}));
		document.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).expect("Error adding event listener");
		mousemove.forget();
	}

	// TODO: check to make sure I never add a duplicate node / course inside a node (then ID's wouldn't be unique)
	fn add_node(&mut self, node: CourseNode) {

		// add the node to the tree element
		let document = window().unwrap().document().unwrap();
		let user_tree = document.get_element_by_id("user-tree").unwrap();

		let new_node_html = document.create_element("div").unwrap();
		new_node_html.set_id(node.get_id());
		new_node_html.set_class_name("class-node");
		new_node_html.set_inner_html(&format!("
			<span>Some element: {}</span>
		", node.get_id()));
		user_tree.append_child(&new_node_html).expect("failed to add node");

		// add node to the list
		self.nodes.insert(node.get_id().clone(), node);
	}

	// TODO: might not even need this method if I just use add_node()
	// TODO: make sure it's not adding a course that's already in the tree
	// add a course to the tree
	pub fn add_course(&mut self, course: Course) {
		// TODO: if the course is already in the tree, split a node instead of making one
		let new_course_node = CourseNode::new(vec![course]);
		self.add_node(new_course_node);
	}
}