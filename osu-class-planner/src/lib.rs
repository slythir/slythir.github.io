extern crate wasm_bindgen;
extern crate web_sys;
extern crate once_cell;

use std::sync::{Mutex, MutexGuard};

use wasm_bindgen::prelude::*;
use once_cell::sync::Lazy;

static BTN_COUNT: Lazy<Mutex<u64>> = Lazy::new(|| { Mutex::new(0) });

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

	// new button
	let btn = document.create_element("button").expect("Error creating button");
	btn.set_inner_html("Click me!");
	btn.set_attribute("onclick", "incriment_counter()").expect("Error setting attribute");
    body.append_child(&btn).expect("Error appending child");
}

#[wasm_bindgen]
pub fn incriment_counter() {

	// incriment counter
	let mut count = get_button_counter();
	*count += 1;

	// update display
	let doc = web_sys::window().unwrap().document().unwrap();
	let span_disp = doc.get_element_by_id("button-count").unwrap();
	span_disp.set_inner_html(&count.to_string());
}

// just here to test calling functions from other functions
fn get_button_counter<'a>() -> MutexGuard<'a, u64> {
	let count = BTN_COUNT.lock().unwrap();
	count
}