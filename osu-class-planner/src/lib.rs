// to compile: wasm-pack build --target web
// THEN: remove .gitignore inside of pkg

extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
	pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn load() {

	// get document
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();

	// new div
	let div = document.create_element("div").expect("Error creating div");
	div.set_class_name("test-class");
	div.set_inner_html("Dynamic div!");

	// add div to document
	let body = document.body().unwrap();
	body.append_child(&div).expect("Error appending child");
}