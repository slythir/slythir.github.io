use web_sys::{MouseEvent, window};

pub struct Draggable {
	elem_x: i32,
	elem_y: i32,
	grab_x: i32,
	grab_y: i32,
	dragging: bool,
}

impl Draggable {

	pub fn new() -> Self {
		Self {
			elem_x: 0,
			elem_y: 0,
			grab_x: 0,
			grab_y: 0,
			dragging: false
		}
	}

	pub fn on_mouse_down(&mut self, event: MouseEvent) {
		self.grab_x = event.client_x();
		self.grab_y = event.client_y();
		self.dragging = true;
	}

	pub fn on_mouse_up(&mut self, event: MouseEvent) {
		if self.dragging {
			(self.elem_x, self.elem_y) = self.get_elem_pos(event);
			self.dragging = false;
		}
	}

	// TODO: store elem somewhere to get faster
	pub fn on_mouse_move(&mut self, event: MouseEvent) {
		if self.dragging {

			let elem_pos = self.get_elem_pos(event);
			let transform = format!("transform: translate({}px, {}px)", elem_pos.0, elem_pos.1);

			let elem =
				window().unwrap()
				.document().unwrap()
				.get_element_by_id("user_tree_wrapper").expect("user_tree_wrapper should exist")
				.query_selector("#user_tree").expect("query_selector failed").expect("user_tree should exist");
			elem.set_attribute("style", &transform).unwrap();
		}
	}

	// TODO: store wrap/drag somewhere to get faster
	fn get_elem_pos(&self, event: MouseEvent) -> (i32, i32) {

		let wrap = window().unwrap().document().unwrap().get_element_by_id("user_tree_wrapper").unwrap();
		let drag = wrap.query_selector("#user_tree").expect("user_tree should exist").unwrap();

		// calculate element position based on cursor position and where the element was grabbed
		let mut elem_pos = (self.elem_x + event.client_x() - self.grab_x, self.elem_y + event.client_y() - self.grab_y);

		// element bounds
		let left_bound = wrap.client_width() - drag.client_width();
		let top_bound = wrap.client_height() - drag.client_height();
		if elem_pos.0 > 0 { elem_pos.0 = 0 }
		if elem_pos.1 > 0 { elem_pos.1 = 0 }
		if elem_pos.0 < left_bound { elem_pos.0 = left_bound }
		if elem_pos.1 < top_bound { elem_pos.1 = top_bound }

		elem_pos
	}
}