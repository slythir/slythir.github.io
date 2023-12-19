mod draggable;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate once_cell;

use draggable::Draggable;

use std::sync::Mutex;
use wasm_bindgen::{prelude::*, JsCast};
use once_cell::sync::Lazy;
use web_sys::MouseEvent;

// TODO: find a better solution for storing the dragable element's data
static DRAG_POS: Lazy<Mutex<Draggable>> = Lazy::new(|| Mutex::new(Draggable::new()));

#[wasm_bindgen]
pub fn load() {

	// get document
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let body = document.body().unwrap();

	// new div
	let div = document.create_element("div").expect("Error creating div");
	div.set_class_name("test-class");
	div.set_inner_html("Dynamic div!");
	body.append_child(&div).expect("Error appending child");

	// set up the draggable interactions:

	// mouse down
	let mousedown: Closure<dyn FnMut(MouseEvent)> = Closure::new(Box::new(|event: MouseEvent| {
		let mut draggable = DRAG_POS.lock().unwrap();
		(*draggable).on_mouse_down(event);
	}));
	
	let elem = document.get_element_by_id("user_tree_wrapper").unwrap().query_selector("#user_tree").expect("user_tree should exist").unwrap();
	elem.add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref()).expect("Error adding event listener");
	mousedown.forget();

	// mouse up
	let mouseup: Closure<dyn FnMut(MouseEvent)> = Closure::new(Box::new(|event: MouseEvent| {
		let mut draggable = DRAG_POS.lock().unwrap();
		(*draggable).on_mouse_up(event);
	}));
	document.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).expect("Error adding event listener");
	mouseup.forget();

	// mouse move
	let mousemove: Closure<dyn FnMut(MouseEvent)> = Closure::new(Box::new(|event: MouseEvent| {
		let mut draggable = DRAG_POS.lock().unwrap();
		(*draggable).on_mouse_move(event);
	}));
	document.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).expect("Error adding event listener");
	mousemove.forget();
}